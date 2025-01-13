use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{files::load_file_or_download, launcher_paths::join_paths, Result};

type FileName = String;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuntimeManifest {
    pub files: HashMap<FileName, RuntimeManifestFile>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuntimeManifestFile {
    pub r#type: FileType,
    pub downloads: Option<RuntimeManifestFileDownloads>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum FileType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "directory")]
    Directory,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuntimeManifestFileDownloads {
    pub lzma: Option<RuntimeManifestFileDownloadEntry>,
    pub raw: RuntimeManifestFileDownloadEntry,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuntimeManifestFileDownloadEntry {
    pub sha1: String,
    pub size: u32,
    pub url: String,
}

impl RuntimeManifest {
    pub async fn download(&self, base_path: String) -> Result<()> {
        for (file_name, file) in self.files.iter() {
            match file.r#type {
                FileType::File => {
                    let download_entry = file.downloads.as_ref().unwrap().raw.clone();
                    let file_path = self.join_runtime_paths(base_path.clone(), file_name.clone());
                    load_file_or_download(file_path, download_entry.url).await?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn join_runtime_paths(&self, base_path: String, file_name: String) -> String {
        if file_name.contains("/") {
            let path_parts = file_name.split("/").collect::<Vec<&str>>();

            join_paths(base_path, path_parts)
        } else {
            join_paths(base_path, vec![&file_name])
        }
    }
}
