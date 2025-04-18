use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;

#[derive(Debug)]
pub struct GetFramesAction;

#[async_trait::async_trait]
impl BackupManagerAction for GetFramesAction {
    async fn apply(&self, manager: &mut BackupManager) -> eyre::Result<()> {
        assert!(manager.frames.is_none(), "Frames already fetched");

        Ok(())
    }
}
