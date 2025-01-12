use serde::{Deserialize, Serialize};

use crate::model::VersionType;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionManifest {
    pub latest: MojangVersionManifestLatest,
    pub versions: Vec<MojangVersionManifestVersion>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionManifestLatest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MojangVersionManifestVersion {
    pub id: String,
    pub r#type: VersionType,
    pub url: String,
    pub time: String,
    pub release_time: String,
}

impl MojangVersionManifest {
    pub fn versions_by_type(&self, version_type: VersionType) -> Vec<MojangVersionManifestVersion> {
        self.versions
            .iter()
            .filter(|version| version.r#type == version_type)
            .cloned()
            .collect()
    }

    pub fn find_version_by_id(&self, id: String) -> Option<MojangVersionManifestVersion> {
        self.versions
            .iter()
            .find(|version| version.id == id)
            .cloned()
    }
}
