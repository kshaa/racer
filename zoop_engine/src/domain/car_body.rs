use crate::domain::game_state::*;
use bevy::core::Name;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::domain::player::Player;

#[derive(Component)]
pub struct CarMeta {
    pub half_size: Vec2,
}

#[derive(Bundle)]
pub struct CarBody {
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
    collider_scale: ColliderScale,
    locked_axes: LockedAxes,
    restitution: Restitution,
    friction: Friction,
    active_events: ActiveEvents,
    ccd: Ccd,
    collision_groups: CollisionGroups,
    sprite: SpriteBundle,
    player: Player,
}

impl CarBody {
    pub fn build(
        car_title: String,
        half_size: Vec2,
        player: Player,
        color: Color,
        physics: EntityPhysics,
    ) -> CarBody {
        CarBody {
            meta: CarMeta { half_size },
            rigid_body: RigidBody::Dynamic,
            name: Name::new(car_title),
            rigid_mass: physics.mass,
            // mass: AdditionalMassProperties::Mass(0.001),
            sleep: Sleeping::disabled(),
            force: physics.force,
            impulse: physics.impulse,
            collider: Collider::cuboid(half_size.x, half_size.y),
            collider_scale: ColliderScale::Absolute(Vec2::new(1., 1.)),
            locked_axes: LockedAxes::default(),
            restitution: Restitution::default(),
            friction: Friction::default(),
            active_events: ActiveEvents::empty(),
            ccd: Ccd::disabled(),
            collision_groups: CollisionGroups::default(),
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
}
