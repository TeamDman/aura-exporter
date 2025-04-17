use crate::types::asset::Asset;
use crate::types::file_name::FileName;
use crate::types::url::Url;
use crate::types::user::UserId;
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
    save_dir: Option<PathBuf>,
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

    pub fn save_dir(&mut self, save_dir: impl AsRef<Path>) -> &mut Self {
        self.save_dir = Some(save_dir.as_ref().to_path_buf());
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
        let save_dir = self
            .save_dir
            .as_ref()
            .ok_or_else(|| eyre!("Save directory is required"))?;
        let save_path = save_dir
            .join(format!("user-{user_id}"))
            .join(format!("{file_name}"));

        let asset_url = Asset::create_download_url(user_id, file_name);

        Ok(AssetDownloadPlan {
            asset_url,
            save_path,
        })
    }
}

pub struct AssetDownloadPlan {
    pub asset_url: Url,
    pub save_path: PathBuf,
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
        if let Some(parent) = self.save_path.parent() {
            if !matches!(tokio::fs::try_exists(parent).await, Ok(true)) {
                tokio::fs::create_dir_all(parent)
                    .await
                    .wrap_err(eyre!("Failed to create directory: {}", parent.display()))?;
            }
        }

        // Create the destination file
        let mut file = tokio::fs::File::create(&self.save_path).await?;

        // Stream the response body to the file
        let mut byte_stream = response.bytes_stream();
        while let Some(item) = byte_stream.next().await {
            tokio::io::copy(&mut item?.as_ref(), &mut file).await?;
        }

        debug!("Downloaded asset to: {}", self.save_path.display());
        Ok(file)
    }
}
