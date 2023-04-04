use crate::math::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use shared::{NetworkPlayer, PlayerId, RoomId};
use url::{ParseError, Url};

#[derive(Clone)]
pub struct NetworkConfig {
    pub server_address: Url,
    pub room: RoomId,
}

#[derive(Clone, Debug, Default, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct SerializedRapierContext {
    pub context: Vec<u8>,
    pub initialized: bool,
}

#[derive(Resource, Clone)]
pub struct GameConfig {
    // Native Bevy units:
    // - distance in pixels
    // - angles in radians
    // - percentages from 0.0 to 1.0
    pub network: NetworkConfig,
    pub players: Vec<NetworkPlayer>,
    pub fps: u16,
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
    pub tire_friction_pushback_percentage: f32,
    pub tire_linear_damping: f32,
    pub tire_angular_damping: f32,
}

impl GameConfig {
    fn _meters2pix(pixels_per_meter: f32, meters: f32) -> f32 {
        pixels_per_meter * meters
    }

    pub fn meters2pix(&self, meters: f32) -> f32 {
        GameConfig::_meters2pix(self.pixels_per_meter, meters)
    }

    pub fn default(network: NetworkConfig, players: Vec<NetworkPlayer>) -> GameConfig {
        let ppm = 10.0;
        let m2p = |meters: f32| GameConfig::_meters2pix(ppm, meters);
        GameConfig {
            network,
            players,
            fps: 60,
            pixels_per_meter: ppm,
            car_half_width: m2p(1.0),
            car_half_length: m2p(2.0),
            tire_half_thickness: m2p(0.2),
            tire_radius: m2p(0.4),
            tire_rotation_per_tick: deg2rad(15.0),
            tire_max_angle: deg2rad(35.0),
            tire_acceleration_force: m2p(140.0),
            tire_reversing_force: m2p(100.0),
            tire_breaking_force: m2p(300.0),
            tire_friction_pushback_percentage: 0.5,
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
