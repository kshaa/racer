use actix::*;
use actix_files::Files;
use actix_web::web;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::{get, Result};
use actix_web_actors::ws;
use std::sync::Mutex;
use uuid::{uuid, Uuid};
use zoop_shared::*;
mod error;
use crate::error::*;
mod lobby;
use crate::lobby::*;
mod room;
use crate::room::*;
mod player;
use crate::player::*;
mod misc;

#[get("/game/new")]
async fn game_room_spawn(
    lobby_mutex: Data<Mutex<GameLobby>>,
) -> Result<web::Json<RoomId>, AppError> {
    let mut lobby = lobby_mutex.lock().unwrap();
    let address = RoomId::new();
    let room = GameRoom::of(address.clone()).start();
    println!("Spawning room {}", &address);
    lobby.add(address.clone(), room).map(|_| web::Json(address))
}

#[get("/game/{room_id}/as/{player_id}")]
async fn game_player(
    path: web::Path<(RoomId, Uuid)>,
    lobby_mutex: Data<Mutex<GameLobby>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let (room_id, player_id) = path.as_ref();
    let lobby = lobby_mutex.lock().unwrap();
    let found_meta = lobby.games.get(room_id).map(|m| m.clone());
    match found_meta {
        Some(meta) => {
            let actor = GamePlayer {
                room_id: room_id.to_owned(),
                player_id: PlayerId(player_id.to_owned()),
                room_address: meta.address.clone(),
            };
            ws::start(actor, &req, stream)
        }
        None => Err(actix_web::Error::from(AppError::GameDoesNotExist {
            id: room_id.clone(),
        })),
    }
}

/// Start server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let lobby = Data::new(Mutex::new(GameLobby::default()));

    println!("Starting game server");
    HttpServer::new(move || {
        App::new()
            .app_data(lobby.clone())
            .service(game_room_spawn)
            .service(game_player)
            .service(Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
