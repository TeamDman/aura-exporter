use super::discover_local_files_action::DiscoverLocalFilesAction;
use super::enqueue_download_generator_action_action::EnqueueDownloadGeneratorActionAction;
use super::enqueue_get_all_frame_assets_action::EnqueueGetFrameAssetActionsAction;
use super::get_frames_action::GetFramesAction;
use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;

#[derive(Debug)]
pub struct BootstrapAction;

impl std::fmt::Display for BootstrapAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[async_trait::async_trait]
impl BackupManagerAction for BootstrapAction {
    async fn apply(self: Box<Self>, manager: &mut BackupManager) -> eyre::Result<()> {
        // Goal: make sure we have a copy of each picture we can read from Aura remote state
        // pictures are assets that belong to users
        // assets are pictures that belong to frames
        if manager.local_backup_structure.local_files.is_none() {
            manager.stack.push_back(Box::new(DiscoverLocalFilesAction));
        }
        if manager.frames.is_none() {
            manager.stack.push_back(Box::new(GetFramesAction));
            manager
                .stack
                .push_back(Box::new(EnqueueGetFrameAssetActionsAction));
        }
        manager
            .stack
            .push_back(Box::new(EnqueueDownloadGeneratorActionAction));

        Ok(())
    }
}
