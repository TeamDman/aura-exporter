use itertools::Itertools;
use tracing::info;

use crate::backup_manager_actions::init_action::BootstrapAction;
use crate::local_backup_structure::LocalBackupStructure;
use crate::remote_types::frame::FrameId;
use crate::remote_types::frame_assets_response::FrameAssetsResponse;
use crate::remote_types::frames_response::FramesResponse;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Duration;
use crate::clap::JiggleStrategy;

#[derive(Debug)]
pub struct BackupManager {
    pub delay: Duration,
    pub frames: Option<FramesResponse>,
    pub frame_assets: HashMap<FrameId, FrameAssetsResponse>,
    pub local_backup_structure: LocalBackupStructure,
    pub stack: VecDeque<Box<dyn BackupManagerAction>>,
    pub jiggle: Duration,
    pub jiggle_strategy: JiggleStrategy,
}
impl BackupManager {
    pub fn new(save_dir: PathBuf, delay: Duration, jiggle: Duration, jiggle_strategy: JiggleStrategy) -> Self {
        Self {
            delay,
            frames: Default::default(),
            frame_assets: Default::default(),
            local_backup_structure: LocalBackupStructure::new(save_dir),
            stack: {
                let mut stack: VecDeque<Box<dyn BackupManagerAction>> = Default::default();
                stack.push_back(Box::new(BootstrapAction));
                stack
            },
            jiggle,
            jiggle_strategy,
        }
    }
    pub async fn run(&mut self) -> eyre::Result<()> {
        while let Some(action) = self.stack.pop_front() {
            let stack_tail = self.stack.iter().map(|entry| entry.to_string()).join(",");
            info!("Running action {action}, remaining: {stack_tail}");
            action.apply(self).await?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait BackupManagerAction: std::fmt::Debug + std::fmt::Display + Send {
    async fn apply(self: Box<Self>, manager: &mut BackupManager) -> eyre::Result<()>;
}
