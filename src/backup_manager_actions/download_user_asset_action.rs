use crate::asset_downloader::AssetDownloadBuilder;
use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;
use crate::remote_types::file_name::FileName;
use crate::remote_types::user::UserId;
use eyre::bail;
use reqwest::Client;

#[derive(Debug)]
pub struct DownloadUserAssetAction {
    pub user_id: UserId,
    pub file_name: FileName,
    pub client: Client,
}
impl std::fmt::Display for DownloadUserAssetAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "DownloadUserAssetAction - user {} - file name {:?}",
            self.user_id, self.file_name
        ))
    }
}

#[async_trait::async_trait]
impl BackupManagerAction for DownloadUserAssetAction {
    async fn apply(self: Box<Self>, manager: &mut BackupManager) -> eyre::Result<()> {
        let output_file_path = manager
            .local_backup_structure
            .get_path_for_user_asset(&self.user_id, &self.file_name);
        let Some(local_files) = manager.local_backup_structure.local_files.as_mut() else {
            bail!("Local files not discovered yet");
        };

        AssetDownloadBuilder::new()
            .user_id(self.user_id.clone())
            .file_name(self.file_name.clone())
            .output_file_path(output_file_path)
            .build()?
            .run(&self.client)
            .await?;

        local_files
            .entry(self.user_id)
            .or_default()
            .insert(self.file_name);

        Ok(())
    }
}
