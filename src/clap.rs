use clap::Parser;
use clap::Subcommand;
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(name = "aura", about, long_about = None)]
pub struct Cli {
    #[arg(long, global = true, default_value = "false")]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Login,
    Logout,
    #[command(subcommand)]
    Frame(FrameCommand),
}

#[derive(Subcommand, Debug)]
pub enum FrameCommand {
    Pull,
    List,
    #[command(subcommand)]
    Asset(FrameAssetCommand),
}

#[derive(Subcommand, Debug)]
pub enum FrameAssetCommand {
    Pull {
        #[arg(long)]
        frame_id: String,
    },
    List {
        #[arg(long)]
        frame_id: String,
    },
}
