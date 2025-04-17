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
    Frame {
        #[command(subcommand)]
        command: FrameCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum FrameCommand {
    Pull,
    List,
}
