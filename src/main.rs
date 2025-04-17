use std::collections::HashMap;

use ::clap::CommandFactory;
use ::clap::FromArgMatches;
use auth::login;
use clap::Cli;
use clap::Commands;
use clap::FrameCommand;
use frames::get_frames;
use frames::pull_frames;
use itertools::Itertools;
use types::frame::FrameId;
use types::user_name::UserName;
pub mod assets;
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
        Commands::Frame(command) => match command {
            FrameCommand::List => {
                for frame in get_frames().await?.frames {
                    println!("{}\t{}", frame.id, frame.name);
                }
            }
            FrameCommand::Pull => {
                pull_frames().await?;
            }
            FrameCommand::Asset(asset_command) => match asset_command {
                clap::FrameAssetCommand::Pull { frame_id } => {
                    let frame_id = FrameId::new(frame_id);
                    assets::pull_assets_for_frame(&frame_id).await?;
                }
                clap::FrameAssetCommand::List { frame_id } => {
                    let frame_id = FrameId::new(frame_id);
                    let frame_assets = assets::read_assets_for_frame(&frame_id).await?;
                    let frames = get_frames().await?;
                    let users = frames
                        .frames
                        .iter()
                        .map(|frame| &frame.contributors)
                        .flatten()
                        .map(|user| (user.id.clone(), user))
                        .collect::<HashMap<_, _>>();
                    let asset_users = frame_assets.assets.iter().counts_by(|asset| &asset.user_id);
                    let longest_name_length = users
                        .values()
                        .filter(|user| asset_users.contains_key(&user.id))
                        .map(|user| user.name.to_string().len())
                        .max()
                        .unwrap_or(0);
                    for (user_id, count) in
                        asset_users.into_iter().sorted_by_key(|(_, count)| *count)
                    {
                        let user_display = users
                            .get(user_id)
                            .map(|user| user.name.clone())
                            .unwrap_or(UserName::new("Unknown User"));
                        println!("{} {:>width$}\t{}", user_id, user_display.to_string(), count, width = longest_name_length);
                    }
                }
            },
        },
    }
    Ok(())
}
