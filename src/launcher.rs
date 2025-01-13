use crate::{
    config::LauncherConfig,
    files::load_file_or_download_serialized,
    launcher_runner::launch_game,
    launcher_version::LauncherVersion,
    model::{
        mojang::{MojangVersionManifest, MojangVersionManifestVersion},
        VersionType,
    },
    platform::PlatformData,
    LauncherPath, LauncherPaths, Result,
};

const VERSION_MANIFEST_V2_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

pub struct Launcher {
    pub platform_data: PlatformData,
    pub launcher_paths: LauncherPaths,
    pub config: LauncherConfig,
    pub version_manifest: Option<MojangVersionManifest>,
    pub selected_manifest_version: Option<MojangVersionManifestVersion>,
    pub selected_launcher_version: Option<LauncherVersion>,
}

impl Launcher {
    pub fn new(
        platform_data: PlatformData,
        launcher_paths: LauncherPaths,
        config: LauncherConfig,
    ) -> Launcher {
        Launcher {
            platform_data,
            launcher_paths,
            config,
            version_manifest: None,
            selected_manifest_version: None,
            selected_launcher_version: None,
        }
    }

    pub async fn load_version_manifest_or_download(&mut self) -> Result<MojangVersionManifest> {
        let path = self.launcher_paths.get_path(LauncherPath::VersionsManifest);
        let download_url = VERSION_MANIFEST_V2_URL.to_string();

        let manifest_file: MojangVersionManifest =
            load_file_or_download_serialized(path, download_url).await?;

        self.version_manifest = Some(manifest_file.clone());

        Ok(manifest_file)
    }

    pub fn list_versions(
        &self,
        version_type: Option<VersionType>,
    ) -> Result<Vec<MojangVersionManifestVersion>> {
        let version_manifest = self.version_manifest_or_err()?;

        let filtered_versions = match version_type {
            None => version_manifest.versions.clone(),
            Some(version_type) => version_manifest.versions_by_type(version_type),
        };

        Ok(filtered_versions)
    }

    pub fn select_version(&mut self, version_id: String) -> Result<MojangVersionManifestVersion> {
        let version = self
            .version_manifest_or_err()?
            .find_version_by_id(version_id.clone())
            .ok_or(crate::Error::VersionNotFound(version_id.clone()))?;

        self.selected_manifest_version = Some(version.clone());
        self.config.last_selected_version_id = Some(version_id.clone());

        Ok(version)
    }

    pub async fn persist_config(&self) -> Result<()> {
        self.config.persist(self.launcher_paths.clone()).await
    }

    pub async fn start_downloads(&mut self) -> Result<()> {
        let selected_manifest_version = self
            .selected_manifest_version
            .as_ref()
            .ok_or(crate::Error::VersionNotSelectedError)?;

        let mut launcher_version = LauncherVersion::new(selected_manifest_version.clone());

        launcher_version
            .start_downloads(&self.launcher_paths, &self.platform_data)
            .await?;

        self.selected_launcher_version = Some(launcher_version);

        Ok(())
    }

    pub async fn launch_game(&self) -> Result<()> {
        launch_game(
            &self.launcher_paths,
            &self.platform_data,
            &self.selected_launcher_version.as_ref().unwrap(),
            &self.config,
        )
        .await?;

        Ok(())
    }

    fn version_manifest_or_err(&self) -> Result<&MojangVersionManifest> {
        self.version_manifest
            .as_ref()
            .ok_or(crate::Error::NoVersionManifestError)
    }
}
