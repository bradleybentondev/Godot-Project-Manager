// grab all engine versions from github
// list all engines
// download one and unzip
// save as a downloaded engine

use std::fs;

use crate::directory::config_directory_service::{Directories, DirectoryService};

use super::godot_engine_version::GodotEngineVersion;

pub fn get_installed_godot_versions() -> Vec<GodotEngineVersion> {
    let directory_service = DirectoryService::new();
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

            engine_versions.push(GodotEngineVersion {
                version_name: name.to_string(),
                version_number: name.to_string(),
                updated_at: "2024-08-12".to_string(),
                path,
            })
        }
    }

    engine_versions
}

mod tests {
    use crate::{
        directory::config_directory_service::{Directories, TestDirectoryService},
        fetcher::download_service::download_and_extract_engine,
        godot_service::godot_engine_service::get_installed_godot_versions,
    };
    use std::fs;

    #[tokio::test]
    async fn test_find_godot_version() {
        let directory_service = TestDirectoryService::new();

        let url = "https://github.com/godotengine/godot/releases/download/4.2.1-stable/Godot_v4.2.1-stable_win64.exe.zip".to_string();
        download_and_extract_engine(url, directory_service.engine_storage_path())
            .await
            .unwrap();

        let versions = get_installed_godot_versions();

        assert!(versions.len() == 1);

        fs::remove_dir_all("./test_data").unwrap();
    }
}
