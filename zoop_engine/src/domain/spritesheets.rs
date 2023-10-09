use bevy::asset::Handle;
use bevy::gltf::Gltf;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SpriteSheets {
    pub car: Handle<Image>,
    pub tire: Handle<Image>,
    pub trace: Handle<Image>,
    pub debug_material: Handle<StandardMaterial>,
    pub building: Handle<Gltf>,
    pub cube: Handle<Mesh>,
}
