use serde::{Deserialize, Serialize};

use crate::{
    files::{load_serialized_file, save_serialized_file},
    launcher_paths::{LauncherPath, LauncherPaths},
    Result,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherConfig {
    pub last_selected_version_id: Option<String>,
    pub user_name: Option<String>,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            last_selected_version_id: Default::default(),
            user_name: Some("Player".to_string()),
        }
    }
}

impl LauncherConfig {
    pub async fn load(launcher_paths: LauncherPaths) -> Result<Option<Self>> {
        let path = launcher_paths.get_path(LauncherPath::LauncherConfig);

        let config = load_serialized_file(path).await?;

        Ok(config)
    }

    pub async fn persist(&self, launcher_paths: LauncherPaths) -> Result<()> {
        let path = launcher_paths.get_path(LauncherPath::LauncherConfig);

        save_serialized_file(path, self).await?;
        Ok(())
    }
}
