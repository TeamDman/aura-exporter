use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;

#[derive(Debug)]
pub struct DiscoverLocalFilesAction;
impl std::fmt::Display for DiscoverLocalFilesAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
#[async_trait::async_trait]
impl BackupManagerAction for DiscoverLocalFilesAction {
    async fn apply(self: Box<Self>, manager: &mut BackupManager) -> eyre::Result<()> {
        assert!(manager.local_backup_structure.local_files.is_none());
        manager.local_backup_structure.discover_local_files().await?;
        Ok(())
    }
}
