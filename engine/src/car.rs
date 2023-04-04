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
    sleep: Sleeping,
    force: ExternalForce,
    collider: Collider,
    sprite: SpriteBundle,
    player: Player,
}

pub struct CarEntities {
    pub body: Entity,
    pub tires: TireEntities,
}

impl Car {
    fn build_without_tires(half_size: Vec2, position: Vec2, player: Player, color: Color) -> Car {
        Car {
            meta: CarMeta { half_size },
            rigid_body: RigidBody::Dynamic,
            name: Name::new(format!("Car #{}", player.handle)),
            rigid_mass: ReadMassProperties::default(),
            sleep: Sleeping::disabled(),
            force: ExternalForce::default(),
            collider: Collider::cuboid(half_size.x, half_size.y),
            sprite: SpriteBundle {
                transform: Transform::from_xyz(position.x, position.y, 0.0),
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
        car_half_size: Vec2,
        car_position: Vec2,
        tire_half_size: Vec2,
        car_color: Color,
        tire_color: Color,
        tire_damping: Damping,
        player: Player,
    ) -> CarEntities {
        let car = commands.spawn(Car::build_without_tires(
            car_half_size,
            car_position,
            player.clone(),
            car_color,
        ));
        let car_id = car.id();
        let tires = Tire::spawn_all_for_car(
            commands,
            car_id.clone(),
            car_position,
            car_half_size,
            tire_half_size,
            tire_color,
            tire_damping,
            player,
        );

        CarEntities {
            body: car_id.clone(),
            tires,
        }
    }
}
