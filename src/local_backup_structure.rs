use crate::remote_types::file_name::FileName;
use crate::remote_types::user::UserId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use tracing::info;
use tracing::warn;

#[derive(Debug)]
pub struct LocalBackupStructure {
    pub root_dir: PathBuf,
    pub local_files: Option<HashMap<UserId, HashSet<FileName>>>,
}
impl LocalBackupStructure {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dir,
            local_files: None,
        }
    }
    pub fn get_path_for_user_asset(&self, user_id: &UserId, file_name: &FileName) -> PathBuf {
        let user_dir = self.root_dir.join("users").join(user_id.to_string());
        let file_path = user_dir.join(file_name.to_string());
        file_path
    }
    pub async fn discover_local_files(&mut self) -> eyre::Result<()> {
        assert!(self.local_files.is_none(), "Local files already discovered");
        let users_dir = self.root_dir.join("users");
        if !users_dir.exists() {
            warn!("Users directory does not exist: {}", users_dir.display());
            self.local_files = Some(HashMap::new());
            return Ok(());
        }
        let mut local_files = HashMap::new();
        let mut user_entries = tokio::fs::read_dir(&users_dir).await?;
        while let Some(user_entry) = user_entries.next_entry().await? {
            let user_id = user_entry
                .file_name()
                .into_string()
                .map_err(|e| eyre::eyre!("Failed to convert user ID to string: {:?}", e))?;
            let user_id: UserId = UserId::new(user_id);
            let user_dir = user_entry.path();
            if !user_dir.is_dir() {
                warn!("User entry is not a directory: {}", user_dir.display());
                continue;
            }
            let mut file_entries = tokio::fs::read_dir(&user_dir).await?;
            let mut file_names = HashSet::new();
            while let Some(file_entry) = file_entries.next_entry().await? {
                let file_name = file_entry
                    .file_name()
                    .into_string()
                    .map_err(|e| eyre::eyre!("Failed to convert file name to string: {:?}", e))?;
                let file_name: FileName = FileName::new(file_name);
                file_names.insert(file_name);
            }
            if file_names.is_empty() {
                warn!("No files found for user: {}", user_id);
            } else {
                info!("Found {} files for user: {}", file_names.len(), user_id);
            }
            local_files.insert(user_id, file_names);
        }
        let total_files: usize = local_files.values().map(|files| files.len()).sum();
        info!("Total files discovered: {}", total_files);
        self.local_files = Some(local_files);

        Ok(())
    }
}
