use std::process::Command;
use zoop_shared::{PlayerId, RoomId};
use uuid::Uuid;
use clap::{Parser, Subcommand};
use zoop_engine::networked_game;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Option<CLICommand>,
}

#[derive(Subcommand)]
pub enum CLICommand {
    JoinGame {
        #[arg(long)]
        is_main_player: bool,
        #[arg(long)]
        player_id_0: Uuid,
        #[arg(long)]
        player_id_1: Uuid,
        #[arg(long)]
        room_id: Uuid,
    },
}

pub async fn run_command(command: CLICommand) {
    match command {
        CLICommand::JoinGame { is_main_player, player_id_0, player_id_1, room_id } => {
            networked_game(
                is_main_player,
                PlayerId(player_id_0),
                PlayerId(player_id_1),
                RoomId(room_id));
        },
    };
}

pub fn exec_join_game(
    is_main_player: bool,
    player_id_0: PlayerId,
    player_id_1: PlayerId,
    room_id: RoomId,
) -> Result<(), String> {
    let exe = std::env::current_exe().unwrap();
    let p0 = player_id_0.0.to_string();
    let p1 = player_id_1.0.to_string();
    let r = room_id.0.to_string();
    let main_flag = String::from("--is-main-player");
    let mut args = vec!(
        "join-game",
        "--player-id-0",
        &p0,
        "--player-id-1",
        &p1,
        "--room-id",
        &r);

    if is_main_player {
        args.push(&main_flag)
    }

    println!("{:?}, {:?}", exe, args);

    Command::new(exe)
        .args(args)
        .output()
        .map(|_| ())
        .map_err(|e| e.to_string())
}