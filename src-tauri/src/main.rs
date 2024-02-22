// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use chrono::{DateTime, Local};
use directory::config_directory_service::{self, ConfigDirectoryService};
use fetcher::download_service::{self, filter_assets_by_name};
use godot_service::{godot_engine_service, godot_engine_version::GodotEngineVersion};
use project::{
    project_data::ProjectData,
    project_service::{self, ProjectDirectoryService},
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

mod command;
mod directory;
mod environmnet;
mod fetcher;
mod godot_service;
mod project;
mod test_data;

pub struct DataState(Mutex<Data>, Mutex<HashMap<String, usize>>);

pub struct Data {
    all_godot_engine_versions: Vec<GodotEngineVersion>,
    installed_godot_engine_versions: Vec<GodotEngineVersion>,
    projects: Vec<ProjectData>,
}

#[derive(Serialize, Deserialize)]
struct GodotEngineVersionResponse {
    #[serde(rename(serialize = "installedVersions"))]
    pub installed_godot_versions: Vec<GodotEngineVersion>,
    #[serde(rename(serialize = "allVersions"))]
    pub all_godot_versions: Vec<GodotEngineVersion>,
}

#[tauri::command]
async fn get_engine_versions(
    state: tauri::State<'_, DataState>,
) -> Result<GodotEngineVersionResponse, ()> {
    let releases = download_service::get_available_releases().await.unwrap();
    let assets = filter_assets_by_name(&releases, "win64");
    let all_godot_versions: Vec<GodotEngineVersion> = assets
        .into_iter()
        .map(|asset| godot_engine_service::from_asset(asset))
        .collect();

    let mut state_guard = state.0.lock().await;
    *state_guard = Data {
        all_godot_engine_versions: all_godot_versions.clone(),
        installed_godot_engine_versions: state_guard.installed_godot_engine_versions.clone(),
        projects: state_guard.projects.clone(),
    };

    Ok(GodotEngineVersionResponse {
        installed_godot_versions: vec![],
        all_godot_versions,
    })
}

#[tauri::command]
async fn download_engine_version(
    state: tauri::State<'_, DataState>,
    engine_name: String,
) -> Result<(), ()> {
    let state_guard = state.0.lock().await;

    if let Some(_) = state_guard
        .installed_godot_engine_versions
        .iter()
        .find(|engine| engine.version_name == engine_name)
    {
        return Ok(()); // Already installed
    }

    let engine = state_guard
        .all_godot_engine_versions
        .clone()
        .into_iter()
        .find(|engine| engine.version_name == engine_name)
        .unwrap();

    drop(state_guard);

    // download
    let directory_service = ConfigDirectoryService::new();

    let updated_engine =
        download_service::download_and_extract_engine(&directory_service, &engine, &state)
            .await
            .unwrap();

    let mut state_guard = state.0.lock().await;

    let mut installed_versions = state_guard.installed_godot_engine_versions.clone();
    installed_versions.push(updated_engine);

    state_guard.installed_godot_engine_versions = installed_versions;

    Ok(())
}

#[tauri::command]
async fn get_installed_versions(
    state: tauri::State<'_, DataState>,
) -> Result<Vec<GodotEngineVersion>, ()> {
    let directory_service = ConfigDirectoryService::new();
    let installed_versions = godot_engine_service::get_installed_godot_versions(&directory_service);

    let guard = state.1.lock().await;

    // Don't include versions actively downloading
    let filtered: Vec<GodotEngineVersion> = installed_versions
        .into_iter()
        .filter(|engine| !guard.contains_key(&engine.version_name))
        .collect();
    drop(guard);

    let mut state_guard = state.0.lock().await;
    state_guard.installed_godot_engine_versions = filtered.clone();

    Ok(filtered)
}

#[tauri::command]
async fn remove_installed_version(
    state: tauri::State<'_, DataState>,
    engine_version_name: String,
) -> Result<Vec<GodotEngineVersion>, ()> {
    let directory_service = ConfigDirectoryService::new();
    let installed_versions = godot_engine_service::get_installed_godot_versions(&directory_service);

    let engine_version = installed_versions
        .iter()
        .find(|engine| engine.version_name == engine_version_name)
        .unwrap();

    let result =
        godot_engine_service::remove_installed_version(engine_version, &directory_service).unwrap();

    let new_installed_versions = get_installed_versions(state).await?;

    // TODO handle errors

    Ok(new_installed_versions)
}

#[tauri::command]
async fn get_all_projects(state: tauri::State<'_, DataState>) -> Result<Vec<ProjectData>, ()> {
    let config_directory = ConfigDirectoryService::new();
    let scanned_projects = get_all_projects_from_dirs(&config_directory);
    let config = config_directory_service::get_project_config(&config_directory);
    let mut state_guard = state.0.lock().await;

    let projects = project_service::project_reconciliation(
        config.tracked_projects,
        scanned_projects,
        &state_guard.installed_godot_engine_versions,
    );

    config_directory_service::save_projects_to_config(&config_directory, &projects);
    state_guard.projects = projects.clone();
    Ok(projects)
}

fn get_all_projects_from_dirs(config_directory: &ConfigDirectoryService) -> Vec<ProjectData> {
    let config = config_directory_service::get_project_config(&config_directory);
    let mut all_projects: Vec<ProjectData> = vec![];

    for directory in config.tracked_directories {
        let project_service = ProjectDirectoryService::new(&directory);
        let mut projects = project_service.find_projects();
        all_projects.append(&mut projects);
    }

    all_projects
}

#[tauri::command]
async fn save_project_path(
    state: tauri::State<'_, DataState>,
    project_directory: String,
) -> Result<Vec<String>, ()> {
    let config_directory = ConfigDirectoryService::new();
    let config = config_directory_service::get_project_config(&config_directory);
    let mut directories = config.tracked_directories;
    directories.push(project_directory);
    config_directory_service::save_tracked_directories_to_config(&config_directory, &directories);

    let updated_config = config_directory_service::get_project_config(&config_directory);
    Ok(updated_config.tracked_directories)
}

#[tauri::command]
async fn get_project_paths(state: tauri::State<'_, DataState>) -> Result<Vec<String>, ()> {
    let config_directory = ConfigDirectoryService::new();
    let config = config_directory_service::get_project_config(&config_directory);

    Ok(config.tracked_directories)
}

#[tauri::command]
async fn remove_project_path(
    state: tauri::State<'_, DataState>,
    project_directory: String,
) -> Result<Vec<String>, ()> {
    let config_directory = ConfigDirectoryService::new();
    let mut config = config_directory_service::get_project_config(&config_directory);
    config.tracked_directories = config
        .tracked_directories
        .into_iter()
        .filter(|value| *value != project_directory.to_string())
        .collect();

    config_directory_service::save_project_config(&config_directory, &config);

    Ok(config.tracked_directories)
}

#[tauri::command]
async fn set_engine_version_for_project(
    state: tauri::State<'_, DataState>,
    project_name: String,
    engine_name: String,
) -> Result<Vec<ProjectData>, ()> {
    let config_directory = ConfigDirectoryService::new();
    let mut config = config_directory_service::get_project_config(&config_directory);

    let state_guard = state.0.lock().await;
    let godot_versions = &state_guard.installed_godot_engine_versions;

    let project = config
        .tracked_projects
        .iter_mut()
        .find(|project| project.project_name == project_name)
        .expect(format!("Could not find tracked project with name {}", project_name).as_str());

    let godot_version = godot_versions
        .iter()
        .find(|engine| engine.version_name == engine_name);

    if let Some(engine_version) = godot_version {
        project.engine_version = engine_version.version_name.clone();
        project.engine_valid = true;
    } else {
        project.engine_version = "".to_string();
        project.engine_valid = false;
    }

    config_directory_service::save_project_config(&config_directory, &config);
    Ok(config.tracked_projects)
}

#[tauri::command]
async fn poll_download_status_list(
    state: tauri::State<'_, DataState>,
) -> Result<Vec<(String, usize)>, ()> {
    let state_guard = state.1.lock().await;
    let pairs = state_guard
        .iter()
        .map(|(name, score)| (name.clone(), score.clone()))
        .collect();

    Ok(pairs)
}

#[tauri::command]
async fn open_project(state: tauri::State<'_, DataState>, project_name: String) -> Result<(), ()> {
    let mut state_guard = state.0.lock().await;

    let project = state_guard
        .projects
        .iter()
        .find(|project| project.project_name == project_name)
        .expect(format!("Did not find a project with name {}", project_name).as_str());

    println!("Project last date opened is {}", project.last_date_opened);
    println!("state_guard {:?}", state_guard.projects.clone());

    let godot_engine = state_guard
        .installed_godot_engine_versions
        .iter()
        .find(|engine| engine.version_name == project.engine_version)
        .expect(
            format!(
                "Did not find a godot engine with name {}",
                project.engine_version
            )
            .as_str(),
        );

    let p = project.clone();
    let g = godot_engine.clone();

    state_guard
        .projects
        .iter_mut()
        .find(|project| project.project_name == project_name)
        .expect(format!("Did not find a project with name {}", project_name).as_str())
        .last_date_opened = Local::now().timestamp_millis();

    let config_directory = ConfigDirectoryService::new();
    config_directory_service::save_projects_to_config(&config_directory, &state_guard.projects);

    drop(state_guard);

    command::command::open_project(&p, &g);

    Ok(())
}

fn main() {
    let state = DataState(
        Mutex::new(Data {
            all_godot_engine_versions: vec![],
            installed_godot_engine_versions: vec![],
            projects: vec![],
        }),
        Mutex::new(HashMap::new()),
    );

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_engine_versions,
            download_engine_version,
            get_installed_versions,
            remove_installed_version,
            get_all_projects,
            save_project_path,
            get_project_paths,
            remove_project_path,
            set_engine_version_for_project,
            poll_download_status_list,
            open_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod tests {

    use std::fs;

    use tokio::sync::Mutex;

    use crate::{
        directory::config_directory_service::{self, ConfigDirectoryService},
        get_all_projects_from_dirs,
        project::{project_data::ProjectConfig, project_service::ProjectDirectoryService},
    };

    #[tokio::test]
    async fn test_reading_empty_config() {
        fs::create_dir_all(".\\test-project\\projects\\project1".to_string()).unwrap();
        fs::File::create(".\\test-project\\projects\\project1\\project.godot").unwrap();

        let config = &ProjectConfig {
            tracked_directories: vec![".\\test-project\\projects".to_string()],
            tracked_godot_versions: vec![],
            tracked_projects: vec![],
        };

        let config_directory = ConfigDirectoryService::new_test(
            ".\\test-project-config".to_string(),
            "config.json".to_string(),
        );

        config_directory_service::save_project_config(&config_directory, config);

        let all_projects = get_all_projects_from_dirs(&config_directory);

        assert!(all_projects.len() == 1);

        fs::remove_dir_all(".\\test-project").unwrap();
        fs::remove_dir_all(".\\test-project-config").unwrap();
    }
}
