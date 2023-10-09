use bevy::gltf::{Gltf, GltfMesh, GltfPrimitive};
use crate::domain::game_state::*;
use crate::domain::spritesheets::SpriteSheets;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::logic::math::deg2rad;


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
    transform: Transform,
    global_transform: GlobalTransform
}

impl Building {
    pub fn build(building: &GameBuilding) -> Building {
        let mut transform = building.transform();

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
            transform,
            global_transform: Default::default(),
        }
    }

    pub fn build_mesh(
        spritesheets: &SpriteSheets,
        gltfs: &Assets<Gltf>,
        gltf_meshes: &Assets<GltfMesh>,
        building: &GameBuilding,
        story: u32
    ) -> PbrBundle {
        let gltf = gltfs.get(&spritesheets.building).unwrap();
        let gltf_mesh = gltf_meshes.get(gltf.meshes.get(0).unwrap()).unwrap();
        let gltf_primitive = gltf_mesh.primitives.get(0).unwrap();

        let mut transform = building.transform();
        transform.translation = transform.translation + Vec3::new(0.0, 0.0, building.half_size * 2.0 * (story as f32));
        transform.rotate_x(deg2rad(-90.0));

        PbrBundle {
            mesh: gltf_primitive.mesh.clone(),
            material: gltf_primitive.material.clone().unwrap().clone(),
            transform,
            ..default()
        }
    }
}
