use eyre::bail;

use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;

use super::get_frame_assets_action::GetFrameAssetsAction;

#[derive(Debug)]
pub struct EnqueueGetFrameAssetActionsAction;
impl std::fmt::Display for EnqueueGetFrameAssetActionsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[async_trait::async_trait]
impl BackupManagerAction for EnqueueGetFrameAssetActionsAction {
    async fn apply(self: Box<Self>, manager: &mut BackupManager) -> eyre::Result<()> {
        let Some(frames) = &manager.frames else {
            bail!("Frames not fetched yet");
        };

        for frame in frames.frames.iter() {
            manager.stack.push_front(Box::new(GetFrameAssetsAction {
                frame_id: frame.id.clone(),
            }));
        }
        Ok(())
    }
}
