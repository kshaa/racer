use crate::math::*;
use crate::sync::Player;
use bevy::core::Name;
use bevy::math::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Debug, Default, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct TireMeta {
    pub half_size: Vec2,
    pub is_front: bool,
    pub is_right: bool,
    pub angle: f32,
}

#[derive(Bundle)]
pub struct Tire {
    meta: TireMeta,
    name: Name,
    rigid_body: RigidBody,
    damping: Damping,
    velocity: Velocity,
    read_mass_properties: ReadMassProperties,
    sleeping: Sleeping,
    external_force: ExternalForce,
    external_impulse: ExternalImpulse,
    collider: Collider,
    sprite_bundle: SpriteBundle,
    player: Player,
}

pub struct TireEntities {
    pub top_right: Entity,
    pub top_left: Entity,
    pub bottom_right: Entity,
    pub bottom_left: Entity,
}

impl Tire {
    fn build(
        is_front: bool,
        is_right: bool,
        half_size: Vec2,
        player: Player,
        color: Color,
        position: Vec3,
        damping: Damping,
    ) -> Tire {
        let front_title = if is_front { "F" } else { "B" };
        let side_title = if is_right { "R" } else { "L" };

        Tire {
            meta: TireMeta {
                half_size,
                is_front,
                is_right,
                angle: 0.0,
            },
            name: Name::new(format!(
                "Tire #{} {}, {}",
                player.handle, front_title, side_title
            )),
            rigid_body: RigidBody::Dynamic,
            damping,
            velocity: Velocity::default(),
            read_mass_properties: ReadMassProperties::default(),
            sleeping: Sleeping::disabled(),
            external_force: ExternalForce::default(),
            external_impulse: ExternalImpulse::default(),
            collider: Collider::cuboid(half_size.x, half_size.y),
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(position.x, position.y, position.z),
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

    fn spawn_one_for_car(
        commands: &mut Commands,
        car: Entity,
        car_position: Vec2,
        car_half_size: Vec2,
        tire_half_size: Vec2,
        is_front: bool,
        is_right: bool,
        player: Player,
        color: Color,
        damping: Damping,
    ) -> Entity {
        let car_anchor = Vec2 {
            x: signed(is_right, car_half_size.x + tire_half_size.x * 2.0),
            y: signed(is_front, car_half_size.y - tire_half_size.y),
        };
        let tire_anchor = Vec2::new(signed(is_right, tire_half_size.x), 0.0);
        let joint = FixedJointBuilder::new()
            .local_anchor1(car_anchor)
            .local_anchor2(tire_anchor);
        let position = Vec3 {
            x: car_position.x + signed(is_right, car_half_size.x + tire_half_size.x * 3.0),
            y: car_position.y + signed(is_front, car_half_size.y - tire_half_size.y),
            z: 0.0,
        };

        commands
            .spawn(Tire::build(
                is_front,
                is_right,
                tire_half_size,
                player,
                color,
                position,
                damping,
            ))
            .insert(ImpulseJoint::new(car, joint))
            .id()
    }

    pub fn spawn_all_for_car(
        commands: &mut Commands,
        car: Entity,
        car_position: Vec2,
        car_half_size: Vec2,
        tire_half_size: Vec2,
        color: Color,
        damping: Damping,
        player: Player,
    ) -> TireEntities {
        TireEntities {
            top_right: Tire::spawn_one_for_car(
                commands,
                car,
                car_position,
                car_half_size,
                tire_half_size,
                true,
                true,
                player.clone(),
                color,
                damping,
            ),
            top_left: Tire::spawn_one_for_car(
                commands,
                car,
                car_position,
                car_half_size,
                tire_half_size,
                true,
                false,
                player.clone(),
                color,
                damping,
            ),
            bottom_right: Tire::spawn_one_for_car(
                commands,
                car,
                car_position,
                car_half_size,
                tire_half_size,
                false,
                true,
                player.clone(),
                color,
                damping,
            ),
            bottom_left: Tire::spawn_one_for_car(
                commands,
                car,
                car_position,
                car_half_size,
                tire_half_size,
                false,
                false,
                player.clone(),
                color,
                damping,
            ),
        }
    }
}
