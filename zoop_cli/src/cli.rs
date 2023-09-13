use clap::{Parser, Subcommand};
use std::process::Command;
use uuid::Uuid;
use zoop_engine::{demo_game, networked_game};
use zoop_shared::player_id::PlayerId;
use zoop_shared::room_config::GameRoomConfig;
use zoop_shared::room_id::RoomId;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Option<CLICommand>,
}

#[derive(Subcommand)]
pub enum CLICommand {
    DemoGame {},
    ConnectGame {
        #[arg(long)]
        http_baseurl: String,
        #[arg(long)]
        ws_baseurl: String,
        #[arg(long)]
        user_id: Uuid,
        #[arg(long)]
        user_ticket: String,
        #[arg(long)]
        room_id: Uuid,
        #[arg(long)]
        room_config_json: String,
    },
}

pub async fn run_command(command: CLICommand) {
    match command {
        CLICommand::DemoGame {} => demo_game(),
        CLICommand::ConnectGame {
            http_baseurl,
            ws_baseurl,
            user_id,
            user_ticket,
            room_id,
            room_config_json,
        } => {
            let room_config = serde_json::from_str(&room_config_json).unwrap();
            networked_game(
                http_baseurl,
                ws_baseurl,
                PlayerId(user_id),
                user_ticket,
                RoomId(room_id),
                room_config,
                None,
            );
        }
    };
}

#[allow(dead_code)]
pub fn exec_connect_game(
    http_baseurl: String,
    ws_baseurl: String,
    user_id: PlayerId,
    user_ticket: String,
    room_id: RoomId,
    room_config: GameRoomConfig,
) -> Result<(), String> {
    let exe = std::env::current_exe().unwrap();
    let user_uuid = user_id.0.to_string();
    let room_uuid = room_id.0.to_string();
    let room_config_json = serde_json::to_string(&room_config).unwrap();
    let args = vec![
        "connect-game",
        "--http-baseurl",
        &http_baseurl,
        "--ws-baseurl",
        &ws_baseurl,
        "--user-id",
        &user_uuid,
        "--user-ticket",
        &user_ticket,
        "--room-id",
        &room_uuid,
        "--room-config-json",
        &room_config_json,
    ];

    println!("{:?}, {:?}", exe, args);

    Command::new(exe)
        .args(args)
        .output()
        .map(|_| ())
        .map_err(|e| e.to_string())
}
