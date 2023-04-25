use crate::error::*;
use crate::room::*;
use actix::*;
use actix_web::Result;
use std::collections::HashMap;
use zoop_shared::RoomId;

/// Game room metadata
#[derive(Debug, Clone)]
pub struct GameLobbyRoomMetadata {
    pub room_id: RoomId,
    pub address: Addr<GameRoom>,
}

/// Game lobby
#[derive(Default)]
pub struct GameLobby {
    pub games: HashMap<RoomId, GameLobbyRoomMetadata>,
}
impl GameLobby {
    pub fn add(&mut self, room_id: RoomId, address: Addr<GameRoom>) -> Result<(), AppError> {
        match self
            .games
            .insert(room_id.clone(), GameLobbyRoomMetadata { room_id, address })
        {
            None => Ok(()),
            Some(room) => Err(AppError::GameAlreadyExists { id: room.room_id }),
        }
    }
}
