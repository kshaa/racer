use crate::math::*;
use crate::sync::*;
use crate::tire::{Tire, TirePhysics};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use url::{ParseError, Url};
use zoop_shared::{NetworkPlayer, PlayerId, RoomId};

#[derive(Clone, Debug)]
pub struct NetworkConfig {
    pub server_address: Url,
    pub room: RoomId,
}

#[derive(Clone, Debug, Default, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct GamePhysicsProps {
    pub transform: Transform,
    pub velocity: Velocity,
    pub force: ExternalForce,
    pub impulse: ExternalImpulse,
    pub mass: ReadMassProperties,
}
impl GamePhysicsProps {
    pub fn of(
        transform: Transform,
        velocity: Velocity,
        force: ExternalForce,
        impulse: ExternalImpulse,
        mass: ReadMassProperties,
    ) -> GamePhysicsProps {
        GamePhysicsProps {
            transform,
            velocity,
            force,
            impulse,
            mass,
        }
    }
    pub fn fixed(position: Vec3) -> GamePhysicsProps {
        GamePhysicsProps {
            transform: Transform::from_translation(position),
            velocity: Velocity::zero(),
            force: ExternalForce::default(),
            impulse: ExternalImpulse::default(),
            mass: ReadMassProperties::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct GameTire {
    pub tire_physics: TirePhysics,
    pub entity_physics: GamePhysicsProps,
}
impl GameTire {
    pub fn of(
        transform: Transform,
        velocity: Velocity,
        force: ExternalForce,
        impulse: ExternalImpulse,
        mass: ReadMassProperties,
        angle: f32,
    ) -> GameTire {
        GameTire {
            tire_physics: TirePhysics { angle },
            entity_physics: GamePhysicsProps::of(transform, velocity, force, impulse, mass),
        }
    }
    pub fn fixed(position: Vec3) -> GameTire {
        GameTire {
            tire_physics: TirePhysics::default(),
            entity_physics: GamePhysicsProps::fixed(position),
        }
    }
}

#[derive(Clone, Debug, Default, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct GameCar {
    pub tire_top_left: GameTire,
    pub tire_top_right: GameTire,
    pub tire_bottom_left: GameTire,
    pub tire_bottom_right: GameTire,
    pub physics: GamePhysicsProps,
    pub player: Player,
}
impl GameCar {
    pub fn fixed_for_player(
        player: Player,
        position: Vec3,
        car_half_size: Vec2,
        tire_half_size: Vec2,
    ) -> GameCar {
        GameCar {
            tire_top_left: GameTire::fixed(Tire::position_for_car(
                position,
                car_half_size,
                tire_half_size,
                true,
                false,
            )),
            tire_top_right: GameTire::fixed(Tire::position_for_car(
                position,
                car_half_size,
                tire_half_size,
                true,
                true,
            )),
            tire_bottom_left: GameTire::fixed(Tire::position_for_car(
                position,
                car_half_size,
                tire_half_size,
                false,
                false,
            )),
            tire_bottom_right: GameTire::fixed(Tire::position_for_car(
                position,
                car_half_size,
                tire_half_size,
                false,
                true,
            )),
            physics: GamePhysicsProps::fixed(position),
            player,
        }
    }
}

#[derive(Clone, Debug, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub enum GameEntity {
    Stub(),
    Car(GameCar),
}
impl Default for GameEntity {
    fn default() -> Self {
        GameEntity::Stub()
    }
}

#[derive(Clone, Debug, Default, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct GameState {
    pub entities: Vec<GameEntity>,
}

#[derive(Resource, Clone, Debug)]
pub struct GameConfig {
    // Native Bevy units:
    // - distance in pixels
    // - angles in radians
    // - percentages from 0.0 to 1.0
    pub network: NetworkConfig,
    pub players: Vec<NetworkPlayer>,
    pub fps: u16,
    pub canvas_selector: Option<String>,
    pub pixels_per_meter: f32,
    pub car_half_width: f32,
    pub car_half_length: f32,
    pub tire_half_thickness: f32,
    pub tire_radius: f32,
    pub tire_rotation_per_tick: f32,
    pub tire_max_angle: f32,
    pub tire_acceleration_force: f32,
    pub tire_reversing_force: f32,
    pub tire_breaking_force: f32,
    pub tire_friction_force: f32,
    pub tire_linear_damping: f32,
    pub tire_angular_damping: f32,
}

impl GameConfig {
    fn _meters2pix(pixels_per_meter: f32, meters: f32) -> f32 {
        pixels_per_meter * meters
    }

    pub fn default(
        network: NetworkConfig,
        players: Vec<NetworkPlayer>,
        canvas_selector: Option<String>,
    ) -> GameConfig {
        let ppm = 10.0;
        let m2p = |meters: f32| GameConfig::_meters2pix(ppm, meters);
        GameConfig {
            network,
            players,
            fps: 60,
            canvas_selector,
            pixels_per_meter: ppm,
            car_half_width: m2p(1.0),
            car_half_length: m2p(2.0),
            tire_half_thickness: m2p(0.2),
            tire_radius: m2p(0.4),
            tire_rotation_per_tick: deg2rad(15.0),
            tire_max_angle: deg2rad(35.0),
            tire_acceleration_force: m2p(14000.0),
            tire_reversing_force: m2p(10000.0),
            tire_breaking_force: m2p(30000.0),
            tire_friction_force: 50.0,
            tire_linear_damping: 0.5,
            tire_angular_damping: 0.1,
        }
    }

    pub fn game_room_address(
        &self,
        room_address: RoomId,
        player_id: PlayerId,
    ) -> Result<Url, ParseError> {
        self.network.server_address.join(
            format!(
                "/game/{}/as/{}",
                room_address.0.to_string(),
                player_id.0.to_string()
            )
            .as_str(),
        )
    }

    pub fn tire_damping(&self) -> Damping {
        Damping {
            linear_damping: self.tire_linear_damping,
            angular_damping: self.tire_angular_damping,
        }
    }

    pub fn tire_half_size(&self) -> Vec2 {
        Vec2 {
            x: self.tire_half_thickness,
            y: self.tire_radius,
        }
    }

    pub fn car_half_size(&self) -> Vec2 {
        Vec2 {
            x: self.car_half_width,
            y: self.car_half_length,
        }
    }

    pub fn rapier_config(&self) -> RapierConfiguration {
        RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: 1.0 / f32::from(self.fps),
                substeps: 1,
            },
            gravity: Vec2::ZERO,
            ..RapierConfiguration::default()
        }
    }
}
