use bevy::prelude::*;
use bevy_ggrs::*;
#[cfg(feature = "world_debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(feature = "debug_lines")]
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

use crate::domain::colors::*;
use crate::domain::game_config::GameConfig;
use crate::domain::game_set::GameSet;
use crate::systems::build_network::*;
use crate::systems::drive_car::*;
use crate::systems::manage_scene::*;

pub fn build_game(game: &mut App, config: GameConfig) {
    // Log panics in browser console
    #[cfg(target_arch = "wasm32")]
    #[cfg(feature = "console_errors")]
    {
        console_error_panic_hook::set_once();
        wasm_logger::init(wasm_logger::Config::default());
    }

    info!("Starting game with config {:?}", config);

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
        .configure_sets((
            GameSet::PrePhysics,
            PhysicsSet::SyncBackend,
            PhysicsSet::SyncBackendFlush,
            PhysicsSet::StepSimulation,
            PhysicsSet::Writeback,
            GameSet::PostPhysics,
        ).chain()
            .after(CoreSet::UpdateFlush)
            .before(CoreSet::PostUpdate))
        .add_systems((
            // destroy_scene,
            // setup_scene,
            drive_car,
        ).chain().in_base_set(GameSet::PrePhysics))
        .add_systems(
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackend)
                .in_base_set(PhysicsSet::SyncBackend)
        )
        .add_systems((
            rapier_stub
                .after(bevy::transform::systems::sync_simple_transforms)
                .before(bevy::transform::systems::propagate_transforms),
            rapier_stub2
                .after(systems::init_joints)
                .before(systems::apply_initial_rigid_body_impulses),
        ).in_base_set(PhysicsSet::SyncBackend))
        .add_systems(
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackendFlush)
                .in_base_set(PhysicsSet::SyncBackendFlush)
        )
        .add_systems(
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::StepSimulation)
                .in_base_set(PhysicsSet::StepSimulation)
        )
        .add_systems(
            RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::Writeback)
                .in_base_set(PhysicsSet::Writeback)
        )
        .add_systems((
            store_scene,
        ).chain().in_base_set(GameSet::PostPhysics));

    // Scene setup
    game.add_startup_system(setup_graphics);
}

fn setup_graphics(mut commands: Commands) {
    let bundle = Camera2dBundle::default();
    commands.spawn(bundle);
}

fn rapier_stub() {}

fn rapier_stub2() {}
