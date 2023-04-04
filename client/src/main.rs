use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, PlayerInputs, Session};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(feature = "debug_lines")]
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;
use ggrs::{PlayerType, SessionBuilder};
use shared::*;
use url::Url;
use uuid::{uuid, Uuid};

mod car;
use crate::car::*;
mod tire;
use crate::tire::*;
mod colors;
use crate::colors::*;
mod controls;
use crate::controls::*;
mod math;
use crate::math::*;
mod movement;
use crate::movement::*;
mod config;
use crate::config::*;
mod sync;
use crate::sync::*;
mod websocket;
use crate::websocket::*;
mod game;
use crate::game::*;
use clap::*;

/// Zoop CLI client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Room to join
    #[arg(short, long)]
    room: Uuid,

    /// Joining room as player 1
    #[arg(short, long)]
    is_main_player: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Define players
    let player_0 = NetworkPlayer {
        id: PlayerId(uuid!("3b62e456-c3e2-11ed-9410-4f03d41e4c51")),
        is_local: args.is_main_player,
        is_spectator: false,
    };
    let player_1 = NetworkPlayer {
        id: PlayerId(uuid!("386dcb76-c3e2-11ed-aaf0-5fc11d6fb44a")),
        is_local: !args.is_main_player,
        is_spectator: false,
    };
    let players: Vec<NetworkPlayer> = vec![player_0.clone(), player_1.clone()];

    // Define network
    let server_address = Url::parse("ws://localhost:8080/").unwrap();
    let room = RoomId(args.room);
    let network = NetworkConfig {
        server_address,
        room,
    };

    // Build game
    let config = GameConfig::default(network, players.clone());
    let mut game = App::new();
    build_game(&mut game, config);

    // Run game
    game.run();
}
