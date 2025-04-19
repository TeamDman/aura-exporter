use crate::auth::get_authenticated_client;
use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;
use crate::backup_manager_actions::download_user_asset_action::DownloadUserAssetAction;
use crate::backup_manager_actions::sleep_action::SleepAction;
use crate::remote_types::file_name::FileName;
use crate::remote_types::user::UserId;
use eyre::bail;
use std::collections::HashSet;
use std::time::Duration;

#[derive(Debug)]
pub struct DownloadGeneratorAction {
    pub files_not_available_locally: HashSet<(UserId, FileName)>,
    pub delay: Duration,
}
impl std::fmt::Display for DownloadGeneratorAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let eta = self.delay * self.files_not_available_locally.len() as u32;
        let eta = humantime::format_duration(eta);
        f.write_fmt(format_args!(
            "DownloadGeneratorAction - {} downloads remain, ETA: {}",
            self.files_not_available_locally.len(),
            eta
        ))
    }
}

impl DownloadGeneratorAction {
    pub fn new(manager: &BackupManager) -> eyre::Result<Self> {
        let Some(local_files_by_user) = &manager.local_backup_structure.local_files else {
            bail!("Local files not discovered yet");
        };

        let mut local_files = HashSet::new();
        for (user_id, file_names) in local_files_by_user {
            for file_name in file_names {
                local_files.insert((user_id.clone(), file_name.clone()));
            }
        }

        let mut remote_files: HashSet<(UserId, FileName)> = Default::default();
        for frame in manager.frame_assets.values() {
            for asset in &frame.assets {
                let user_id = asset.user_id.clone();
                let file_name = asset.file_name.clone();
                remote_files.insert((user_id, file_name));
            }
        }
        let mut files_not_available_locally: HashSet<(UserId, FileName)> = Default::default();
        for (user_id, file_name) in remote_files {
            if !local_files.contains(&(user_id.clone(), file_name.clone())) {
                files_not_available_locally.insert((user_id, file_name));
            }
        }

        Ok(Self {
            files_not_available_locally,
            delay: manager.delay,
        })
    }
}

#[async_trait::async_trait]
impl BackupManagerAction for DownloadGeneratorAction {
    async fn apply(mut self: Box<Self>, manager: &mut BackupManager) -> eyre::Result<()> {
        let Some(next_download) = self.files_not_available_locally.iter().next().cloned() else {
            // No more files to download
            return Ok(());
        };
        assert!(self.files_not_available_locally.remove(&next_download));

        let (user_id, file_name) = next_download;
        manager.stack.push_back(Box::new(DownloadUserAssetAction {
            user_id,
            file_name,
            client: get_authenticated_client().await?,
        }));

        manager.stack.push_back(Box::new(SleepAction {
            duration: self.delay,
        }));

        manager.stack.push_back(self);

        Ok(())
    }
}
