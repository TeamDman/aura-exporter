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
    #[command(subcommand)]
    Backup(BackupCommand),
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

#[derive(Subcommand, Debug)]
pub enum BackupCommand {
    Sync {
        /// Root directory of the vault where backup information will be persisted
        #[arg(long)]
        save_dir: PathBuf,

        /// Delay between remote actions, e.g., fetching image assets
        #[arg(long)]
        delay_ms: u32,

        /// Jitter in milliseconds to add to the delay
        #[arg(long, default_value_t = 0)]
        jiggle_ms: u32,

        /// Jiggle distribution strategy (uniform, normal)
        #[arg(long, default_value = "normal")]
        jiggle_strategy: JiggleStrategy,

        /// How often to refresh the frames and assets from the remote API, e.g., 8h, 1d. If not set, data is read from local cache if available.
        #[arg(long, value_parser = humantime::parse_duration)]
        refresh_every: Option<std::time::Duration>,
    }
}

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum JiggleStrategy {
    Uniform,
    Normal,
}
