use super::download_generator_action::DownloadGeneratorAction;
use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;

#[derive(Debug)]
pub struct EnqueueDownloadGeneratorActionAction;
impl std::fmt::Display for EnqueueDownloadGeneratorActionAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[async_trait::async_trait]
impl BackupManagerAction for EnqueueDownloadGeneratorActionAction {
    async fn apply(self: Box<Self>, manager: &mut BackupManager) -> eyre::Result<()> {
        manager
            .stack
            .push_back(Box::new(DownloadGeneratorAction::new(manager)?));
        Ok(())
    }
}
