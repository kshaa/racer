use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, PlayerInputs, Session};
#[cfg(feature = "world_debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(feature = "debug_lines")]
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;
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

    // Default Bevy plugins
    game.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            canvas: config.canvas_selector.clone(),
            ..default()
        },
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
    game.add_plugin(WorldInspectorPlugin);

    // Init game state
    let state = init_state(&config);
    game.insert_resource(state);

    // Synchronized game logic stage
    let game_stage = SystemStage::parallel().with_system(keyboard_based_tire_acceleration);

    // Synchronized game schedule
    let mut synchronized_schedule = Schedule::default();
    synchronized_schedule.add_stage(
        "destroy_scene",
        SystemStage::parallel().with_system(destroy_scene),
    );
    synchronized_schedule.add_stage_after(
        "destroy_scene",
        "setup_scene",
        SystemStage::parallel().with_system(setup_scene),
    );
    synchronized_schedule.add_stage_after("setup_scene", "game_logic", game_stage);
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
        "store_scene",
        SystemStage::parallel().with_system(store_scene),
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
        // # physics
        // .register_rollback_component::<Velocity>()
        // .register_rollback_component::<AdditionalMassProperties>()
        // .register_rollback_component::<ReadMassProperties>()
        // // .register_rollback_component::<MassProperties>()
        // .register_rollback_component::<LockedAxes>()
        // .register_rollback_component::<ExternalForce>()
        // .register_rollback_component::<ExternalImpulse>()
        // .register_rollback_component::<Sleeping>()
        // .register_rollback_component::<Damping>()
        // .register_rollback_component::<Dominance>()
        // .register_rollback_component::<Ccd>()
        // .register_rollback_component::<GravityScale>()
        // .register_rollback_component::<CollidingEntities>()
        // .register_rollback_component::<Sensor>()
        // .register_rollback_component::<Friction>()
        // .register_rollback_component::<Restitution>()
        // .register_rollback_component::<CollisionGroups>()
        // .register_rollback_component::<SolverGroups>()
        // .register_rollback_component::<ContactForceEventThreshold>()
        // .register_rollback_component::<Group>()
        .register_rollback_resource::<GameState>()
        // # bevy
        // .register_rollback_component::<Transform>()
        // # game
        // .register_rollback_component::<TireMeta>()
        // these systems will be executed as part of the advance frame update
        .with_rollback_schedule(synchronized_schedule)
        .build(game);

    game.insert_resource(Session::P2PSession(session));
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

fn init_state(config: &GameConfig) -> GameState {
    let cars = config
        .players
        .iter()
        .enumerate()
        .map(|(handle, _)| {
            let player = Player { handle };
            let position = Vec3 {
                x: config.car_half_size().x * 6.0 * (handle as f32),
                y: 0.0,
                z: 0.0,
            };
            let tire_half_size = config.tire_half_size();
            let car_half_size = config.car_half_size();

            GameEntity::Car(GameCar::fixed_for_player(
                player,
                position,
                car_half_size,
                tire_half_size,
            ))
        })
        .collect();

    GameState { entities: cars }
}

fn destroy_scene(
    config: Res<GameConfig>,
    player_entities: Query<(Entity, With<Player>)>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut rapier_render_time: ResMut<SimulationToRenderTime>,
    mut rapier_context: ResMut<RapierContext>,
    mut rapier_collision_events: ResMut<Events<CollisionEvent>>,
    mut rapier_force_events: ResMut<Events<ContactForceEvent>>,
    mut rapier_hooks: ResMut<PhysicsHooksWithQueryResource<NoUserData>>,
    mut commands: Commands,
) {
    for (entity, _) in player_entities.iter() {
        commands.entity(entity).despawn();
    }
    *rapier_config = config.rapier_config();
    *rapier_render_time = SimulationToRenderTime::default();
    *rapier_collision_events = Events::<CollisionEvent>::default();
    *rapier_force_events = Events::<ContactForceEvent>::default();
    *rapier_context = RapierContext::default();
    *rapier_hooks = PhysicsHooksWithQueryResource::<NoUserData>(Box::new(()));
}

fn setup_scene(config: Res<GameConfig>, state: Res<GameState>, mut commands: Commands) {
    spawn_scene(config.as_ref(), &state, &mut commands);
}

fn spawn_scene(config: &GameConfig, state: &GameState, commands: &mut Commands) {
    for entity in state.entities.iter() {
        match entity {
            GameEntity::Stub() => (),
            GameEntity::Car(car) => setup_car(config, car.clone(), commands),
        }
    }
}

fn store_car(
    car_query: &mut Query<
        (
            &Transform,
            &Velocity,
            &ExternalForce,
            &ExternalImpulse,
            &ReadMassProperties,
            &Player,
        ),
        Without<TireMeta>,
    >,
    fallback: GamePhysicsProps,
    player_handle: usize,
) -> GamePhysicsProps {
    // Sort query for more determinism
    let mut query = car_query.iter_mut().collect::<Vec<_>>();
    query.sort_by_key(|(_, _, _, _, _, player)| player.handle);

    // Store tire from ECS query into game state
    query
        .into_iter()
        .find(|(_, _, _, _, _, player)| player.handle == player_handle)
        .map(|(transform, velocity, force, impulse, mass, _)| {
            GamePhysicsProps::of(
                transform.clone(),
                velocity.clone(),
                force.clone(),
                impulse.clone(),
                mass.clone(),
            )
        })
        .unwrap_or_else(|| fallback)
}

