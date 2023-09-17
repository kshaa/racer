use bevy::asset::Handle;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SpriteSheets {
    pub car: Handle<Image>,
    pub tire: Handle<Image>,
    pub debug_material: Handle<StandardMaterial>,
    pub cube: Handle<Mesh>,
}
