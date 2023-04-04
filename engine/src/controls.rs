use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;

const INPUT_ACCELERATE: u64 = 1 << 0;
const INPUT_REVERSE: u64 = 1 << 1;
const INPUT_BREAK: u64 = 1 << 2;
const INPUT_STEER_RIGHT: u64 = 1 << 3;
const INPUT_STEER_LEFT: u64 = 1 << 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
pub struct Controls {
    pub input: u64,
}
impl Controls {
    pub fn accelerating(&self) -> bool {
        (self.input & INPUT_ACCELERATE) != 0
    }
    pub fn reversing(&self) -> bool {
        (self.input & INPUT_REVERSE) != 0
    }
    pub fn breaking(&self) -> bool {
        (self.input & INPUT_BREAK) != 0
    }
    pub fn steering_right(&self) -> bool {
        (self.input & INPUT_STEER_RIGHT) != 0
    }
    pub fn steering_left(&self) -> bool {
        (self.input & INPUT_STEER_LEFT) != 0
    }

    pub fn none() -> Controls {
        Controls::zeroed()
    }

    pub fn steering_any(&self) -> bool {
        self.steering_right() || self.steering_left()
    }

    pub fn from_keys(
        input: &Input<KeyCode>,
        accelerator: KeyCode,
        reverser: KeyCode,
        breaker: KeyCode,
        steer_right: KeyCode,
        steer_left: KeyCode,
    ) -> Controls {
        let mut serialized: u64 = 0;

        if input.pressed(accelerator) {
            serialized |= INPUT_ACCELERATE
        }
        if input.pressed(reverser) {
            serialized |= INPUT_REVERSE
        }
        if input.pressed(breaker) {
            serialized |= INPUT_BREAK
        }
        if input.pressed(steer_right) {
            serialized |= INPUT_STEER_RIGHT
        }
        if input.pressed(steer_left) {
            serialized |= INPUT_STEER_LEFT
        }

        Controls { input: serialized }
    }

    pub fn from_wasd(input: &Input<KeyCode>) -> Controls {
        Controls::from_keys(
            input,
            KeyCode::W,
            KeyCode::S,
            KeyCode::C,
            KeyCode::D,
            KeyCode::A,
        )
    }

    pub fn from_ijkl(input: &Input<KeyCode>) -> Controls {
        Controls::from_keys(
            input,
            KeyCode::I,
            KeyCode::K,
            KeyCode::N,
            KeyCode::L,
            KeyCode::J,
        )
    }
}

pub fn synchronized_input(
    _handle: In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
) -> Controls {
    Controls::from_wasd(keyboard_input.as_ref())
}
