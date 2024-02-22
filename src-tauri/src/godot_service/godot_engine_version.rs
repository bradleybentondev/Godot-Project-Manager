use core::fmt;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GodotEngineVersion {
    #[serde(rename(serialize = "engineName", deserialize = "engineName"))]
    pub version_name: String,
    #[serde(rename(serialize = "engineVersion", deserialize = "engineVersion"))]
    pub version_number: String,
    #[serde(rename(serialize = "updatedAt", deserialize = "updatedAt"))]
    pub updated_at: String,
    #[serde(rename(serialize = "installationPath", deserialize = "installationPath"))]
    pub path: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub download_url: String,
    pub executable_path: String,
    pub console_executable_path: String,
}

impl GodotEngineVersion {
    pub fn new(
        version_name: String,
        updated_at: String,
        path: String,
        download_url: String,
    ) -> GodotEngineVersion {
        let version_re = Regex::new(r"v[\d+.+]+-").unwrap();

        let mut version_number = version_name.to_string();
        if let Some(result) = version_re.find(&version_name) {
            let str = result.as_str();
            version_number = str[1..(str.len() - 1)].to_string();
        }

        if version_name.contains("_mono_") {
            version_number += " mono"
        }

        let mut name = version_name.clone();
        name = name.replace(".zip", "");
        name = name.replace(".exe", "");

        let path_buf = PathBuf::from(&path);

        let mut executable_path = PathBuf::from(path.clone());
        executable_path.push(format!("{}.exe", &name));

        let mut console_executable_path = PathBuf::from(path.clone());
        console_executable_path.push(format!("{}_console.exe", &name));

        if path_buf.exists() && !&executable_path.exists() {
            println!("executable_path does not exist: {:?}", &executable_path);
        }

        if path_buf.exists() && !&console_executable_path.exists() {
            println!(
                "console_executable_path does not exist: {:?}",
                &console_executable_path
            );
        }

        GodotEngineVersion {
            version_name: name,
            version_number: version_number,
            path: path,
            updated_at: updated_at,
            download_url: download_url,
            executable_path: executable_path.to_str().unwrap().to_string(),
            console_executable_path: console_executable_path.to_str().unwrap().to_string(),
        }
    }
}

impl fmt::Debug for GodotEngineVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "version_name: {}, version_number: {}, updated_at: {}, path: {}, download_url: {}",
            self.version_name, self.version_number, self.updated_at, self.path, self.download_url
        )
    }
}
