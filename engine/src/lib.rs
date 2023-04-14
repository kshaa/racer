use bevy::prelude::*;
use shared::{NetworkPlayer, PlayerId, RoomId};
use url::Url;

mod car;
mod colors;
mod config;
mod controls;
mod math;
mod movement;
mod tire;
use crate::config::*;
mod game;
mod sync;
mod websocket;
use crate::game::*;

pub async fn networked_game(
    is_main_player: bool,
    player_id_0: PlayerId,
    player_id_1: PlayerId,
    room_id: RoomId,
) {
    // Define players
    let player_0 = NetworkPlayer {
        id: player_id_0,
        is_local: is_main_player,
        is_spectator: false,
    };
    let player_1 = NetworkPlayer {
        id: player_id_1,
        is_local: !is_main_player,
        is_spectator: false,
    };
    let players: Vec<NetworkPlayer> = vec![player_0.clone(), player_1.clone()];

    // Define network
    let server_address = Url::parse("ws://localhost:8080/").unwrap();
    let room = room_id;
    let network = NetworkConfig {
        server_address,
        room,
    };

    // Build game
    let config = GameConfig::default(network, players.clone());
    let mut game = App::new();
    build_game(&mut game, config);

    // Run game
    println!("Running game");
    game.run();
}
