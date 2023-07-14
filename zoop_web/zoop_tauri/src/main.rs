#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use clap::Parser;
use uuid::Uuid;
use zoop_cli::*;
use zoop_shared::player_id::PlayerId;
use zoop_shared::room_id::RoomId;

#[tokio::main]
async fn main() {
    let cli = CLI::parse();

    match cli.command {
        None => run_tauri(),
        Some(c) => run_command(c).await,
    };
}

fn run_tauri() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn connect_game(
    http_baseurl: String,
    ws_baseurl: String,
    user_uuid: String,
    user_ticket: String,
    room_uuid: String,
    room_config_json: String,
) -> Result<(), String> {
    let user_id = PlayerId(Uuid::parse_str(&user_uuid).unwrap());
    let room_id = RoomId(Uuid::parse_str(&room_uuid).unwrap());
    let room_config = serde_json::from_str(&room_config_json).unwrap();

    exec_connect_game(http_baseurl, ws_baseurl, user_id, user_ticket, room_id, room_config)
}
