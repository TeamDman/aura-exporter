use ::clap::CommandFactory;
use ::clap::FromArgMatches;
use auth::login;
use clap::Cli;
use clap::Commands;
use clap::FrameCommand;
use frames::pull_frames;
use tracing::info;
pub mod auth;
pub mod clap;
pub mod frames;
pub mod types;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut cmd = Cli::command();
    let version = env!("CARGO_PKG_VERSION");
    cmd = cmd.version(version);
    let cli = Cli::from_arg_matches(&cmd.get_matches())?;

    color_eyre::install()?;

    let level = if cli.debug {
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "full");
        }
        // std::env::set_var("RUST_BACKTRACE", "1");
        tracing::level_filters::LevelFilter::DEBUG
    } else {
        tracing::level_filters::LevelFilter::INFO
    };
    let env_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_file(true)
        .with_target(false)
        .with_line_number(true)
        .without_time()
        .init();

    info!("Hello, world!");
    let Some(command) = cli.command else {
        info!("No command provided.");
        return Ok(());
    };
    match command {
        Commands::Login => {
            info!("Login command called.");
            login().await?;
        }
        Commands::Logout => {
            info!("Logout command called.");
        }
        Commands::Frame { command } => match command {
            Some(FrameCommand::List) => {
                info!("Frame list command called.");
                pull_frames().await?;
            }
            None => {
                info!("No frame command provided.");
            }
        },
    }
    Ok(())
}
