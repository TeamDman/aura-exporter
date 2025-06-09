use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;
use crate::frames::get_frames;

#[derive(Debug)]
pub struct GetFramesAction;
impl std::fmt::Display for GetFramesAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[async_trait::async_trait]
impl BackupManagerAction for GetFramesAction {
    async fn apply(self: Box<Self>, manager: &mut BackupManager) -> eyre::Result<()> {
        assert!(manager.frames.is_none(), "Frames already fetched");
        
        let frames = get_frames(manager.refresh_every).await?;
        manager.frames = Some(frames);

        Ok(())
    }
}
