use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::platform::{PlatformData, PlatformType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeIndex {
    pub linux: HashMap<String, Vec<RuntimeEntry>>,
    #[serde(rename = "linux-i386")]
    pub linux_i386: HashMap<String, Vec<RuntimeEntry>>,
    #[serde(rename = "mac-os")]
    pub mac_os: HashMap<String, Vec<RuntimeEntry>>,
    #[serde(rename = "mac-os-arm64")]
    pub mac_os_arm64: HashMap<String, Vec<RuntimeEntry>>,
    #[serde(rename = "windows-arm64")]
    pub windows_arm64: HashMap<String, Vec<RuntimeEntry>>,
    #[serde(rename = "windows-x64")]
    pub windows_x64: HashMap<String, Vec<RuntimeEntry>>,
    #[serde(rename = "windows-x86")]
    pub windows_x86: HashMap<String, Vec<RuntimeEntry>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeEntry {
    pub availability: RuntimeEntryAvailability,
    pub manifest: RuntimeEntryManifest,
    pub version: RuntimeEntryVersion,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeEntryManifest {
    pub sha1: String,
    pub size: u32,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeEntryAvailability {
    pub group: u32,
    pub progress: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeEntryVersion {
    pub name: String,
    pub released: String,
}

impl RuntimeIndex {
    pub fn select_entry(
        &self,
        platform_data: &PlatformData,
        version_runtime_name: String,
    ) -> Option<RuntimeEntryManifest> {
        let arch = platform_data.arch.clone();
        let entry = match platform_data.platform_type {
            PlatformType::Windows => {
                if arch.contains("86") {
                    &self.windows_x86
                } else if arch == "arm64" {
                    &self.windows_arm64
                } else if arch == "64" {
                    &self.windows_x64
                } else {
                    return None;
                }
            }
            PlatformType::Linux => {
                if arch.contains("i386") {
                    &self.linux_i386
                } else {
                    &self.linux
                }
            }
            PlatformType::MacOs => {
                if arch.contains("arm64") {
                    &self.mac_os_arm64
                } else {
                    &self.mac_os
                }
            }
        };

        let first_runtime_entry = entry.get(version_runtime_name.as_str())?.get(0).cloned()?;

        Some(first_runtime_entry.manifest)
    }
}
