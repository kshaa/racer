mod cli;
use crate::cli::*;
use clap::*;
use uuid::Uuid;

/// Zoop CLI client
#[tokio::main]
async fn main() {
    let mut cmd = CLI::command();
    let cli = CLI::parse();

    match cli.command {
        None => cmd.print_help().unwrap(),
        Some(c) => run_command(c).await,
    };
}
