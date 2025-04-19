use crate::remote_types::asset::Asset;
use crate::remote_types::file_name::FileName;
use crate::remote_types::url::Url;
use crate::remote_types::user::UserId;
use eyre::Context;
use eyre::eyre;
use futures_util::StreamExt;
use reqwest::Client;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::File;
use tracing::debug;

#[derive(Default)]
pub struct AssetDownloadBuilder {
    user_id: Option<UserId>,
    file_name: Option<FileName>,
    output_file_path: Option<PathBuf>,
}
impl AssetDownloadBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn asset(&mut self, asset: &Asset) -> &mut Self {
        self.user_id = Some(asset.user.id.clone());
        self.file_name = Some(asset.file_name.clone());
        self
    }

    pub fn user_id(&mut self, user_id: UserId) -> &mut Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn file_name(&mut self, file_name: FileName) -> &mut Self {
        self.file_name = Some(file_name);
        self
    }

    pub fn output_file_path(&mut self, path: impl AsRef<Path>) -> &mut Self {
        self.output_file_path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn build(&self) -> eyre::Result<AssetDownloadPlan> {
        let user_id = self
            .user_id
            .as_ref()
            .ok_or_else(|| eyre!("User ID is required"))?;
        let file_name = self
            .file_name
            .as_ref()
            .ok_or_else(|| eyre!("File name is required"))?;
        let output_file_path = self
            .output_file_path
            .clone()
            .ok_or_else(|| eyre!("Output file path is required"))?;
        let asset_url = Asset::create_download_url(user_id, file_name);
        Ok(AssetDownloadPlan {
            asset_url,
            output_file_path,
        })
    }
}

pub struct AssetDownloadPlan {
    pub asset_url: Url,
    pub output_file_path: PathBuf,
}
impl AssetDownloadPlan {
    pub async fn run(self, client: &Client) -> eyre::Result<File> {
        // Launch the request
        let response = client
            .get(&**self.asset_url)
            .send()
            .await?
            .error_for_status()?;

        // Create the save directory if it doesn't exist
        if let Some(parent) = self.output_file_path.parent() {
            if !matches!(tokio::fs::try_exists(parent).await, Ok(true)) {
                tokio::fs::create_dir_all(parent)
                    .await
                    .wrap_err(eyre!("Failed to create directory: {}", parent.display()))?;
            }
        }

        // Create the destination file
        let mut file = tokio::fs::File::create(&self.output_file_path).await?;

        // Stream the response body to the file
        let mut byte_stream = response.bytes_stream();
        while let Some(item) = byte_stream.next().await {
            tokio::io::copy(&mut item?.as_ref(), &mut file).await?;
        }

        debug!("Downloaded asset to: {}", self.output_file_path.display());
        Ok(file)
    }
}
