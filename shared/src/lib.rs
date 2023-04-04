use serde::{Serialize, Deserialize};
use uuid::Uuid;
use derive_more::Display;

#[derive(Serialize, Deserialize, Debug, Clone, Display, Eq, PartialEq, Hash)]
pub struct PlayerId(pub Uuid);
impl PlayerId {
    pub fn new() -> PlayerId {
        PlayerId(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct NetworkPlayer {
    pub id: PlayerId,
    pub is_spectator: bool,
    pub is_local: bool,
}
impl NetworkPlayer {
    pub fn remote_player(id: PlayerId) -> NetworkPlayer {
        NetworkPlayer {
            id,
            is_spectator: false,
            is_local: false,
        }
    }
    pub fn local_player(id: PlayerId) -> NetworkPlayer {
        NetworkPlayer {
            id,
            is_spectator: false,
            is_local: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Display, Eq, PartialEq, Hash)]
pub struct RoomId(pub Uuid);
impl RoomId {
    pub fn new() -> RoomId {
        RoomId(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerMessage {
    pub address: PlayerId,
    pub message: String // Bevy JSON message
}
impl PlayerMessage {
    pub fn to(to: PlayerId, message: String) -> PlayerMessage {
        PlayerMessage {
            address: to,
            message
        }
    }
    pub fn from(from: PlayerId, message: String) -> PlayerMessage {
        PlayerMessage {
            address: from,
            message
        }
    }
}
