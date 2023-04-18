use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, PlayerInputs, Session};
#[cfg(feature = "world_debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(feature = "debug_lines")]
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::PhysicsPipeline;
use ggrs::{P2PSession, PlayerType, SessionBuilder};

use crate::car::*;
use crate::colors::*;
use crate::config::*;
use crate::controls::*;
use crate::math::*;
use crate::movement::*;
use crate::sync::*;
use crate::tire::*;
use crate::websocket::*;

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

    // Physics plugin
    game.insert_resource(config.rapier_config()).add_plugin(
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(config.pixels_per_meter)
            .with_default_system_setup(false),
    );

    // Debug line renderer
    #[cfg(feature = "debug_lines")]
    game.add_plugin(DebugLinesPlugin::default());

    // Debug physics renderer
    #[cfg(feature = "rapier_debug_physics")]
    game.add_plugin(RapierDebugRenderPlugin::default());

    // Default Bevy plugins
    game.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            canvas: config.canvas_selector.clone(),
            ..default()
        },
        ..default()
    }));

    // Debug world inspector
    #[cfg(feature = "world_debug")]
    game.add_plugin(WorldInspectorPlugin);

    // Synchronized game logic stage
    let game_stage = SystemStage::parallel().with_system(keyboard_based_tire_acceleration);

    // Synchronized game schedule
    let mut synchronized_schedule = Schedule::default();
    synchronized_schedule.add_stage(
        "rapier_load",
        SystemStage::parallel().with_system(rapier_context_load),
    );
    synchronized_schedule.add_stage_after("rapier_load", "game_logic", game_stage);
    synchronized_schedule.add_stage_after(
        "game_logic",
        "rapier_sync_backend",
        SystemStage::parallel().with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
            PhysicsStages::SyncBackend,
        )),
    );
    synchronized_schedule.add_stage_after(
        "rapier_sync_backend",
        "rapier_step_simulation",
        SystemStage::parallel().with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
            PhysicsStages::StepSimulation,
        )),
    );
    synchronized_schedule.add_stage_after(
        "rapier_step_simulation",
        "rapier_writeback",
        SystemStage::parallel().with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
            PhysicsStages::Writeback,
        )),
    );
    synchronized_schedule.add_stage_after(
        "rapier_writeback",
        "rapier_detect_despawn",
        SystemStage::parallel().with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
            PhysicsStages::DetectDespawn,
        )),
    );
    synchronized_schedule.add_stage_after(
        "rapier_detect_despawn",
        "rapier_store",
        SystemStage::parallel().with_system(rapier_context_store),
    );

    // Configure networking
    let session = start_network_session(&config);
    build_network(game, &config, synchronized_schedule, session);

    // Scene setup
    game.add_startup_system(setup_graphics)
        .add_startup_system(setup_scene);
}

fn build_network(
    game: &mut App,
    config: &GameConfig,
    synchronized_schedule: Schedule,
    session: P2PSession<GGRSConfig>,
) {
    GGRSPlugin::<GGRSConfig>::new()
        // define frequency of rollback game logic update
        .with_update_frequency(usize::from(config.fps))
        // define system that returns inputs given a player handle, so GGRS can send the inputs around
        .with_input_system(synchronized_input)
        // register types of components AND resources you want to be rolled back
        .register_rollback_component::<Transform>()
        .register_rollback_component::<Velocity>()
        .register_rollback_component::<Damping>()
        .register_rollback_component::<ExternalForce>()
        .register_rollback_component::<ExternalImpulse>()
        .register_rollback_component::<ReadMassProperties>()
        .register_rollback_component::<Sleeping>()
        .register_rollback_component::<TireMeta>()
        .register_rollback_resource::<SerializedRapierContext>()
        // these systems will be executed as part of the advance frame update
        .with_rollback_schedule(synchronized_schedule)
        .build(game);

    game.insert_resource(SerializedRapierContext::default())
        .insert_resource(Session::P2PSession(session));
}

fn start_network_session(config: &GameConfig) -> P2PSession<GGRSConfig> {
    // Create a GGRS session
    let mut session_builder = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(config.players.len())
        .with_max_prediction_window(12) // (optional) set max prediction window
        .with_input_delay(2); // (optional) set input delay for the local player

    // Add players
    for (i, network_player) in config.players.iter().enumerate() {
        // local player
        if network_player.is_local {
            session_builder = session_builder.add_player(PlayerType::Local, i).unwrap();
        } else {
            // remote players
            session_builder = session_builder
                .add_player(PlayerType::Remote(network_player.id.clone()), i)
                .unwrap();
        }
    }

    // Start the GGRS session
    let local_player = config.players.iter().find(|p| p.is_local).unwrap();
    let room_address = config
        .game_room_address(config.network.room.clone(), local_player.id.clone())
        .unwrap();
    let socket = NonBlockingWebSocket::connect(room_address.to_string()).unwrap();

    session_builder.start_p2p_session(socket).unwrap()
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    let bundle = Camera2dBundle::default();
    commands.spawn(bundle);
}

