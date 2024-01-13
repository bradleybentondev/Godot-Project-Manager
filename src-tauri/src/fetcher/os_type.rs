pub enum OsType {
    Windows64,
    Windows32,
    LinuxArm32,
    LinuxArm64,
    Linux64,
    Linux32,
    Mac
}

impl OsType {
    pub fn all(&self) -> String {
        match *self {
            OsType::Windows64 => "win64".to_string(),
            OsType::Windows32 => "win32".to_string(),
            OsType::LinuxArm32 => "linux.arm32".to_string(),
            OsType::LinuxArm64 => "linux.arm64".to_string(),
            OsType::Linux64 => "linux.x86_64".to_string(),
            OsType::Linux32 => "linux.x86_32".to_string(),
            OsType::Mac => "macos.universal".to_string(),
        }
    }

    pub fn value(&self) -> String {
        match *self {
            OsType::Windows64 => "win64".to_string(),
            OsType::Windows32 => "win32".to_string(),
            OsType::LinuxArm32 => "linux.arm32".to_string(),
            OsType::LinuxArm64 => "linux.arm64".to_string(),
            OsType::Linux64 => "linux.x86_64".to_string(),
            OsType::Linux32 => "linux.x86_32".to_string(),
            OsType::Mac => "macos.universal".to_string(),
        }
    }
}