use crate::config::{GameCar, GamePhysicsProps};
use crate::sync::Player;
use crate::tire::*;
use bevy::core::Name;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct CarMeta {
    pub half_size: Vec2,
}

#[derive(Bundle)]
pub struct Car {
    meta: CarMeta,
    name: Name,
    rigid_body: RigidBody,
    rigid_mass: ReadMassProperties,
    // mass: AdditionalMassProperties,
    sleep: Sleeping,
    force: ExternalForce,
    impulse: ExternalImpulse,
    velocity: Velocity,
    collider: Collider,
    sprite: SpriteBundle,
    player: Player,
}

pub struct CarEntities {
    pub body: Entity,
    pub tires: TireEntities,
}

impl Car {
    fn build_without_tires(
        car_title: String,
        half_size: Vec2,
        player: Player,
        color: Color,
        physics: GamePhysicsProps,
    ) -> Car {
        Car {
            meta: CarMeta { half_size },
            rigid_body: RigidBody::Dynamic,
            name: Name::new(car_title),
            rigid_mass: physics.mass,
            // mass: AdditionalMassProperties::Mass(0.001),
            sleep: Sleeping::disabled(),
            force: physics.force,
            impulse: physics.impulse,
            collider: Collider::cuboid(half_size.x, half_size.y),
            velocity: physics.velocity,
            sprite: SpriteBundle {
                transform: physics.transform,
                sprite: Sprite {
                    color,
                    custom_size: Some(half_size * 2.0),
                    ..default()
                },
                ..default()
            },
            player,
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        player: Player,
        car_title: String,
        car_half_size: Vec2,
        tire_half_size: Vec2,
        car_color: Color,
        tire_color: Color,
        tire_damping: Damping,
        car_physics: GameCar,
    ) -> CarEntities {
        let car = commands.spawn(Car::build_without_tires(
            car_title.clone(),
            car_half_size,
            player.clone(),
            car_color,
            car_physics.physics.clone(),
        ));
        let car_id = car.id();
        let tires = Tire::spawn_all_for_car(
            commands,
            player.clone(),
            car_id.clone(),
            car_half_size,
            tire_half_size,
            tire_color,
            tire_damping,
            car_title,
            car_physics,
        );

        CarEntities {
            body: car_id.clone(),
            tires,
        }
    }
}
