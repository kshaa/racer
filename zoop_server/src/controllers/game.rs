use crate::actors::player::*;
use crate::actors::room::*;
use crate::domain::error::*;
use crate::domain::lobby::*;
use crate::domain::users::{Ticket, Users};
use actix::*;
use actix_web::web::Data;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::{get, post, Result};
use actix_web::{web, ResponseError};
use actix_web_actors::ws;
use std::sync::Mutex;
use uuid::Uuid;
use zoop_shared::player_id::PlayerId;
use zoop_shared::room_id::RoomId;

#[post("/game/new/by/{player_id}?ticket={ticket}")]
pub async fn game_room_spawn(
    path: web::Path<(PlayerId, Ticket)>,
    users_mutex: Data<Mutex<Users>>,
    lobby_mutex: Data<Mutex<GameLobby>>,
) -> Result<web::Json<RoomId>, AppError> {
    let (player_id, ticket) = path.as_ref();
    let users = users_mutex.lock().unwrap();
    let is_user_with_ticket = users.has(player_id, ticket.clone());

    if !is_user_with_ticket {
        Err(AppError::UserTicketWrong())
    } else {
        let mut lobby = lobby_mutex.lock().unwrap();
        let address = RoomId::new();
        let room = GameRoom::of(address.clone()).start();
        println!("Spawning room {}", &address);
        lobby.add(address.clone(), room).map(|_| web::Json(address))
    }
}

#[get("/game/{room_id}/as/{player_id}?ticket={ticket}")]
pub async fn game_player(
    path: web::Path<(RoomId, PlayerId, Ticket)>,
    users_mutex: Data<Mutex<Users>>,
    lobby_mutex: Data<Mutex<GameLobby>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let (room_id, player_id, ticket) = path.as_ref();
    let users = users_mutex.lock().unwrap();
    let is_user_with_ticket = users.has(player_id, ticket.clone());

    if !is_user_with_ticket {
        Err(actix_web::Error::from(AppError::UserTicketWrong()))
    } else {
        let lobby = lobby_mutex.lock().unwrap();
        let found_meta = lobby.games.get(room_id).map(|m| m.clone());
        match found_meta {
            Some(meta) => {
                let actor = GamePlayer {
                    room_id: room_id.to_owned(),
                    player_id: player_id.clone(),
                    room_address: meta.address.clone(),
                };
                ws::start(actor, &req, stream)
            }
            None => Err(actix_web::Error::from(AppError::GameDoesNotExist {
                id: room_id.clone(),
            })),
        }
    }
}
