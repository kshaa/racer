use crate::domain::game_state::*;

use crate::domain::spritesheets::SpriteSheets;


use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


#[derive(Bundle)]
pub struct Building {
    rigid_body: RigidBody,
    collider: Collider,
    collider_scale: ColliderScale,
    locked_axes: LockedAxes,
    restitution: Restitution,
    friction: Friction,
    active_events: ActiveEvents,
    ccd: Ccd,
    collision_groups: CollisionGroups,
    pbr_bundle: PbrBundle,
}

impl Building {
    pub fn build(spritesheets: &SpriteSheets, building: GameBuilding) -> Building {
        Building {
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(building.half_size, building.half_size),
            collider_scale: ColliderScale::Absolute(Vec2::new(1., 1.)),
            locked_axes: LockedAxes::default(),
            restitution: Restitution::default(),
            friction: Friction::default(),
            active_events: ActiveEvents::empty(),
            ccd: Ccd::disabled(),
            collision_groups: CollisionGroups::default(),
            pbr_bundle: PbrBundle {
                mesh: spritesheets.cube.clone(),
                material: spritesheets.debug_material.clone(),
                transform: Transform::from_translation(building.position)
                    .with_scale(Vec3::ONE * building.half_size * 2.0),
                ..default()
            },
        }
    }
}
