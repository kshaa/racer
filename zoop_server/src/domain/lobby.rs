use crate::actors::room::GameRoom;
use crate::error::*;
use actix::*;
use actix_web::Result;

use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Notify;
use zoop_shared::player_id::PlayerId;
use zoop_shared::room_config::GameRoomConfig;
use zoop_shared::room_id::RoomId;

/// Game room metadata
pub struct GameLobbyRoomMetadata {
    pub room_id: RoomId,
    pub player_count: u32,
    pub created_by: PlayerId,
    pub players: Cell<Vec<PlayerId>>,
    pub address: Cell<Option<Addr<GameRoom>>>,
    pub ready_notification: Arc<Notify>,
}

impl GameLobbyRoomMetadata {
    fn is_ready(&mut self) -> bool {
        self.address.get_mut().clone().is_some()
    }
    fn config(&mut self) -> GameRoomConfig {
        GameRoomConfig {
            players: self.players.get_mut().clone(),
        }
    }
}

/// Game lobby
#[derive(Default)]
pub struct GameLobby {
    pub games: HashMap<RoomId, GameLobbyRoomMetadata>,
}
impl GameLobby {
    pub fn create(
        &mut self,
        room_id: RoomId,
        player_count: u32,
        by: PlayerId,
    ) -> Result<(), AppError> {
        match self.games.insert(
            room_id.clone(),
            GameLobbyRoomMetadata {
                room_id,
                player_count,
                created_by: by.clone(),
                players: Cell::new(vec![by]),
                address: Cell::new(None),
                ready_notification: Arc::new(Notify::new()),
            },
        ) {
            None => Ok(()),
            Some(room) => Err(AppError::GameAlreadyExists { id: room.room_id }),
        }
    }

    pub fn enqueue_player(
        &mut self,
        room_id: RoomId,
        player: PlayerId,
        starter: fn(RoomId, u32) -> Addr<GameRoom>,
    ) -> Result<(), AppError> {
        match self.games.get_mut(&room_id) {
            None => Err(AppError::GameDoesNotExist {
                id: room_id.clone(),
            }),
            Some(room) => {
                let players = room.players.get_mut();
                let address = room.address.get_mut();

                if (players.len() + 1) == (room.player_count as usize) {
                    players.push(player);
                    println!("All players joined, starting room {}", &room.room_id);
                    *address = Some(starter(room.room_id.clone(), room.player_count));
                    room.ready_notification.notify_waiters();

                    Ok(())
                } else if players.len() < (room.player_count as usize) {
                    players.push(player);
                    Ok(())
                } else {
                    Err(AppError::RoomFull())
                }
            }
        }
    }

    // Right(None) - game is ready
    // Right(Some(notify)) - you'll be notified when the game's ready
    // Left - game does not exist
    pub fn ready(&mut self, room_id: RoomId) -> Result<Option<Arc<Notify>>, AppError> {
        match self.games.get_mut(&room_id) {
            None => Err(AppError::GameDoesNotExist {
                id: room_id.clone(),
            }),
            Some(room) => {
                if room.is_ready() {
                    Ok(None)
                } else {
                    Ok(Some(room.ready_notification.clone()))
                }
            }
        }
    }

    pub fn config(&mut self, room_id: RoomId) -> Result<GameRoomConfig, AppError> {
        match self.games.get_mut(&room_id) {
            None => Err(AppError::GameDoesNotExist {
                id: room_id.clone(),
            }),
            Some(room) => {
                if room.is_ready() {
                    Ok(GameRoomConfig {
                        players: room.players.get_mut().clone(),
                    })
                } else {
                    Err(AppError::GameNotReady())
                }
            }
        }
    }
}
