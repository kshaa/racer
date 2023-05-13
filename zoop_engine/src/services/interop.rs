use crate::domain::game_config::GameConfig;
use crate::domain::room_config::RoomConfig;
use crate::systems::build_game::build_game;
use bevy::prelude::*;
use url::Url;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use zoop_shared::network_player::NetworkPlayer;
use zoop_shared::player_id::PlayerId;
use zoop_shared::room_id::RoomId;

#[wasm_bindgen]
pub fn networked_game_raw(
    is_main_player: bool,
    player_0_uuid: String,
    player_1_uuid: String,
    room_uuid: String,
    canvas_selector: Option<String>,
) {
    let player_id_0 = PlayerId(Uuid::parse_str(&player_0_uuid).unwrap());
    let player_id_1 = PlayerId(Uuid::parse_str(&player_1_uuid).unwrap());
    let room_id = RoomId(Uuid::parse_str(&room_uuid).unwrap());

    networked_game(
        is_main_player,
        player_id_0,
        player_id_1,
        room_id,
        canvas_selector,
    )
}

pub fn networked_game(
    is_main_player: bool,
    player_id_0: PlayerId,
    player_id_1: PlayerId,
    room_id: RoomId,
    canvas_selector: Option<String>,
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
    let network = RoomConfig {
        server_address,
        room,
    };

    // Build game
    let config = GameConfig::default(network, players, canvas_selector);
    let mut game = App::new();
    build_game(&mut game, config);

    // Run game
    game.run();
}
