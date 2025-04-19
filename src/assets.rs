use crate::auth::get_authenticated_client;
use crate::remote_types::frame::FrameId;
use crate::remote_types::frame_assets_response::FrameAssetsResponse;
use eyre::Context;
use eyre::bail;
use std::path::PathBuf;
use tracing::debug;

pub fn get_path_for_frame_assets(frame_id: &FrameId) -> PathBuf {
    PathBuf::from(format!("aura-frame-assets-{}.json", frame_id))
}

pub async fn pull_assets_for_frame(frame_id: &FrameId) -> eyre::Result<FrameAssetsResponse> {
    debug!("Pulling assets for frame {frame_id} from API");
    let client = get_authenticated_client().await?;
    let url =
        format!("https://api.pushd.com/v5/frames/{frame_id}/assets.json?side_load_users=false");
    let result = client.get(url).send().await?;
    if !result.status().is_success() {
        let error_text = result.text().await?;
        bail!("Frame asset pull failed: {}", error_text);
    }

    let path = get_path_for_frame_assets(frame_id);
    debug!("Writing frame assets to file: {}", path.display());
    let response_json: serde_json::Value = serde_json::from_str(&result.text().await?)?;
    let response_json_pretty = serde_json::to_string_pretty(&response_json)?;
    tokio::fs::write(&path, &response_json_pretty).await?;

    debug!("Parsing frame assets response as known type");
    let result: FrameAssetsResponse = serde_json::from_str(&response_json_pretty)
        .wrap_err(format!("frame id: {frame_id}"))
        .wrap_err(path.display().to_string())?;
    Ok(result)
}

pub async fn read_assets_for_frame(frame_id: &FrameId) -> eyre::Result<FrameAssetsResponse> {
    let path = get_path_for_frame_assets(frame_id);
    debug!("Reading frame assets from file: {}", path.display());
    let file = tokio::fs::read_to_string(&path).await?;
    let result: FrameAssetsResponse = match serde_json::from_str(&file) {
        Ok(x) => eyre::Ok(x),
        Err(e) => {
            let line = e.line();
            let col = e.column();
            Err(e).wrap_err(format!("Path: {path}:{line}:{col}", path = path.display()))?
        }
    }?;
    Ok(result)
}

pub async fn get_assets_for_frame(frame_id: &FrameId) -> eyre::Result<FrameAssetsResponse> {
    let path = get_path_for_frame_assets(frame_id);
    if path.exists() {
        read_assets_for_frame(frame_id).await
    } else {
        pull_assets_for_frame(frame_id).await
    }
}
