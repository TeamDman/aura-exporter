use crate::auth::get_authenticated_client;
use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;
use crate::backup_manager_actions::download_user_asset_action::DownloadUserAssetAction;
use crate::backup_manager_actions::sleep_action::SleepAction;
use crate::clap::JiggleStrategy;
use crate::remote_types::file_name::FileName;
use crate::remote_types::user::UserId;
use eyre::bail;
use rand_distr::Distribution;
use rand_distr::Normal;
use rand_distr::Uniform;
use std::collections::HashSet;
use std::time::Duration;

#[derive(Debug)]
pub struct DownloadGeneratorAction {
    pub files_not_available_locally: HashSet<(UserId, FileName)>,
    pub max_delay_for_eta_calculation: Duration,
}
impl std::fmt::Display for DownloadGeneratorAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let eta =
            self.max_delay_for_eta_calculation * self.files_not_available_locally.len() as u32;
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
            max_delay_for_eta_calculation: manager.delay + manager.jiggle/2,
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

        // Calculate jiggle and create sleep action
        let mut rng = rand::rng();
        let sleep_duration = {
            let jiggle_ms = manager.jiggle.as_millis() as f64;
            if jiggle_ms > 0.0 {
                match manager.jiggle_strategy {
                    JiggleStrategy::Uniform => {
                        let distr = Uniform::new_inclusive(0.0, jiggle_ms)?;
                        let jiggle = distr.sample(&mut rng) as u64;
                        self.max_delay_for_eta_calculation + Duration::from_millis(jiggle)
                    }
                    JiggleStrategy::Normal => {
                        // using a normal distribution with mean at jiggle_ms / 2 and std_dev of jiggle_ms / 4
                        // Should result in values mostly between 0 and jiggle_ms
                        let mean = jiggle_ms / 2.0;
                        let std_dev = jiggle_ms / 4.0;
                        let normal = Normal::new(mean, std_dev).unwrap();
                        let mut jiggle = normal.sample(&mut rng);
                        // clamp between 0 and jiggle_ms
                        jiggle = jiggle.max(0.0).min(jiggle_ms);

                        self.max_delay_for_eta_calculation + Duration::from_millis(jiggle as u64)
                    }
                }
            } else {
                self.max_delay_for_eta_calculation
            }
        };

        manager.stack.push_back(Box::new(SleepAction {
            duration: sleep_duration,
        }));

        manager.stack.push_back(self);

        Ok(())
    }
}
