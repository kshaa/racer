use bevy::prelude::*;
use ggrs::PlayerHandle;
use crate::domain::controls::Controls;

pub fn read_controls(
    _handle: In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
) -> Controls {
    Controls::from_wasd(keyboard_input.as_ref())
}
