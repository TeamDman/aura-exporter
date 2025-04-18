use crate::backup_manager_actions::init_action::InitAction;
use crate::types::file_name::FileName;
use crate::types::frame::FrameId;
use crate::types::frame_assets_response::FrameAssetsResponse;
use crate::types::frames_response::FramesResponse;
use crate::types::user::UserId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::path::PathBuf;

#[derive(Debug)]
pub struct BackupManager {
    pub save_dir: PathBuf,
    pub delay_ms: u64,
    pub frames: Option<FramesResponse>,
    pub frame_assets: Option<HashMap<FrameId, FrameAssetsResponse>>,
    pub local_files: Option<HashMap<UserId, HashSet<FileName>>>,
    pub stack: VecDeque<Box<dyn BackupManagerAction>>,
}
impl BackupManager {
    pub fn new(save_dir: PathBuf, delay_ms: u64) -> Self {
        Self {
            save_dir,
            delay_ms,
            frames: Default::default(),
            frame_assets: Default::default(),
            local_files: Default::default(),
            stack: {
                let mut stack: VecDeque<Box<dyn BackupManagerAction>> = Default::default();
                stack.push_back(Box::new(InitAction));
                stack
            },
        }
    }
}

#[async_trait::async_trait]
pub trait BackupManagerAction: std::fmt::Debug + Send {
    async fn apply(&self, manager: &mut BackupManager) -> eyre::Result<()>;
}
