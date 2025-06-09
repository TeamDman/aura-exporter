use crate::asset_downloader::AssetDownloadBuilder;
use crate::assets::get_assets_for_frame;
use crate::auth::get_authenticated_client;
use crate::frames::get_frames;
use crate::local_backup_structure::LocalBackupStructure;
use cloud_terrastodon_core_user_input::prelude::Choice;
use cloud_terrastodon_core_user_input::prelude::FzfArgs;
use cloud_terrastodon_core_user_input::prelude::pick;
use cloud_terrastodon_core_user_input::prelude::pick_many;

use std::path::PathBuf;
use tracing::info;

pub async fn download_picker(save_dir: PathBuf) -> eyre::Result<()> {
    let client = get_authenticated_client().await?;
    let frames = get_frames(None).await?.frames;
    let chosen_frame = pick(FzfArgs {
        choices: frames
            .into_iter()
            .map(|frame| Choice {
                key: frame.name.to_string(),
                value: frame,
            })
            .collect(),
        header: Some("Pick the frame to download assets from".to_string()),
        ..Default::default()
    })?;
    let frame_assets = get_assets_for_frame(&chosen_frame.id).await?.assets;
    let mut chosen_assets = pick_many(FzfArgs {
        choices: frame_assets
            .into_iter()
            .map(|asset| Choice {
                key: asset.file_name.to_string(),
                value: asset,
            })
            .collect(),
        header: Some("Pick the assets to download".to_string()),
        ..Default::default()
    })?;

    let local_backup_structure = LocalBackupStructure::new(save_dir.to_path_buf());
    while let Some(asset) = chosen_assets.pop() {
        let output_file_path =
            local_backup_structure.get_path_for_user_asset(&asset.user_id, &asset.file_name);
        AssetDownloadBuilder::new()
            .asset(&asset)
            .output_file_path(output_file_path)
            .build()?
            .run(&client)
            .await?;
        info!("Downloaded asset, {} remain", chosen_assets.len());
    }

    Ok(())
}