fn setup_scene(config: Res<GameConfig>, mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));

    /* Create cars */
    for (handle, _) in config.players.iter().enumerate() {
        Car::spawn(
            &mut commands,
            config.car_half_size(),
            Vec2 {
                x: config.car_half_size().x * 4.0 * (handle as f32),
                y: 0.0,
            },
            config.tire_half_size(),
            ZOOP_RED,
            ZOOP_BLACK,
            config.tire_damping(),
            Player { handle },
        );
    }
}

fn rapier_context_load(
    serialized: Res<SerializedRapierContext>,
    mut unserialized: ResMut<RapierContext>,
) {
    if serialized.as_ref().initialized {
        let context: RapierContext = bincode::deserialize(&serialized.as_ref().context).unwrap();
        unserialized.islands = context.islands;
        unserialized.broad_phase = context.broad_phase;
        unserialized.narrow_phase = context.narrow_phase;
        unserialized.bodies = context.bodies;
        unserialized.colliders = context.colliders;
        unserialized.impulse_joints = context.impulse_joints;
        unserialized.multibody_joints = context.multibody_joints;
        unserialized.ccd_solver = context.ccd_solver;
        unserialized.query_pipeline = context.query_pipeline;
        unserialized.integration_parameters = context.integration_parameters;
        unserialized.pipeline = PhysicsPipeline::new();
    }
}

fn rapier_context_store(
    mut serialized: ResMut<SerializedRapierContext>,
    unserialized: Res<RapierContext>,
) {
    *serialized = SerializedRapierContext {
        context: bincode::serialize(&unserialized.as_ref()).unwrap(),
        initialized: true,
    };
}

fn keyboard_based_tire_acceleration(
    config: Res<GameConfig>,
    inputs: Res<PlayerInputs<GGRSConfig>>,
    car_query: Query<(&CarMeta, &Transform, &Player), Without<TireMeta>>,
    mut tire_query: Query<
        (
            &mut TireMeta,
            &mut Transform,
            &Velocity,
            &mut ExternalForce,
            &mut ExternalImpulse,
            &Player,
        ),
        Without<CarMeta>,
    >,
    #[cfg(feature = "debug_lines")] mut lines: ResMut<DebugLines>,
) {
    #[cfg(feature = "debug_lines")]
    {
        for (_, car_transform, _) in &car_query {
            let (_, _, car_rotation) = car_transform.rotation.to_euler(EulerRot::XYZ);
            let car_direction = Vec2::from_angle(car_rotation);
            let car_direction3 = Vec3 {
                x: car_direction.x,
                y: car_direction.y,
                z: 0.0,
            } * 60.0;
            lines.line(
                car_transform.translation,
                car_transform.translation + car_direction3,
                0.0,
            );
        }
    }

    for (mut tire_meta, mut transform, velocity, mut forcable, mut impulsable, tire_player) in
        &mut tire_query
    {
        // Parse controls from keyboard
        // let controls = if tire_meta.tire_set_index == 0 {
        //     Controls::from_wasd(&input)
        // } else if tire_meta.tire_set_index == 1 {
        //     Controls::from_ijkl(&input)
        // } else {
        //     Controls::none()
        // };
        let controls = Controls {
            input: inputs[tire_player.handle].0.input,
        };

        // Apply controls to tire angle
        tire_meta.angle = tire_meta.angle
            + tire_angle_change(
                tire_meta.as_ref(),
                &controls,
                config.tire_max_angle,
                config.tire_rotation_per_tick,
            );

        // Apply tire angle to tire rotation transform
        let (_, car_transform, _) = car_query
            .iter()
            .find(|(_, _, car_player)| car_player.handle == tire_player.handle)
            .unwrap();
        let (_, _, car_rotation) = car_transform.rotation.to_euler(EulerRot::XYZ);
        let tire_rotation = car_rotation + tire_meta.angle;
        let tire_direction = Vec2::from_angle(tire_rotation + deg2rad(90.0)).normalize_or_zero();
        let direction_velocity = velocity.linvel.dot(tire_direction);
        #[cfg(feature = "debug_lines")]
        {
            let tire_direction3 = Vec3 {
                x: tire_direction.x,
                y: tire_direction.y,
                z: 0.0,
            } * 60.0;
            lines.line(
                transform.translation,
                transform.translation + tire_direction3,
                0.0,
            );
        }
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, tire_rotation);

        // Apply tire acceleration
        forcable.force = tire_direction
            * tire_acceleration(
                &tire_meta,
                &controls,
                config.pixels_per_meter,
                direction_velocity,
                config.tire_acceleration_force,
                config.tire_reversing_force,
                config.tire_breaking_force,
            );

        // Apply friction
        let friction_impulse = tire_friction_impulse(
            config.tire_friction_pushback_percentage,
            &tire_direction,
            &velocity.linvel,
        );
        #[cfg(feature = "debug_lines")]
        {
            let friction_impulse3 = Vec3 {
                x: friction_impulse.x,
                y: friction_impulse.y,
                z: 0.0,
            } * 60.0;
            lines.line(
                transform.translation,
                transform.translation + friction_impulse3,
                0.0,
            );
        }
        impulsable.impulse += friction_impulse;
    }
}
