use crate::types::asset::Asset;
use crate::types::file_name::FileName;
use crate::types::user::UserId;
use futures_util::StreamExt;
use reqwest::Client;
use std::path::Path;
use tracing::debug;
use tracing::info;

pub async fn download_asset(
    client: &Client,
    user_id: &UserId,
    file_name: &FileName,
    save_dir: &Path,
) -> eyre::Result<()> {
    debug!("Downloading asset: {} {}", user_id, file_name);

    // Prepare the save directory
    let save_dir = save_dir.join(&***user_id).join(&***file_name);
    if let Some(parent) = save_dir.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create the url
    let asset_url = Asset::create_download_url(user_id, file_name);

    // Launch the request
    let response = client
        .get(&*asset_url.inner)
        .send()
        .await?
        .error_for_status()?;

    // Create the destination file
    let mut file = tokio::fs::File::create(&save_dir).await?;

    // Stream the response body to the file
    let mut byte_stream = response.bytes_stream();
    while let Some(item) = byte_stream.next().await {
        tokio::io::copy(&mut item?.as_ref(), &mut file).await?;
    }

    info!("Downloaded asset to: {}", save_dir.display());
    Ok(())
}
