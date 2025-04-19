use crate::auth::get_authenticated_client;
use crate::remote_types::frames_response::FramesResponse;
use eyre::bail;
use tracing::debug;

const FRAMES_FILE_NAME: &str = "aura-frames.json";

pub async fn pull_frames() -> eyre::Result<FramesResponse> {
    debug!("Pulling frames from API");
    let client = get_authenticated_client().await?;
    let url = "https://api.pushd.com/v5/frames/";
    let result = client.get(url).send().await?;
    if !result.status().is_success() {
        let error_text = result.text().await?;
        bail!("Frame list failed: {}", error_text);
    }

    debug!("Writing frames to file: {}", FRAMES_FILE_NAME);
    let response_json: serde_json::Value = serde_json::from_str(&result.text().await?)?;
    let response_json_pretty = serde_json::to_string_pretty(&response_json)?;
    tokio::fs::write(FRAMES_FILE_NAME, &response_json_pretty).await?;

    debug!("Parsing frame response as known type");
    let result: FramesResponse = serde_json::from_str(&response_json_pretty)?;
    Ok(result)
}

pub async fn read_frames() -> eyre::Result<FramesResponse> {
    debug!("Reading frames from file: {}", FRAMES_FILE_NAME);
    let file = tokio::fs::read_to_string(FRAMES_FILE_NAME).await?;
    let result: FramesResponse = serde_json::from_str(&file)?;
    Ok(result)
}

pub async fn get_frames() -> eyre::Result<FramesResponse> {
    let path = std::path::Path::new(FRAMES_FILE_NAME);
    if path.exists() {
        read_frames().await
    } else {
        pull_frames().await
    }
}
