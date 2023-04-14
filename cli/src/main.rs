use clap::*;
use shared::*;
use url::Url;
use uuid::{uuid, Uuid};

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

    engine::networked_game(
        args.is_main_player,
        PlayerId(uuid!("3b62e456-c3e2-11ed-9410-4f03d41e4c51")),
        PlayerId(uuid!("386dcb76-c3e2-11ed-aaf0-5fc11d6fb44a")),
        RoomId(args.room),
    )
    .await;
}
