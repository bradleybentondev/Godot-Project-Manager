use std::{
    fs,
    io::{Cursor, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

use crate::{
    directory::config_directory_service::ConfigDirectoryService,
    environmnet::is_prod,
    godot_service::{godot_engine_service, godot_engine_version::GodotEngineVersion},
    test_data, Data, DataState,
};

const GITHUB_URL: &str = "https://api.github.com/repos/godotengine/godot/releases";

#[derive(Serialize, Deserialize, Clone)]
pub struct Asset {
    pub browser_download_url: String,
    pub name: String,
    pub created_at: String,
    pub size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Release {
    assets: Vec<Asset>,
}

/// Gets all releases from https://api.github.com/repos/godotengine/godot/releases
///
/// # Panics
///
/// Panics if the body from the response cannot be parsed from json into the Release object.
///
/// # Errors
///
/// This function will return an error if there was an error sending a request to the url.
pub async fn get_available_releases() -> Result<Vec<Release>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body: String = if is_prod() {
        client
            .get(GITHUB_URL)
            .header(USER_AGENT, "My Rust Program 1.0")
            .send()
            .await?
            .text()
            .await?
    } else {
        test_data::TEST_DATA.to_string()
    };

    let value: Vec<Release> = serde_json::from_str(&body).unwrap();

    return Ok(value);
}

pub async fn download_and_extract_engine(
    directory_service: &ConfigDirectoryService,
    godot_engine_version: &GodotEngineVersion,
) -> Result<GodotEngineVersion, Box<dyn std::error::Error>> {
    let godot_engine_path = directory_service.engine_storage_path();
    let engine_name = godot_engine_version.download_url.split("/").last().unwrap();
    let version_path = create_engine_version_path(&godot_engine_path, &engine_name);
    let file_path = create_file_path_from_url_at_path(&version_path, &engine_name);

    fs::File::create(&file_path).unwrap();

    let archive: Vec<u8> = download_url(&godot_engine_version.download_url, &file_path)
        .await
        .unwrap();
    let target_dir = PathBuf::from(&version_path); // Doesn't need to exist

    // The third parameter allows you to strip away toplevel directories.
    // If `archive` contained a single folder, that folder's contents would be extracted instead.
    zip_extract::extract(Cursor::new(archive), &target_dir, true)?;

    Ok(GodotEngineVersion::new(
        godot_engine_version.version_name.clone(),
        godot_engine_version.updated_at.clone(),
        version_path.to_str().unwrap().to_string(),
        godot_engine_version.download_url.clone(),
    ))
}

fn create_engine_version_path(godot_engine_path: &Path, engine_name: &str) -> PathBuf {
    let mut path = PathBuf::from(godot_engine_path);
    let name = &engine_name.replace(".exe", "");
    let name2 = &name.replace(".zip", "");
    let name3 = &name2.replace(" ", "_");
    path.push(name3);
    fs::create_dir_all(&path).unwrap();
    path
}

fn create_file_path_from_url_at_path(folder_path: &Path, url: &str) -> PathBuf {
    let mut file_path = PathBuf::from(folder_path);
    let file_name = url.split("/").last().unwrap();
    file_path.push(file_name);
    file_path
}

async fn download_url(url: &str, file_path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_path)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    let byte_array = content.get_ref().to_vec().try_into().unwrap();

    Ok(byte_array)
}

/// Filters assets by name
pub fn filter_assets_by_name(releases: &Vec<Release>, filter: &str) -> Vec<Asset> {
    let assets: Vec<&Asset> = releases
        .into_iter()
        .map(|release| &release.assets)
        .flatten()
        .collect();

    let filtered: Vec<Asset> = assets
        .into_iter()
        .filter(|asset| asset.name.contains(&filter))
        .map(|asset| asset.clone())
        .collect();

    return filtered;
}

#[cfg(test)]
mod tests {
    use std::{fs, sync::Arc};

    use crate::{
        directory::config_directory_service::ConfigDirectoryService,
        fetcher::{
            download_service::{self, download_and_extract_engine, get_available_releases},
            os_type::OsType,
        },
        godot_service::godot_engine_version::GodotEngineVersion,
    };

    #[tokio::test]
    async fn test_download_engine_version() {
        let directory_service = ConfigDirectoryService::new_test(
            ".\\test-data-6".to_string(),
            "test1.json".to_string(),
        );

        let engine = GodotEngineVersion::new("Godot_v4.2.1-stable_win64.exe.zip".to_string(), "2024-01-20".to_string(), "".to_string(), 
        "https://github.com/godotengine/godot/releases/download/4.2.1-stable/Godot_v4.2.1-stable_win64.exe.zip".to_string());

        let updated_engine = download_and_extract_engine(&directory_service, &engine)
            .await
            .unwrap();

        assert!(updated_engine.version_name == "Godot_v4.2.1-stable_win64");
        assert!(updated_engine.version_number == "4.2.1");
        assert!(!updated_engine.path.is_empty());

        let paths = fs::read_dir(directory_service.engine_storage_path()).unwrap();
        assert!(paths.into_iter().next().is_some());

        fs::remove_dir_all(".\\test-data-6").unwrap();
    }

    #[tokio::test]
    async fn test_releases() {
        let releases = get_available_releases().await.unwrap();

        assert!(releases.len() >= 1)
    }

    #[tokio::test]
    async fn test_filter_assets() {
        let releases = get_available_releases().await.unwrap();
        let filters = vec![
            OsType::Windows64.value(),
            OsType::Windows32.value(),
            OsType::LinuxArm64.value(),
            OsType::LinuxArm32.value(),
            OsType::Linux64.value(),
            OsType::Linux32.value(),
            OsType::Mac.value(),
        ];

        for filter in filters {
            let filtered = download_service::filter_assets_by_name(&releases, &filter);

            println!("testing '{}'", &filter);
            assert!(filtered.len() >= 1);
            assert!(filtered.first().unwrap().name.contains(&filter));
        }
    }
}
