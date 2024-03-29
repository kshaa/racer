use crate::domain::game_state::GameTire;
use crate::domain::player::Player;
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
    collider_scale: ColliderScale,
    locked_axes: LockedAxes,
    restitution: Restitution,
    friction: Friction,
    active_events: ActiveEvents,
    ccd: Ccd,
    collision_groups: CollisionGroups,
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
    pub fn build(
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
            collider_scale: ColliderScale::Absolute(Vec2::new(1., 1.)),
            locked_axes: LockedAxes::default(),
            restitution: Restitution::default(),
            friction: Friction::default(),
            active_events: ActiveEvents::empty(),
            ccd: Ccd::disabled(),
            collision_groups: CollisionGroups::default(),
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
}
