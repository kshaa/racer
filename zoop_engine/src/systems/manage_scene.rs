use crate::domain::car::spawn_car;
use bevy::asset::LoadState;


use crate::domain::colors::{ZOOP_BLACK, ZOOP_RED, ZOOP_YELLOW};
use crate::domain::game_config::GameConfig;
use crate::domain::game_state::{GameBuilding, GameCar, GameEntity, GameState};
use crate::domain::player::Player;
use crate::domain::spawn::DeterministicSpawn;

use crate::domain::building::Building;
use crate::domain::game_readiness::GameReadiness;
use crate::domain::spritesheets::SpriteSheets;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_ggrs::{Rollback, RollbackIdProvider};
use bevy_rapier2d::prelude::*;
use bevy_sprite3d::Sprite3dParams;

pub fn init_scene(config: &GameConfig) -> GameState {
    println!("Initiating scene state");
    let mut cars = config
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

    let building_half_size = config.meters2pix(5.0);
    let building = GameEntity::Building(GameBuilding::of(
        Vec3 {
            x: -building_half_size * 1.5,
            y: 0.0,
            z: building_half_size,
        },
        1,
        building_half_size,
    ));
    let mut buildings = vec![building];

    let mut entities = vec![];
    entities.append(&mut cars);
    entities.append(&mut buildings);

    GameState { entities }
}

#[allow(dead_code)]
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

pub fn init_materials(
    mut spritesheets: ResMut<SpriteSheets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let cube = meshes.add(shape::Cube::default().into());
    spritesheets.debug_material = debug_material;
    spritesheets.cube = cube;
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

pub fn setup_scene(
    asset_server: Res<AssetServer>,
    mut background: ResMut<ClearColor>,
    mut next_state: ResMut<NextState<GameReadiness>>,
    spritesheets: Res<SpriteSheets>,
    mut sprite_params: Sprite3dParams,
    config: Res<GameConfig>,
    state: Res<GameState>,
    mut rip: ResMut<RollbackIdProvider>,
    spawn_pool: Query<(Entity, &DeterministicSpawn)>,
    mut commands: Commands,
) {
    // Check assets loaded
    if asset_server.get_load_state(&spritesheets.car) != LoadState::Loaded {
        return;
    }
    if asset_server.get_load_state(&spritesheets.tire) != LoadState::Loaded {
        return;
    }

    // Visual loading state logic
    next_state.set(GameReadiness::Ready);
    background.0 = ZOOP_YELLOW;

    // Get our entities and sort them by the spawn component index
    let mut sorted_spawn_pool: Vec<(Entity, &DeterministicSpawn)> = spawn_pool.iter().collect();
    sorted_spawn_pool.sort_by_key(|e| e.1.index);
    // Get the Entities in reverse for easy popping
    let mut sorted_entity_pool: Vec<Entity> = sorted_spawn_pool.iter().map(|p| p.0).rev().collect();

    spawn_scene(
        &spritesheets,
        &mut sprite_params,
        config.as_ref(),
        &state,
        &mut commands,
        &mut sorted_entity_pool,
        &mut rip,
    );
}

pub fn spawn_scene(
    spritesheets: &SpriteSheets,
    sprite_params: &mut Sprite3dParams,
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
                setup_car(
                    spritesheets,
                    sprite_params,
                    config,
                    car.clone(),
                    commands,
                    spawn_pool,
                    rip,
                )
            }
            GameEntity::Building(building) => setup_building(
                spritesheets,
                config,
                building.clone(),
                commands,
                spawn_pool,
                rip,
            ),
        }
    }

    while !spawn_pool.is_empty() {
        let mut leftover = commands.entity(spawn_pool.pop().unwrap());
        leftover.despawn();
    }
}

pub fn setup_building(
    spritesheets: &SpriteSheets,
    _config: &GameConfig,
    building: GameBuilding,
    commands: &mut Commands,
    spawn_pool: &mut Vec<Entity>,
    rip: &mut RollbackIdProvider,
) {
    let mut entity = commands.entity(spawn_pool.pop().unwrap());
    entity.insert(Rollback::new(rip.next_id()));
    entity.insert(Building::build(spritesheets, building));
}

pub fn setup_car(
    spritesheets: &SpriteSheets,
    sprite_params: &mut Sprite3dParams,
    config: &GameConfig,
    car: GameCar,
    commands: &mut Commands,
    spawn_pool: &mut Vec<Entity>,
    rip: &mut RollbackIdProvider,
) {
    spawn_car(
        spritesheets,
        sprite_params,
        config.pixels_per_meter,
        commands,
        spawn_pool,
        rip,
        car.player.clone(),
        String::from(format!("Car #{}", car.player.handle)),
        config.car_half_size(),
        config.car_radius,
        config.tire_half_size(),
        ZOOP_RED,
        ZOOP_BLACK,
        config.tire_damping(),
        car,
    );
}
