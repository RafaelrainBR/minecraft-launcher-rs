use crate::Result;

#[derive(Debug)]
pub struct PlatformData {
    pub platform_type: PlatformType,
    pub arch: Arch,
}

#[derive(Debug)]
pub enum PlatformType {
    Windows,
    Linux,
    MacOs,
}

#[derive(Debug)]
pub enum Arch {
    X86,
    X86_64,
    Arm,
    Aarch64,
}

impl PlatformData {
    pub fn new() -> Result<Self> {
        let os = std::env::consts::OS;
        let platform_type = match os {
            "windows" => PlatformType::Windows,
            "linux" => PlatformType::Linux,
            "macos" => PlatformType::MacOs,
            _ => return Err(crate::Error::UnsupportedPlatform(os.to_string())),
        };

        let arch = match std::env::consts::ARCH {
            "x86" => Arch::X86,
            "x86_64" => Arch::X86_64,
            "arm" => Arch::Arm,
            "aarch64" => Arch::Aarch64,
            _ => {
                return Err(crate::Error::UnsupportedPlatform(
                    std::env::consts::ARCH.to_string(),
                ))
            }
        };

        Ok(Self {
            platform_type,
            arch,
        })
    }
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
