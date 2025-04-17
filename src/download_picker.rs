use crate::asset_download::download_asset;
use crate::assets::get_assets_for_frame;
use crate::auth::create_authenticated_client;
use crate::frames::get_frames;
use cloud_terrastodon_core_user_input::prelude::Choice;
use cloud_terrastodon_core_user_input::prelude::FzfArgs;
use cloud_terrastodon_core_user_input::prelude::pick;
use cloud_terrastodon_core_user_input::prelude::pick_many;
use std::path::Path;
use tracing::info;

pub async fn download_picker(save_dir: &Path) -> eyre::Result<()> {
    let frames = get_frames().await?.frames;
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

    let client = create_authenticated_client().await?;
    while let Some(asset) = chosen_assets.pop() {
        download_asset(&client, &asset.user.id, &asset.file_name, save_dir).await?;
        info!("Downloaded asset, {} remain", chosen_assets.len());
    }

    Ok(())
}
