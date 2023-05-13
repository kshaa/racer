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
        .invoke_handler(tauri::generate_handler![join_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn join_game(
    is_main_player: bool,
    player_0_uuid: String,
    player_1_uuid: String,
    room_uuid: String,
) -> Result<(), String> {
    let player_id_0 = PlayerId(Uuid::parse_str(&player_0_uuid).unwrap());
    let player_id_1 = PlayerId(Uuid::parse_str(&player_1_uuid).unwrap());
    let room_id = RoomId(Uuid::parse_str(&room_uuid).unwrap());

    exec_join_game(is_main_player, player_id_0, player_id_1, room_id)
}
