use std::time::Duration;

use ::clap::CommandFactory;
use ::clap::FromArgMatches;
use asset_downloader::AssetDownloadBuilder;
use asset_summary::summarize_assets_for_frame;
use auth::get_authenticated_client;
use auth::login;
use backup_manager::BackupManager;
use clap::AssetCommand;
use clap::BackupCommand;
use clap::Cli;
use clap::Commands;
use clap::FrameAssetCommand;
use clap::FrameCommand;
use frames::get_frames;
use local_backup_structure::LocalBackupStructure;
use remote_types::file_name::FileName;
use remote_types::frame::FrameId;
use remote_types::user::UserId;

pub mod asset_downloader;
pub mod asset_summary;
pub mod assets;
pub mod auth;
pub mod backup_manager;
pub mod backup_manager_actions;
pub mod clap;
pub mod download_picker;
pub mod frames;
pub mod local_backup_structure;
pub mod remote_types;

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
                for frame in get_frames(None).await?.frames {
                    println!("{}: {}", frame.id, frame.name);
                }
            }
            FrameCommand::Asset(asset_command) => match asset_command {
                FrameAssetCommand::List { frame_id } => {
                    let frame_id = FrameId::new(frame_id);
                    summarize_assets_for_frame(&frame_id).await?;
                }
                FrameAssetCommand::DownloadPicker { save_dir } => {
                    download_picker::download_picker(save_dir).await?;
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
                let client = get_authenticated_client().await?;
                let local_backup_structure = LocalBackupStructure::new(save_dir.to_path_buf());
                let output_file_path =
                    local_backup_structure.get_path_for_user_asset(&user_id, &file_name);
                AssetDownloadBuilder::new()
                    .user_id(user_id.clone())
                    .file_name(file_name.clone())
                    .output_file_path(output_file_path)
                    .build()?
                    .run(&client)
                    .await?;
            }
        },
        Commands::Backup(command) => match command {
            BackupCommand::Sync {
                save_dir,
                delay_ms,
                jiggle_ms,
                jiggle_strategy,
                refresh_every,
            } => {
                let delay = Duration::from_millis(delay_ms as u64);
                let jiggle = Duration::from_millis(jiggle_ms as u64);
                let mut backup_manager = BackupManager::new(save_dir, delay, jiggle, jiggle_strategy, refresh_every);
                backup_manager.run().await?;
            }
        },
    }
    Ok(())
}
