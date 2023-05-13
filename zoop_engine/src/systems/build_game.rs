use bevy::prelude::*;
use bevy_ggrs::*;
#[cfg(feature = "world_debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(feature = "debug_lines")]
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

use crate::domain::colors::*;
use crate::domain::desync::*;
use crate::domain::frames::*;
use crate::domain::game_config::GameConfig;
use crate::domain::game_set::GameSet;
use crate::domain::spawn::*;
use crate::systems::build_network::*;
use crate::systems::drive_car::*;
use crate::systems::manage_scene::*;
use crate::systems::rollback_rapier_context::*;
use crate::systems::save_rapier_context::*;

pub fn build_game(game: &mut App, config: GameConfig) {
    // Log panics in browser console
    #[cfg(target_arch = "wasm32")]
    #[cfg(feature = "console_errors")]
    {
        console_error_panic_hook::set_once();
        wasm_logger::init(wasm_logger::Config::default());
    }

    info!("Starting game with config {:?}", config);

    // Pre-spawn entities which will be re-used as game entities
    // for some reason Rapier requires these to be deterministic
    let _ = game
        .world
        .spawn_batch((0..101).map(DeterministicSpawnBundle::new))
        .collect::<Vec<Entity>>();

    // Generic game resources
    game.insert_resource(config.clone())
        .insert_resource(ClearColor(ZOOP_YELLOW));

    // Default Bevy plugins
    game.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: config.canvas_selector.clone(),
            ..default()
        }),
        ..default()
    }));

    // Physics plugin
    game.insert_resource(config.rapier_config());
    game.add_plugin(
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(config.pixels_per_meter)
            .with_default_system_setup(false),
    );

    // Debug line renderer
    #[cfg(feature = "debug_lines")]
    game.add_plugin(DebugLinesPlugin::default());

    // Debug physics renderer
    #[cfg(feature = "rapier_debug_physics")]
    game.add_plugin(RapierDebugRenderPlugin::default());

    // Debug world inspector
    #[cfg(feature = "world_debug")]
    game.add_plugin(WorldInspectorPlugin::new());

    // Init rollback & desync resources
    // frame updating
    game.insert_resource(LastFrame::default());
    game.insert_resource(CurrentFrame::default());
    game.insert_resource(CurrentSessionFrame::default());
    game.insert_resource(ConfirmedFrame::default());
    game.insert_resource(RollbackStatus::default());
    game.insert_resource(ValidatableFrame::default());

    // desync detection
    game.insert_resource(FrameHashes::default());
    game.insert_resource(RxFrameHashes::default());

    // physics toggling
    game.insert_resource(EnablePhysicsAfter::with_default_offset(
        0,
        config.fps as i32,
        config.load_seconds as i32,
    ));
    game.insert_resource(PhysicsEnabled::default());

    // Reset rapier
    game.add_startup_system(reset_rapier);

    // Init game state
    let state = init_scene(&config);
    game.insert_resource(state);
    game.add_startup_system(setup_scene);

    // Configure networking
    build_network(game, &config);

    // Synchronized game logic stage
    let game_schedule = GGRSSchedule;

    game.get_schedule_mut(game_schedule)
        .unwrap()
        .configure_sets(
            (
                GameSet::Rollback,
                GameSet::Game,
                PhysicsSet::SyncBackend,
                PhysicsSet::SyncBackendFlush,
                PhysicsSet::StepSimulation,
                PhysicsSet::Writeback,
                GameSet::SaveAndChecksum,
            )
                .chain()
                .after(CoreSet::UpdateFlush)
                .before(CoreSet::PostUpdate),
        )
        .add_systems(
            (
                update_current_frame,
                update_current_session_frame,
                update_confirmed_frame,
                // the three above must actually come before we update rollback status
                update_rollback_status,
                // these three must actually come after we update rollback status
                update_validatable_frame,
                toggle_physics,
                rollback_rapier_context,
                // Make sure to flush everything before we apply our game logic.
                apply_system_buffers,
            )
                .chain()
                .in_base_set(GameSet::Rollback),
        )
        .add_systems(
            (
                // destroy_scene,
                // setup_scene,
                drive_car,
                // The `frame_validator` relies on the execution of `apply_inputs` and must come after.
                // It could happen anywhere else, I just stuck it here to be clear.
                // If this is causing your game to quit, you have a bug!
                frame_validator,
                force_update_rollbackables,
                // Make sure to flush everything before Rapier syncs
                apply_system_buffers,
            )
                .chain()
                .in_base_set(GameSet::Game),
        )
        .add_systems(
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackend)
                .in_base_set(PhysicsSet::SyncBackend),
        )
        .add_systems(
            (
                rapier_stub
                    .after(bevy::transform::systems::sync_simple_transforms)
                    .before(bevy::transform::systems::propagate_transforms),
                rapier_stub2
                    .after(systems::init_joints)
                    .before(systems::apply_initial_rigid_body_impulses),
            )
                .in_base_set(PhysicsSet::SyncBackend),
        )
        .add_systems(
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackendFlush)
                .in_base_set(PhysicsSet::SyncBackendFlush),
        )
        .add_systems(
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::StepSimulation)
                .in_base_set(PhysicsSet::StepSimulation),
        )
        .add_systems(
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::Writeback)
                .in_base_set(PhysicsSet::Writeback),
        )
        .add_systems(
            (
                save_rapier_context, // This must execute after writeback to store the RapierContext
                apply_system_buffers, // Flushing again
            )
                .chain()
                .in_base_set(GameSet::SaveAndChecksum),
        );

    // Scene setup
    game.add_startup_system(setup_graphics);
}

fn setup_graphics(mut commands: Commands) {
    let bundle = Camera2dBundle::default();
    commands.spawn(bundle);
}

fn rapier_stub() {}

fn rapier_stub2() {}
