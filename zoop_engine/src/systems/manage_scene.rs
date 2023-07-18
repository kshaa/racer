use crate::domain::car::spawn_car;

use crate::domain::colors::{ZOOP_BLACK, ZOOP_RED};
use crate::domain::game_config::GameConfig;
use crate::domain::game_state::{GameCar, GameEntity, GameState};
use crate::domain::player::Player;
use crate::domain::spawn::DeterministicSpawn;

use bevy::prelude::*;
use bevy_ggrs::RollbackIdProvider;
use bevy_rapier2d::prelude::*;

pub fn init_scene(config: &GameConfig) -> GameState {
    println!("Initiating scene state");
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

pub fn destroy_scene(
    config: Res<GameConfig>,
    player_entities: Query<(Entity, With<Player>)>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut rapier_render_time: ResMut<SimulationToRenderTime>,
    mut rapier_context: ResMut<RapierContext>,
    mut rapier_collision_events: ResMut<Events<CollisionEvent>>,
    mut rapier_force_events: ResMut<Events<ContactForceEvent>>,
    mut commands: Commands,
) {
    println!("Destroying rapier context");
    for (entity, _) in player_entities.iter() {
        commands.entity(entity).despawn();
    }
    *rapier_config = config.rapier_config();
    *rapier_render_time = SimulationToRenderTime::default();
    *rapier_collision_events = Events::<CollisionEvent>::default();
    *rapier_force_events = Events::<ContactForceEvent>::default();
    *rapier_context = RapierContext::default();
}

pub fn setup_scene(
    config: Res<GameConfig>,
    state: Res<GameState>,
    mut rip: ResMut<RollbackIdProvider>,
    spawn_pool: Query<(Entity, &DeterministicSpawn)>,
    mut commands: Commands,
) {
    // Get our entities and sort them by the spawn component index
    let mut sorted_spawn_pool: Vec<(Entity, &DeterministicSpawn)> = spawn_pool.iter().collect();
    sorted_spawn_pool.sort_by_key(|e| e.1.index);
    // Get the Entities in reverse for easy popping
    let mut sorted_entity_pool: Vec<Entity> = sorted_spawn_pool.iter().map(|p| p.0).rev().collect();

    spawn_scene(
        config.as_ref(),
        &state,
        &mut commands,
        &mut sorted_entity_pool,
        &mut rip,
    );
}

pub fn spawn_scene(
    config: &GameConfig,
    state: &GameState,
    commands: &mut Commands,
    spawn_pool: &mut Vec<Entity>,
    rip: &mut RollbackIdProvider,
) {
    println!("Spawning scene from state");
    for entity in state.entities.iter() {
        match entity {
            GameEntity::Stub() => (),
            GameEntity::Car(car) => {
                println!("Spawning car for player {}", car.player.handle);
                setup_car(config, car.clone(), commands, spawn_pool, rip)
            }
        }
    }
    while !spawn_pool.is_empty() {
        let mut leftover = commands.entity(spawn_pool.pop().unwrap());
        leftover.despawn();
    }
}
//
// pub fn store_car(
//     car_query: &mut Query<
//         (
//             &Transform,
//             &Velocity,
//             &ExternalForce,
//             &ExternalImpulse,
//             &ReadMassProperties,
//             &Player,
//         ),
//         Without<TireMeta>,
//     >,
//     fallback: EntityPhysics,
//     player_handle: usize,
// ) -> EntityPhysics {
//     // Sort query for more determinism
//     let mut query = car_query.iter_mut().collect::<Vec<_>>();
//     query.sort_by_key(|(_, _, _, _, _, player)| player.handle);
//
//     // Store tire from ECS query into game state
//     query
//         .into_iter()
//         .find(|(_, _, _, _, _, player)| player.handle == player_handle)
//         .map(|(transform, velocity, force, impulse, mass, _)| {
//             EntityPhysics::of(
//                 transform.clone(),
//                 velocity.clone(),
//                 force.clone(),
//                 impulse.clone(),
//                 mass.clone(),
//             )
//         })
//         .unwrap_or_else(|| fallback)
// }
//
// pub fn store_tire(
//     tire_query: &mut Query<
//         (
//             &Transform,
//             &Velocity,
//             &ExternalForce,
//             &ExternalImpulse,
//             &ReadMassProperties,
//             &TirePhysics,
//             &TireMeta,
//             &Player,
//         ),
//         Without<CarMeta>,
//     >,
//     is_front: bool,
//     is_right: bool,
//     fallback: GameTire,
//     player_handle: usize,
// ) -> GameTire {
//     // Sort query for more determinism
//     let mut query = tire_query.iter_mut().collect::<Vec<_>>();
//     query.sort_by_key(|(_, _, _, _, _, _, meta, player)| {
//         (
//             player.handle,
//             if meta.is_front { 1 } else { 0 },
//             if meta.is_right { 1 } else { 0 },
//         )
//     });
//
//     // Store tire from ECS query into game state
//     query
//         .into_iter()
//         .find(|(_, _, _, _, _, _, meta, player)| {
//             player.handle == player_handle && meta.is_front == is_front && meta.is_right == is_right
//         })
//         .map(
//             |(transform, velocity, force, impulse, mass, physics, _, _)| {
//                 GameTire::of(
//                     transform.clone(),
//                     velocity.clone(),
//                     force.clone(),
//                     impulse.clone(),
//                     mass.clone(),
//                     physics.angle,
//                 )
//             },
//         )
//         .unwrap_or_else(|| fallback)
// }
//
// pub fn store_scene(
//     mut car_query: Query<
//         (
//             &Transform,
//             &Velocity,
//             &ExternalForce,
//             &ExternalImpulse,
//             &ReadMassProperties,
//             &Player,
//         ),
//         Without<TireMeta>,
//     >,
//     mut tire_query: Query<
//         (
//             &Transform,
//             &Velocity,
//             &ExternalForce,
//             &ExternalImpulse,
//             &ReadMassProperties,
//             &TirePhysics,
//             &TireMeta,
//             &Player,
//         ),
//         Without<CarMeta>,
//     >,
//     mut state: ResMut<GameState>,
// ) {
//     // println!("Storing state from scene");
//     let new_entities = state
//         .entities
//         .iter_mut()
//         .map(|e| match e {
//             GameEntity::Stub() => GameEntity::Stub(),
//             GameEntity::Car(c) => {
//                 // Take existing car
//                 let mut new_car = c.clone();
//
//                 // Copy car
//                 new_car.physics = store_car(&mut car_query, new_car.physics, c.player.handle);
//                 new_car.tire_top_right = store_tire(
//                     &mut tire_query,
//                     true,
//                     true,
//                     new_car.tire_top_right.clone(),
//                     c.player.handle,
//                 );
//                 new_car.tire_top_left = store_tire(
//                     &mut tire_query,
//                     true,
//                     false,
//                     new_car.tire_top_left.clone(),
//                     c.player.handle,
//                 );
//                 new_car.tire_bottom_right = store_tire(
//                     &mut tire_query,
//                     false,
//                     true,
//                     new_car.tire_bottom_right.clone(),
//                     c.player.handle,
//                 );
//                 new_car.tire_bottom_left = store_tire(
//                     &mut tire_query,
//                     false,
//                     false,
//                     new_car.tire_bottom_left.clone(),
//                     c.player.handle,
//                 );
//
//                 // Store new car
//                 GameEntity::Car(new_car)
//             }
//         })
//         .collect();
//     state.entities = new_entities;
// }

pub fn setup_car(
    config: &GameConfig,
    car: GameCar,
    commands: &mut Commands,
    spawn_pool: &mut Vec<Entity>,
    rip: &mut RollbackIdProvider,
) {
    spawn_car(
        commands,
        spawn_pool,
        rip,
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
