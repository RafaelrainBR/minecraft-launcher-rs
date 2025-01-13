use std::{io::Cursor, path::PathBuf};

use crate::{
    files::{load_file_or_download, load_file_or_download_serialized},
    launcher_paths::LauncherPaths,
    model::mojang::{
        MojangAssetIndexFile, MojangVersion, MojangVersionArtifactKey,
        MojangVersionManifestVersion, RuntimeIndex, RuntimeManifest, VersionLibrary,
    },
    platform::PlatformData,
    LauncherPath, Result,
};

const ASSET_INDEX_OBJECT_DOWNLOAD_BASE_URL: &str = "https://resources.download.minecraft.net";
const JRE_RUNTIMES_URL: &str = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";

#[derive(Debug)]
pub struct LauncherVersion {
    pub manifest_version: MojangVersionManifestVersion,
    pub mojang_version: Option<MojangVersion>,
    pub libraries: Option<Vec<VersionLibrary>>,
    pub asset_index: Option<MojangAssetIndexFile>,
}

impl LauncherVersion {
    pub fn new(manifest_version: MojangVersionManifestVersion) -> LauncherVersion {
        LauncherVersion {
            manifest_version,
            mojang_version: None,
            libraries: None,
            asset_index: None,
        }
    }

    pub async fn start_downloads(
        &mut self,
        launcher_paths: &LauncherPaths,
        platform_data: &PlatformData,
    ) -> Result<()> {
        let mojang_version = self.download_mojang_version(launcher_paths).await?;
        self.download_client_file(launcher_paths, &mojang_version, &self.manifest_version.id)
            .await?;

        self.download_libraries(&mojang_version, launcher_paths, platform_data)
            .await?;

        let asset_index = self
            .download_asset_index(&mojang_version, launcher_paths)
            .await?;
        self.download_asset_objects(&asset_index, launcher_paths)
            .await?;
        self.download_runtime(&launcher_paths, &mojang_version, &platform_data)
            .await?;

        self.extract_natives(launcher_paths).await?;

        Ok(())
    }

    async fn download_mojang_version(
        &mut self,
        launcher_paths: &LauncherPaths,
    ) -> Result<MojangVersion> {
        let version_id = self.manifest_version.id.clone();
        let version_file_path = launcher_paths.build_version_file_path(&version_id);

        let download_url = self.manifest_version.url.clone();
        let mojang_version: MojangVersion =
            load_file_or_download_serialized(version_file_path, download_url).await?;

        self.mojang_version = Some(mojang_version.clone());

        Ok(mojang_version)
    }

    async fn download_client_file(
        &self,
        launcher_paths: &LauncherPaths,
        mojang_version: &MojangVersion,
        version_id: &str,
    ) -> Result<()> {
        let client_file_path = launcher_paths.build_client_file_path(version_id);

        let download_url = mojang_version
            .downloads
            .get(&MojangVersionArtifactKey::Client)
            .ok_or(crate::Error::ClientDownloadNotFound(version_id.to_string()))?
            .url
            .clone();
        let _ = load_file_or_download(client_file_path, download_url).await?;

        Ok(())
    }

    async fn download_libraries(
        &mut self,
        mojang_version: &MojangVersion,
        launcher_paths: &LauncherPaths,
        platform_data: &PlatformData,
    ) -> Result<()> {
        let filtered_libraries = mojang_version.filter_libraries_by_platform_data(platform_data)?;
        self.libraries = Some(filtered_libraries.clone());

        for library in filtered_libraries {
            self.download_library(library, launcher_paths).await?;
        }

        Ok(())
    }

    async fn download_library(
        &self,
        library: VersionLibrary,
        launcher_paths: &LauncherPaths,
    ) -> Result<()> {
        let library_path = launcher_paths.build_library_path(&library.get_path());

        let download_url = library.url.clone();
        let _ = load_file_or_download(library_path, download_url).await?;

        Ok(())
    }

    async fn download_asset_index(
        &mut self,
        mojang_version: &MojangVersion,
        launcher_paths: &LauncherPaths,
    ) -> Result<MojangAssetIndexFile> {
        let asset_index = mojang_version.asset_index.clone();
        let asset_index_path = launcher_paths.build_asset_index_path(&asset_index.id);

        let download_url = asset_index.url.clone();
        let result: MojangAssetIndexFile =
            load_file_or_download_serialized(asset_index_path, download_url).await?;

        self.asset_index = Some(result.clone());

        Ok(result)
    }

    async fn download_asset_objects(
        &self,
        asset_index: &MojangAssetIndexFile,
        launcher_paths: &LauncherPaths,
    ) -> Result<()> {
        for object in asset_index.objects.values() {
            let assets_objects_base_folder =
                launcher_paths.get_path(crate::LauncherPath::AssetsObjects);
            let file_path = object.build_file_path(&assets_objects_base_folder);
            let download_url = object.build_download_url(ASSET_INDEX_OBJECT_DOWNLOAD_BASE_URL);

            let _ = load_file_or_download(file_path, download_url).await?;
        }

        Ok(())
    }

    async fn extract_natives(&self, launcher_paths: &LauncherPaths) -> Result<()> {
        let version_id = self.manifest_version.id.clone();

        let natives_dir = launcher_paths.build_natives_dir_path(&version_id);
        let target_dir = PathBuf::from(&natives_dir);
        tokio::fs::create_dir_all(natives_dir).await?;

        let native_libraries: Vec<VersionLibrary> = self
            .libraries
            .clone()
            .unwrap_or_default()
            .iter()
            .filter(|library| library.is_native())
            .cloned()
            .collect();

        for library in native_libraries {
            let library_path = launcher_paths.build_library_path(&library.get_path());
            let download_url = library.url.clone();

            let library_content = load_file_or_download(library_path, download_url).await?;

            zip_extract::extract(Cursor::new(library_content), &target_dir, true)?;
        }

        Ok(())
    }

    async fn download_runtime(
        &self,
        launcher_paths: &LauncherPaths,
        mojang_version: &MojangVersion,
        platform_data: &PlatformData,
    ) -> Result<()> {
        let index_file_path = launcher_paths.get_path(LauncherPath::RuntimesIndex);
        let download_url = JRE_RUNTIMES_URL.to_string();

        let index: RuntimeIndex =
            load_file_or_download_serialized(index_file_path, download_url).await?;

        let runtime_name = mojang_version.java_version.component.clone();
        let runtime_manifest_url = index
            .select_entry(&platform_data, runtime_name.clone())
            .ok_or(crate::Error::RuntimeNotFound(runtime_name.clone()))?
            .url
            .clone();

        let runtime_manifest_path =
            launcher_paths.build_runtime_manifest_path(&runtime_name.clone());
        let runtime_manifest_content: RuntimeManifest =
            load_file_or_download_serialized(runtime_manifest_path, runtime_manifest_url).await?;

        runtime_manifest_content
            .download(launcher_paths.build_runtime_path(&runtime_name))
            .await?;

        println!("{:?}", runtime_manifest_content);
        Ok(())
    }
}