fn store_tire(
    tire_query: &mut Query<
        (
            &Transform,
            &Velocity,
            &ExternalForce,
            &ExternalImpulse,
            &ReadMassProperties,
            &TirePhysics,
            &TireMeta,
            &Player,
        ),
        Without<CarMeta>,
    >,
    is_front: bool,
    is_right: bool,
    fallback: GameTire,
    player_handle: usize,
) -> GameTire {
    // Sort query for more determinism
    let mut query = tire_query.iter_mut().collect::<Vec<_>>();
    query.sort_by_key(|(_, _, _, _, _, _, meta, player)| {
        (
            player.handle,
            if meta.is_front { 1 } else { 0 },
            if meta.is_right { 1 } else { 0 },
        )
    });

    // Store tire from ECS query into game state
    query
        .into_iter()
        .find(|(_, _, _, _, _, _, meta, player)| {
            player.handle == player_handle && meta.is_front == is_front && meta.is_right == is_right
        })
        .map(
            |(transform, velocity, force, impulse, mass, physics, _, _)| {
                GameTire::of(
                    transform.clone(),
                    velocity.clone(),
                    force.clone(),
                    impulse.clone(),
                    mass.clone(),
                    physics.angle,
                )
            },
        )
        .unwrap_or_else(|| fallback)
}

fn store_scene(
    mut car_query: Query<
        (
            &Transform,
            &Velocity,
            &ExternalForce,
            &ExternalImpulse,
            &ReadMassProperties,
            &Player,
        ),
        Without<TireMeta>,
    >,
    mut tire_query: Query<
        (
            &Transform,
            &Velocity,
            &ExternalForce,
            &ExternalImpulse,
            &ReadMassProperties,
            &TirePhysics,
            &TireMeta,
            &Player,
        ),
        Without<CarMeta>,
    >,
    mut state: ResMut<GameState>,
) {
    let new_entities = state
        .entities
        .iter_mut()
        .map(|e| match e {
            GameEntity::Stub() => GameEntity::Stub(),
            GameEntity::Car(c) => {
                // Take existing car
                let mut new_car = c.clone();

                // Copy car
                new_car.physics = store_car(&mut car_query, new_car.physics, c.player.handle);
                new_car.tire_top_right = store_tire(
                    &mut tire_query,
                    true,
                    true,
                    new_car.tire_top_right.clone(),
                    c.player.handle,
                );
                new_car.tire_top_left = store_tire(
                    &mut tire_query,
                    true,
                    false,
                    new_car.tire_top_left.clone(),
                    c.player.handle,
                );
                new_car.tire_bottom_right = store_tire(
                    &mut tire_query,
                    false,
                    true,
                    new_car.tire_bottom_right.clone(),
                    c.player.handle,
                );
                new_car.tire_bottom_left = store_tire(
                    &mut tire_query,
                    false,
                    false,
                    new_car.tire_bottom_left.clone(),
                    c.player.handle,
                );

                // Store new car
                GameEntity::Car(new_car)
            }
        })
        .collect();
    state.entities = new_entities;
}

fn setup_car(config: &GameConfig, car: GameCar, commands: &mut Commands) {
    Car::spawn(
        commands,
        car.player.clone(),
        String::from(format!("Car #{}", car.player.handle)),
        config.car_half_size(),
        config.tire_half_size(),
        ZOOP_RED,
        ZOOP_BLACK,
        config.tire_damping(),
        car,
    );
}

fn keyboard_based_tire_acceleration(
    config: Res<GameConfig>,
    inputs: Res<PlayerInputs<GGRSConfig>>,
    mut source_car_query: Query<(&CarMeta, &Transform, &Player), Without<TireMeta>>,
    mut source_tire_query: Query<
        (
            &mut TirePhysics,
            &TireMeta,
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
    // Sort queries for more determinism
    let mut car_query = source_car_query.iter_mut().collect::<Vec<_>>();
    car_query.sort_by_key(|(_, _, player)| player.handle);

    let mut tire_query = source_tire_query.iter_mut().collect::<Vec<_>>();
    tire_query.sort_by_key(|(_, meta, _, _, _, _, player)| {
        (
            player.handle,
            if meta.is_front { 1 } else { 0 },
            if meta.is_right { 1 } else { 0 },
        )
    });

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

    for (
        mut tire_physics,
        tire_meta,
        mut transform,
        velocity,
        mut forcable,
        mut impulsable,
        tire_player,
    ) in tire_query
    {
        let controls = Controls {
            input: inputs[tire_player.handle].0.input,
        };

        // Apply controls to tire angle
        tire_physics.angle = tire_physics.angle
            + tire_angle_change(
                tire_meta,
                tire_physics.as_ref(),
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
        let tire_rotation = car_rotation + tire_physics.angle;
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
            config.tire_friction_force,
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
