use crate::controls::Controls;
use bevy::prelude::Component;
use ggrs::Config;
use shared::PlayerId;

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = Controls;
    // Docs say this can be left as u8 :shrugs:
    type State = u8;
    type Address = PlayerId;
}

#[derive(Default, Debug, Component, Clone)]
pub struct Player {
    pub handle: usize,
}
