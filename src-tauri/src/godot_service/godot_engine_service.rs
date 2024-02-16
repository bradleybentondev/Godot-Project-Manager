// grab all engine versions from github
// list all engines
// download one and unzip
// save as a downloaded engine

use std::fs;

use regex::Regex;

use crate::{
    directory::config_directory_service::ConfigDirectoryService, fetcher::download_service::Asset,
};

use super::godot_engine_version::GodotEngineVersion;

pub fn from_asset(asset: Asset) -> GodotEngineVersion {
    let version_re = Regex::new(r"v[\d+.+]+-").unwrap();

    let mut version_number = asset.name.to_string();
    if let Some(result) = version_re.find(&asset.name) {
        let str = result.as_str();
        version_number = str[1..(str.len() - 1)].to_string();
    }

    if asset.name.contains("_mono_") {
        version_number += " mono"
    }

    let mut name = asset.name;
    name = name.replace(".zip", "");
    name = name.replace(".exe", "");

    GodotEngineVersion {
        version_name: name,
        version_number: version_number,
        updated_at: asset.created_at,
        path: "".to_string(),
        download_url: asset.browser_download_url,
    }
}

pub fn get_installed_godot_versions(
    directory_service: &ConfigDirectoryService,
) -> Vec<GodotEngineVersion> {
    let path = directory_service.engine_storage_path();

    let paths = fs::read_dir(path).unwrap();

    let mut engine_versions: Vec<GodotEngineVersion> = vec![];

    for path in paths {
        let p = path.unwrap().path();

        if p.is_dir() {
            let name = p
                .display()
                .to_string()
                .split("\\")
                .last()
                .unwrap()
                .to_string();

            let path = p.display().to_string();

            engine_versions.push(GodotEngineVersion::new(
                name.to_string(),
                "2024-08-12".to_string(),
                path,
                "".to_string(),
            ))
        }
    }

    engine_versions
}

pub fn remove_installed_version(
    godot_engine_version: &GodotEngineVersion,
    directory_service: &ConfigDirectoryService,
) -> Result<bool, ()> {
    if godot_engine_version.path.is_empty() {
        return Ok(false);
    }

    if !godot_engine_version
        .path
        .contains(&godot_engine_version.version_name)
    {
        return Ok(false);
    }

    fs::remove_dir_all(&godot_engine_version.path).unwrap();

    Ok(true)
}

mod tests {
    use crate::{
        directory::config_directory_service::ConfigDirectoryService,
        fetcher::download_service,
        godot_service::{
            godot_engine_service::get_installed_godot_versions,
            godot_engine_version::GodotEngineVersion,
        },
    };
    use std::fs;

    #[tokio::test]
    async fn test_find_godot_version() {
        let directory_service = ConfigDirectoryService::new_test(
            ".\\test-data-7".to_string(),
            "test1.json".to_string(),
        );

        let engine = GodotEngineVersion::new("Godot_v4.2.1-stable_win64.exe.zip".to_string(), "2024-01-20".to_string(), "".to_string(), 
        "https://github.com/godotengine/godot/releases/download/4.2.1-stable/Godot_v4.2.1-stable_win64.exe.zip".to_string());

        let updated_engine =
            download_service::download_and_extract_engine(&directory_service, &engine)
                .await
                .unwrap();

        assert!(updated_engine.version_name == "Godot_v4.2.1-stable_win64");
        assert!(updated_engine.version_number == "4.2.1");
        assert!(!updated_engine.path.is_empty());

        let versions = get_installed_godot_versions(&directory_service);

        assert!(versions.len() == 1);

        fs::remove_dir_all(".\\test-data-7").unwrap();
    }
}
