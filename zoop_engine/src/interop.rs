use wasm_bindgen::prelude::*;
use bevy::prelude::*;
use zoop_shared::{NetworkPlayer, PlayerId, RoomId};
use url::Url;
use uuid::Uuid;
use crate::config::*;
use crate::game::*;

#[wasm_bindgen]
pub fn networked_game_raw(
    is_main_player: bool,
    player_0_uuid: String,
    player_1_uuid: String,
    room_uuid: String,
) {
    let player_id_0 = PlayerId(Uuid::parse_str(&player_0_uuid).unwrap());
    let player_id_1 = PlayerId(Uuid::parse_str(&player_1_uuid).unwrap());
    let room_id = RoomId(Uuid::parse_str(&room_uuid).unwrap());

    networked_game(
        is_main_player,
        player_id_0,
        player_id_1,
        room_id
    )
}

pub fn networked_game(
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
