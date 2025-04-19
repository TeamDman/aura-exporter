use crate::assets::get_assets_for_frame;
use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;
use crate::remote_types::frame::FrameId;
use tracing::debug;

#[derive(Debug)]
pub struct GetFrameAssetsAction {
    pub frame_id: FrameId,
}
impl std::fmt::Display for GetFrameAssetsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "GetFrameAssetsAction - frame {}",
            self.frame_id
        ))
    }
}

#[async_trait::async_trait]
impl BackupManagerAction for GetFrameAssetsAction {
    async fn apply(self: Box<Self>, manager: &mut BackupManager) -> eyre::Result<()> {
        assert!(!manager.frame_assets.contains_key(&self.frame_id));
        debug!("Getting assets for frame {}", self.frame_id);
        let frame_assets = get_assets_for_frame(&self.frame_id).await?;
        manager
            .frame_assets
            .insert(self.frame_id.clone(), frame_assets);

        Ok(())
    }
}
