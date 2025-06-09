use crate::auth::get_authenticated_client;
use crate::remote_types::frames_response::FramesResponse;
use eyre::bail;
use std::time::Duration;
use tracing::debug;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};

const FRAMES_FILE_NAME: &str = "aura-frames.json";
const OLD_FILES_DIR: &str = "old";

async fn move_stale_files(frames_file_path: &Path) -> eyre::Result<()> {
    let frames_file_path_abs = fs::canonicalize(frames_file_path)
        .map_err(|e| eyre::eyre!("Failed to access frames file at '{}': {}", frames_file_path.display(), e))?;

    let metadata = fs::metadata(&frames_file_path_abs)?;
    let modified_time: DateTime<Utc> = metadata.modified()?.into();
    let old_dir_name = format!("{}/{}", OLD_FILES_DIR, modified_time.format("%Y-%m-%d_%H-%M-%S"));
    let old_dir_path = Path::new(&old_dir_name); // Relative to CWD, e.g., "old/YYYY-MM-DD_HH-MM-SS"

    // Ensure the directory for old files exists.
    // fs::create_dir_all will create it if it doesn't exist,
    // and do nothing if it already exists.
    fs::create_dir_all(old_dir_path)?;

    // Move frames file
    let frames_filename = frames_file_path_abs.file_name()
        .ok_or_else(|| eyre::eyre!("Frames file path has no filename: {}", frames_file_path_abs.display()))?;
    let old_frames_file_path = old_dir_path.join(frames_filename);
    fs::rename(&frames_file_path_abs, &old_frames_file_path)?;
    debug!("Moved {} to {}", frames_file_path_abs.display(), old_frames_file_path.display());

    // Move asset files
    let asset_source_dir = frames_file_path_abs.parent()
        .ok_or_else(|| eyre::eyre!("Absolute frames file path '{}' has no parent", frames_file_path_abs.display()))?;
    debug!("Moving asset files from {}", asset_source_dir.display());
    for entry in fs::read_dir(asset_source_dir)? {
        let entry = entry?;
        let path = entry.path(); // This will be an absolute path if asset_source_dir is absolute
        if path.is_file() {
            if let Some(filename_str) = path.file_name().and_then(|f| f.to_str()) {
                if filename_str.starts_with("aura-frame-assets-") && filename_str.ends_with(".json") {
                    let old_asset_file_path = old_dir_path.join(filename_str);
                    fs::rename(&path, &old_asset_file_path)?;
                    debug!("Moved {} to {}", path.display(), old_asset_file_path.display());
                }
            }
        }
    }

    Ok(())
}

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

pub async fn get_frames(refresh_every: Option<Duration>) -> eyre::Result<FramesResponse> {
    let path = std::path::Path::new(FRAMES_FILE_NAME);
    if path.exists() {
        if let Some(refresh_duration) = refresh_every {
            let metadata = tokio::fs::metadata(FRAMES_FILE_NAME).await?;
            let modified_time = metadata.modified()?;
            let elapsed_time = std::time::SystemTime::now().duration_since(modified_time)?;
            if elapsed_time > refresh_duration {
                debug!("Frames file is older than refresh_every duration, pulling fresh data.");
                move_stale_files(path).await?;
                pull_frames().await
            } else {
                let remaining = refresh_duration.checked_sub(elapsed_time)
                    .unwrap_or_else(|| Duration::from_secs(0));
                tracing::info!(
                    "Using cached frames file. Time remaining until refresh: {:.1} seconds",
                    remaining.as_secs_f64()
                );
                read_frames().await
            }
        } else {
            read_frames().await
        }
    } else {
        pull_frames().await
    }
}
