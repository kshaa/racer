use crate::domain::game_config::GameConfig;
use crate::domain::room_config::RoomConfig;
use crate::systems::build_game::build_game;
use bevy::prelude::*;
use url::Url;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use zoop_shared::network_player::NetworkPlayer;
use zoop_shared::player_id::PlayerId;
use zoop_shared::room_config::GameRoomConfig;
use zoop_shared::room_id::RoomId;

#[wasm_bindgen]
pub fn networked_game_raw(
    http_baseurl: String,
    ws_baseurl: String,
    user_uuid: String,
    user_ticket: String,
    room_uuid: String,
    room_config_json: String,
    canvas_selector: Option<String>,
) {
    let player_id = PlayerId(Uuid::parse_str(&user_uuid).unwrap());
    let room_config = serde_json::from_str(&room_config_json).unwrap();
    let room_id = RoomId(Uuid::parse_str(&room_uuid).unwrap());

    networked_game(
        http_baseurl,
        ws_baseurl,
        player_id,
        user_ticket,
        room_id,
        room_config,
        canvas_selector,
    )
}

pub fn networked_game(
    _http_baseurl: String,
    ws_baseurl: String,
    user_id: PlayerId,
    user_ticket: String,
    room_id: RoomId,
    room_config: GameRoomConfig,
    canvas_selector: Option<String>,
) {
    // Define local and remote players
    let my_network_player = NetworkPlayer {
        id: user_id.clone(),
        is_local: true,
        is_spectator: false,
    };
    let mut network_players: Vec<NetworkPlayer> = room_config
        .players
        .into_iter()
        .filter(|p| p.clone() != user_id.clone())
        .map(|p| NetworkPlayer {
            id: p.clone(),
            is_local: false,
            is_spectator: false,
        })
        .collect();
    network_players.push(my_network_player);
    network_players.sort_by_key(|p| p.id.0.to_string().clone());

    // Define network
    let server_address = Url::parse(&ws_baseurl).unwrap();
    let room = room_id;
    let network = Some(RoomConfig {
        server_address,
        room,
        user_id,
        user_ticket,
    });

    // Build game
    let config = GameConfig::default(network, network_players, canvas_selector);
    let mut game = App::new();
    build_game(&mut game, config);

    // Run game
    game.run();
}

pub fn demo_game() {
    // Build game
    let canvas_selector = None; // TODO: Add web support for demo game
    let no_network = None;
    let player1 = NetworkPlayer {
        id: PlayerId::new(),
        is_local: true,
        is_spectator: false,
    };
    let player2 = NetworkPlayer {
        id: PlayerId::new(),
        is_local: true,
        is_spectator: false,
    };
    let players = vec![player1, player2];
    let config = GameConfig::default(no_network, players, canvas_selector);
    let mut game = App::new();
    build_game(&mut game, config);

    // Run game
    game.run();
}
