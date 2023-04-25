use crate::config::{GameCar, GameTire};
use crate::math::*;
use crate::sync::Player;
use bevy::core::Name;
use bevy::math::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Debug, Default, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct TireMeta {
    pub is_front: bool,
    pub is_right: bool,
}

#[derive(Copy, Clone, Debug, Default, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct TirePhysics {
    pub angle: f32,
}

#[derive(Bundle)]
pub struct Tire {
    meta: TireMeta,
    phyics: TirePhysics,
    name: Name,
    rigid_body: RigidBody,
    damping: Damping,
    velocity: Velocity,
    read_mass_properties: ReadMassProperties,
    // mass: AdditionalMassProperties,
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
        player: Player,
        is_front: bool,
        is_right: bool,
        half_size: Vec2,
        car_title: String,
        color: Color,
        damping: Damping,
        physics: GameTire,
    ) -> Tire {
        let front_title = if is_front { "F" } else { "B" };
        let side_title = if is_right { "R" } else { "L" };

        Tire {
            meta: TireMeta { is_front, is_right },
            phyics: physics.tire_physics,
            name: Name::new(format!(
                "Tire {}{} for {}",
                front_title, side_title, car_title
            )),
            rigid_body: RigidBody::Dynamic,
            damping,
            velocity: physics.entity_physics.velocity,
            read_mass_properties: physics.entity_physics.mass,
            // mass: AdditionalMassProperties::Mass(0.001),
            sleeping: Sleeping::disabled(),
            external_force: physics.entity_physics.force,
            external_impulse: physics.entity_physics.impulse,
            collider: Collider::cuboid(half_size.x, half_size.y),
            sprite_bundle: SpriteBundle {
                transform: physics.entity_physics.transform,
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
        player: Player,
        car: Entity,
        car_anchor: FixedJointBuilder,
        tire_half_size: Vec2,
        is_front: bool,
        is_right: bool,
        car_title: String,
        color: Color,
        damping: Damping,
        physics: GameTire,
    ) -> Entity {
        let mut tire = commands.spawn(Tire::build(
            player,
            is_front,
            is_right,
            tire_half_size,
            car_title,
            color,
            damping,
            physics,
        ));

        tire.insert(ImpulseJoint::new(car, car_anchor));

        tire.id()
    }

    pub fn spawn_all_for_car(
        commands: &mut Commands,
        player: Player,
        car: Entity,
        car_half_size: Vec2,
        tire_half_size: Vec2,
        color: Color,
        damping: Damping,
        car_title: String,
        car_physics: GameCar,
    ) -> TireEntities {
        TireEntities {
            top_right: Tire::spawn_one_for_car(
                commands,
                player.clone(),
                car,
                Tire::anchor_for_car(car_half_size, tire_half_size, true, true),
                tire_half_size,
                true,
                true,
                car_title.clone(),
                color,
                damping,
                car_physics.tire_top_right,
            ),
            top_left: Tire::spawn_one_for_car(
                commands,
                player.clone(),
                car,
                Tire::anchor_for_car(car_half_size, tire_half_size, true, false),
                tire_half_size,
                true,
                false,
                car_title.clone(),
                color,
                damping,
                car_physics.tire_top_left,
            ),
            bottom_right: Tire::spawn_one_for_car(
                commands,
                player.clone(),
                car,
                Tire::anchor_for_car(car_half_size, tire_half_size, false, true),
                tire_half_size,
                false,
                true,
                car_title.clone(),
                color,
                damping,
                car_physics.tire_bottom_right,
            ),
            bottom_left: Tire::spawn_one_for_car(
                commands,
                player.clone(),
                car,
                Tire::anchor_for_car(car_half_size, tire_half_size, false, false),
                tire_half_size,
                false,
                false,
                car_title.clone(),
                color,
                damping,
                car_physics.tire_bottom_left,
            ),
        }
    }

    pub fn anchor_for_car(
        car_half_size: Vec2,
        tire_half_size: Vec2,
        is_front: bool,
        is_right: bool,
    ) -> FixedJointBuilder {
        let car_anchor = Vec2 {
            x: signed(is_right, car_half_size.x + tire_half_size.x * 3.0),
            y: signed(is_front, car_half_size.y - tire_half_size.y),
        };
        let tire_anchor = Vec2::new(0.0, 0.0);

        FixedJointBuilder::new()
            .local_anchor1(car_anchor)
            .local_anchor2(tire_anchor)
    }

    pub fn position_for_car(
        car_position: Vec3,
        car_half_size: Vec2,
        tire_half_size: Vec2,
        is_front: bool,
        is_right: bool,
    ) -> Vec3 {
        Vec3 {
            x: car_position.x + signed(is_right, car_half_size.x + tire_half_size.x * 3.0),
            y: car_position.y + signed(is_front, car_half_size.y - tire_half_size.y),
            z: 0.0,
        }
    }
}
