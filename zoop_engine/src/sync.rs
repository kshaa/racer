use crate::controls::Controls;
use bevy::prelude::*;
use ggrs::Config;
use zoop_shared::PlayerId;

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = Controls;
    // Docs say this can be left as u8 :shrugs:
    type State = u8;
    type Address = PlayerId;
}

#[derive(Clone, Debug, Default, Component, Reflect, FromReflect)]
pub struct Player {
    pub handle: usize,
}
