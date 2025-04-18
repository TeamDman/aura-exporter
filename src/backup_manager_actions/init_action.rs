use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;

use super::get_frames_action::GetFramesAction;

#[derive(Debug)]
pub struct InitAction;

#[async_trait::async_trait]
impl BackupManagerAction for InitAction {
    async fn apply(&self, manager: &mut BackupManager) -> eyre::Result<()> {
        if manager.frames.is_none() {
            manager.stack.push_front(Box::new(GetFramesAction));
        }
        Ok(())
    }
}
