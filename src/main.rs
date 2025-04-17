use ::clap::CommandFactory;
use ::clap::FromArgMatches;
use asset_downloader::AssetDownloadBuilder;
use asset_summary::summarize_assets_for_frame;
use auth::create_authenticated_client;
use auth::login;
use clap::AssetCommand;
use clap::Cli;
use clap::Commands;
use clap::FrameAssetCommand;
use clap::FrameCommand;
use frames::get_frames;
use types::file_name::FileName;
use types::frame::FrameId;
use types::user::UserId;

pub mod asset_downloader;
pub mod asset_summary;
pub mod assets;
pub mod auth;
pub mod clap;
pub mod download_picker;
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
            FrameCommand::Asset(asset_command) => match asset_command {
                FrameAssetCommand::List { frame_id } => {
                    let frame_id = FrameId::new(frame_id);
                    summarize_assets_for_frame(&frame_id).await?;
                }
                FrameAssetCommand::DownloadPicker { save_dir } => {
                    download_picker::download_picker(&save_dir).await?;
                }
            },
        },
        Commands::Asset(command) => match command {
            AssetCommand::Download {
                user_id,
                file_name,
                save_dir,
            } => {
                let user_id = UserId::new(user_id);
                let file_name = FileName::new(file_name);
                let client = create_authenticated_client().await?;
                AssetDownloadBuilder::new()
                    .user_id(user_id.clone())
                    .file_name(file_name.clone())
                    .save_dir(save_dir.clone())
                    .build()?
                    .run(&client)
                    .await?;
            }
        },
    }
    Ok(())
}
