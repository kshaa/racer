use bevy::prelude::*;

#[derive(Clone, Debug, Default, Component, Reflect, FromReflect)]
pub struct Player {
    pub handle: usize,
}
