use crate::player_id::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerMessage {
    pub address: PlayerId,
    pub message: String, // Bevy JSON message
}
impl PlayerMessage {
    pub fn to(to: PlayerId, message: String) -> PlayerMessage {
        PlayerMessage {
            address: to,
            message,
        }
    }
    pub fn from(from: PlayerId, message: String) -> PlayerMessage {
        PlayerMessage {
            address: from,
            message,
        }
    }
}
