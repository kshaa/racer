use std::collections::HashMap;
use crate::player_id::PlayerId;
use serde::{Deserialize, Serialize};

/// Game room config
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameRoomConfig {
    pub players: Vec<PlayerId>
}