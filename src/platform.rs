pub struct PlatformData {
    pub platform_type: PlatformType,
    pub arch: String,
}

pub enum PlatformType {
    Windows,
    Linux,
    MacOs,
}

impl PlatformType {
    pub fn native_id(&self) -> String {
        match self {
            PlatformType::Windows => "windows",
            PlatformType::Linux => "linux",
            PlatformType::MacOs => "osx",
        }
        .to_string()
    }
}
