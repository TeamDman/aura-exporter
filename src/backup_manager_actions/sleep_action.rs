use crate::backup_manager::BackupManager;
use crate::backup_manager::BackupManagerAction;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug)]
pub struct SleepAction {
    pub duration: Duration,
}

impl std::fmt::Display for SleepAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "SleepAction - {} seconds",
            self.duration.as_secs()
        ))
    }
}

#[async_trait::async_trait]
impl BackupManagerAction for SleepAction {
    async fn apply(self: Box<Self>, _manager: &mut BackupManager) -> eyre::Result<()> {
        sleep(self.duration).await;
        Ok(())
    }
}
