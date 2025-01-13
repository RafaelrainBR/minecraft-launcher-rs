use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{platform::PlatformData, Result};

use super::{version_library::VersionLibrary, MojangVersionArguments};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersion {
    pub id: String,
    pub assets: String,
    pub asset_index: MojangVersionAssetIndex,
    pub downloads: HashMap<MojangVersionArtifactKey, MojangVersionArtifact>,
    pub java_version: MojangVersionJavaVersion,
    pub main_class: String,
    pub minecraft_arguments: Option<String>,
    pub arguments: Option<MojangVersionArguments>,
    pub libraries: Vec<MojangVersionLibrary>,
    pub logging: Option<HashMap<String, MojangVersionLogging>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionAssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub total_size: u64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MojangVersionArtifactKey {
    Client,
    Server,
    WindowsServer,
    ClientMappings,
    ServerMappings,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionArtifact {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionJavaVersion {
    pub component: String,
    pub major_version: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionLibrary {
    pub name: String,
    pub downloads: MojangVersionLibraryDownloads,
    pub rules: Option<Vec<MojangVersionLibraryRule>>,
    pub natives: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionLibraryDownloads {
    pub artifact: Option<MojangVersionLibraryArtifact>,
    pub classifiers: Option<HashMap<String, MojangVersionLibraryClassifier>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionLibraryRule {
    pub action: String,
    pub os: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionLibraryArtifact {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionLibraryClassifier {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionLogging {
    pub file: MojangVersionLoggingFile,
    pub argument: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionLoggingFile {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

impl MojangVersion {
    pub fn filter_libraries_by_platform_data(
        &self,
        platform_data: &PlatformData,
    ) -> Result<Vec<VersionLibrary>> {
        self.libraries
            .iter()
            .filter(|library| {
                if let Some(rules) = library.rules.clone() {
                    rules.iter().any(|rule| rule.is_allowed(platform_data))
                } else {
                    true
                }
            })
            .map(|library| library.into_version_library(platform_data))
            .collect()
    }
}

impl MojangVersionLibrary {
    pub fn into_version_library(&self, platform_data: &PlatformData) -> Result<VersionLibrary> {
        let native_id = platform_data.platform_type.native_id();

        let arch_as_string = match &platform_data.arch {
            crate::platform::Arch::X86 => "32",
            crate::platform::Arch::X86_64 => "64",
            _ => "",
        };

        let native_classifier_name = match self.natives.clone() {
            Some(natives) => natives
                .get(&native_id)
                .map(|classifier| classifier.replace("${arch}", &arch_as_string)),
            None => None,
        };

        match native_classifier_name {
            Some(native_classifier_name) => {
                if let Some(classifiers) = self.downloads.classifiers.clone() {
                    let classifier = classifiers
                        .get(&native_classifier_name)
                        .ok_or(crate::Error::LibraryDownloadNotFound(self.name.clone()))?;

                    Ok(VersionLibrary::new(
                        classifier.sha1.clone(),
                        classifier.size,
                        classifier.url.clone(),
                        true,
                    ))
                } else {
                    Err(crate::Error::LibraryDownloadNotFound(self.name.clone()))
                }
            }
            None => {
                let artifact = self
                    .downloads
                    .artifact
                    .clone()
                    .ok_or(crate::Error::LibraryDownloadNotFound(self.name.clone()))?;

                Ok(VersionLibrary::new(
                    artifact.sha1.clone(),
                    artifact.size,
                    artifact.url.clone(),
                    false,
                ))
            }
        }
    }
}

impl MojangVersionLibraryRule {
    fn is_allowed(&self, platform_data: &PlatformData) -> bool {
        if self.action == "allow" {
            if let Some(os) = &self.os {
                if let Some(allowed_os) = os.get("name") {
                    return allowed_os == &platform_data.platform_type.native_id();
                }
            } else {
                return true;
            }
        }

        false
    }
}
