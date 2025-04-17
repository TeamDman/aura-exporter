use clap::Parser;
use clap::Subcommand;
use std::fmt::Debug;
use std::path::PathBuf;

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
    #[command(subcommand)]
    Asset(AssetCommand),
}

#[derive(Subcommand, Debug)]
pub enum FrameCommand {
    List,
    #[command(subcommand)]
    Asset(FrameAssetCommand),
}

#[derive(Subcommand, Debug)]
pub enum FrameAssetCommand {
    List {
        #[arg(long)]
        frame_id: String,
    },
    DownloadPicker {
        #[arg(long)]
        save_dir: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum AssetCommand {
    Download {
        #[arg(long)]
        user_id: String,
        #[arg(long)]
        file_name: String,
        #[arg(long)]
        save_dir: PathBuf,
    },
}
