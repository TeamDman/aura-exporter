use ::clap::CommandFactory;
use ::clap::FromArgMatches;
use auth::login;
use clap::Cli;
use clap::Commands;
use clap::FrameCommand;
use frames::get_frames;
use frames::pull_frames;
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

    match cli.command {
        Commands::Login => {
            login().await?;
        }
        Commands::Logout => {}
        Commands::Frame { command } => match command {
            FrameCommand::List => {
                for frame in get_frames().await?.frames {
                    println!("{}", frame.name);
                }
            }
            FrameCommand::Pull => {
                pull_frames().await?;
            }
        },
    }
    Ok(())
}
