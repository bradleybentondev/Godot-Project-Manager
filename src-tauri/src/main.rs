// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fetcher::download_service::{download_releases, filter_assets_by_name};
use godot_service::godot_engine_version::GodotEngineVersion;
use serde::{Deserialize, Serialize};

mod directory;
mod environmnet;
mod fetcher;
pub mod godot_service;
mod service;
mod test_data;

#[derive(Serialize, Deserialize)]
struct GodotEngineVersionResponse {
    #[serde(rename(serialize = "installedVersions"))]
    pub installed_godot_versions: Vec<GodotEngineVersion>,
    #[serde(rename(serialize = "allVersions"))]
    pub all_godot_versions: Vec<GodotEngineVersion>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_engine_versions() -> GodotEngineVersionResponse {
    let releases = download_releases().await.unwrap();
    let assets = filter_assets_by_name(&releases, "win64");
    let all_godot_versions: Vec<GodotEngineVersion> = assets
        .into_iter()
        .map(|asset| GodotEngineVersion {
            version_name: asset.name.to_string(),
            version_number: asset.name,
            updated_at: asset.created_at,
            path: "".to_string(),
        })
        .collect();

    GodotEngineVersionResponse {
        installed_godot_versions: vec![],
        all_godot_versions,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_engine_versions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
