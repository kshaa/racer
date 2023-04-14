#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use engine::networked_game;
use shared::{PlayerId, RoomId};
use uuid::{uuid, Uuid};

fn main() {
    tauri::async_runtime::spawn(async move {
        println!("Mic check");
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![join_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn join_game(
    is_main_player: bool,
    player_id_0_json: String,
    player_id_1_json: String,
    room_id_json: String,
) {
    let player_0_text: String = serde_json::from_str(&player_id_0_json).unwrap();
    let player_0 = PlayerId(Uuid::parse_str(&player_0_text).unwrap());
    let player_1_text: String = serde_json::from_str(&player_id_1_json).unwrap();
    let player_1 = PlayerId(Uuid::parse_str(&player_1_text).unwrap());
    let room_id_text: String = serde_json::from_str(&room_id_json).unwrap();
    let room_id = RoomId(Uuid::parse_str(&room_id_text).unwrap());

    println!("Tauri starting game");
    networked_game(is_main_player, player_0, player_1, room_id).await;
}
