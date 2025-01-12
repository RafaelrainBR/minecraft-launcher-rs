use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MojangAssetIndexFile {
    pub objects: HashMap<String, MojangAssetIndexFileObject>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MojangAssetIndexFileObject {
    pub hash: String,
    pub size: u64,
}

impl MojangAssetIndexFileObject {
    pub fn build_download_url(&self, base_url: &str) -> String {
        let hash_prefix = &self.hash[0..2];
        format!("{}/{}/{}", base_url, hash_prefix, self.hash)
    }

    pub fn build_file_path(&self, base_path: &str) -> String {
        let hash_prefix = &self.hash[0..2];
        format!("{}/{}/{}", base_path, hash_prefix, self.hash)
    }
}
