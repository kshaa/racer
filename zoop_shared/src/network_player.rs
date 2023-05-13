use crate::player_id::PlayerId;
use serde::{Deserialize, Serialize};

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