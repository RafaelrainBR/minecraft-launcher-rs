use std::path::PathBuf;

use crate::Result;

#[derive(Clone, Debug)]
pub struct LauncherPaths {
    pub base_path: String,
}

impl LauncherPaths {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    pub fn get_path(&self, path: LauncherPath) -> String {
        path.get_path(self.base_path.clone())
    }

    pub fn build_version_file_path(&self, version_id: &str) -> String {
        let versions_dir = self.get_path(LauncherPath::VersionsDir);

        join_paths(
            versions_dir,
            vec![version_id, format!("{}.json", version_id).as_str()],
        )
    }

    pub fn build_client_file_path(&self, version_id: &str) -> String {
        let versions_dir = self.get_path(LauncherPath::VersionsDir);

        join_paths(
            versions_dir,
            vec![version_id, format!("{}.jar", version_id).as_str()],
        )
    }

    pub fn build_library_path(&self, library_path: &str) -> String {
        let libraries_dir = self.get_path(LauncherPath::LibrariesDir);

        join_paths(libraries_dir, vec![library_path])
    }

    pub fn build_asset_index_path(&self, asset_index_id: &str) -> String {
        let assets_index_dir = self.get_path(LauncherPath::AssetsIndex);

        join_paths(
            assets_index_dir,
            vec![format!("{}.json", asset_index_id).as_str()],
        )
    }

    pub fn build_natives_dir_path(&self, version_id: &str) -> String {
        let versions_dir = self.get_path(LauncherPath::VersionsDir);

        join_paths(versions_dir, vec![version_id, "natives"])
    }

    pub fn build_runtime_path(&self, runtime: &str) -> String {
        let runtimes_dir = self.get_path(LauncherPath::RuntimesDir);

        join_paths(runtimes_dir, vec![runtime])
    }

    pub fn build_runtime_manifest_path(&self, runtime: &str) -> String {
        let runtimes_manifest_dir = self.get_path(LauncherPath::RuntimesManifest);

        join_paths(
            runtimes_manifest_dir,
            vec![format!("{}.json", runtime).as_str()],
        )
    }

    pub async fn create_folders(&self) -> Result<()> {
        let paths = vec![
            LauncherPath::VersionsDir,
            LauncherPath::LibrariesDir,
            LauncherPath::AssetsDir,
            LauncherPath::AssetsIndex,
            LauncherPath::AssetsObjects,
            LauncherPath::GameDir,
        ];

        for path in paths {
            let path = path.get_path(self.base_path.clone());
            tokio::fs::create_dir_all(path).await?;
        }

        Ok(())
    }
}

pub enum LauncherPath {
    LauncherConfig,
    VersionsManifest,
    VersionsDir,
    LibrariesDir,
    AssetsDir,
    AssetsIndex,
    AssetsObjects,
    GameDir,
    RuntimesDir,
    RuntimesIndex,
    RuntimesManifest,
}

impl LauncherPath {
    pub fn get_path(&self, base_path: String) -> String {
        let suffix = match self {
            LauncherPath::LauncherConfig => vec!["launcher_config.json"],
            LauncherPath::VersionsManifest => vec!["versions", "version_manifest.json"],
            LauncherPath::VersionsDir => vec!["versions"],
            LauncherPath::LibrariesDir => vec!["libraries"],
            LauncherPath::AssetsDir => vec!["assets"],
            LauncherPath::AssetsIndex => vec!["assets", "indexes"],
            LauncherPath::AssetsObjects => vec!["assets", "objects"],
            LauncherPath::GameDir => vec!["game"],
            LauncherPath::RuntimesDir => vec!["runtimes"],
            LauncherPath::RuntimesIndex => vec!["runtimes", "index.json"],
            LauncherPath::RuntimesManifest => vec!["runtimes", "manifests"],
        };

        join_paths(base_path, suffix)
    }
}

pub fn join_paths(base_path: String, suffix: Vec<&str>) -> String {
    let mut path = PathBuf::from(base_path);
    for part in suffix {
        path.push(part);
    }
    path.to_str().unwrap().to_string()
}
