use eyre::bail;
use tracing::info;

use crate::auth::create_authenticated_client;

pub async fn pull_frames() -> eyre::Result<()> {
    let client = create_authenticated_client().await?;
    let url = "https://api.pushd.com/v5/frames/";
    let result = client.get(url).send().await?;
    if result.status().is_success() {
        let response_text = result.text().await?;
        info!("Frame list successful!");
        info!("Response: {}", response_text);
    } else {
        let error_text = result.text().await?;
        bail!("Frame list failed: {}", error_text);
    }
    Ok(())
}
