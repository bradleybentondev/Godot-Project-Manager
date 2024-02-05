use std::{
    fs,
    path::{Path, PathBuf},
};

use directories::BaseDirs;

use crate::{
    godot_service::godot_engine_version::GodotEngineVersion,
    project::project_data::{ProjectConfig, ProjectData},
};

pub struct ConfigDirectoryService {
    config_file_name: String,
    storage_path: PathBuf,
    engine_storage_path: PathBuf,
    config_file_path: PathBuf,
}

impl ConfigDirectoryService {
    pub fn new(config_file_name: String) -> ConfigDirectoryService {
        let storage_path = BaseDirs::new().unwrap().config_dir().to_path_buf();
        ConfigDirectoryService {
            config_file_name: config_file_name.to_string(),
            storage_path: storage_path.clone(),
            engine_storage_path: Self::get_engine_dir_path(storage_path.clone()),
            config_file_path: Self::get_config_file_path(storage_path.clone(), config_file_name),
        }
    }

    pub fn new_test(base_path: String, config_file_name: String) -> ConfigDirectoryService {
        let storage_path = PathBuf::from(base_path);
        ConfigDirectoryService {
            config_file_name: config_file_name.to_string(),
            storage_path: storage_path.clone(),
            engine_storage_path: Self::get_engine_dir_path(storage_path.clone()),
            config_file_path: Self::get_config_file_path(storage_path.clone(), config_file_name),
        }
    }

    /// Gets the storage path of the engines
    ///
    /// # Panics
    ///
    /// Panics if no valid home directory can be found
    fn get_engine_dir_path(storage_path: PathBuf) -> PathBuf {
        let mut path = storage_path;
        path.push("engines");
        path
    }

    /// Gets the path of the config file
    ///
    /// # Panics
    ///
    /// Panics if no valid home directory can be found
    pub fn get_config_file_path(storage_path: PathBuf, config_file_name: String) -> PathBuf {
        let mut path = storage_path;
        path.push(config_file_name);
        path
    }

    pub fn storage_path(&self) -> &Path {
        &self.storage_path
    }

    pub fn engine_storage_path(&self) -> &Path {
        &self.engine_storage_path
    }

    pub fn config_file_path(&self) -> &Path {
        &self.config_file_path
    }

    pub fn engine_version_path(&self, engine_version_name: &str) -> PathBuf {
        let mut path = PathBuf::from(&self.engine_storage_path);
        path.push(engine_version_name);
        path
    }

    fn create_config_path_if_not_exsits(&self, storage_path: &Path) {
        if !storage_path.exists() {
            let mut path = PathBuf::from(storage_path);
            fs::create_dir_all(&path).unwrap();
            path.push(&self.config_file_name);
            fs::File::create(&path).unwrap();

            let config = ProjectConfig {
                tracked_directories: vec![],
                tracked_godot_versions: vec![],
                tracked_projects: vec![],
            };

            fs::write(&path, serde_json::to_string(&config).unwrap()).unwrap();
        }
    }
}

fn get_existing_projects_from_config(directory: &ConfigDirectoryService) -> Vec<ProjectData> {
    directory.create_config_path_if_not_exsits(directory.storage_path());

    let path = directory.config_file_path();

    let contents = fs::read_to_string(&path)
        .expect(format!("Could not read data from file at path {}", path.display()).as_str());
    let data: ProjectConfig = serde_json::from_str(contents.as_str()).unwrap();

    return data.tracked_projects;
}

pub fn get_project_config(directory: &ConfigDirectoryService) -> ProjectConfig {
    directory.create_config_path_if_not_exsits(directory.storage_path());

    let path = directory.config_file_path();

    println!("Trying to read from path {}", &path.display());

    let contents = fs::read_to_string(&path)
        .expect(format!("Could not read data from file at path {}", path.display()).as_str());

    let config: ProjectConfig = serde_json::from_str(contents.as_str()).unwrap();

    return config;
}

pub fn save_project_config(directory: &ConfigDirectoryService, config: &ProjectConfig) {
    directory.create_config_path_if_not_exsits(directory.storage_path());

    let path = directory.config_file_path();

    let serialized = serde_json::to_string(&config).unwrap();

    fs::write(path, serialized).unwrap();
}

pub fn save_projects_to_config(directory: &ConfigDirectoryService, projects: &Vec<ProjectData>) {
    let mut data: ProjectConfig = get_project_config(directory);

    data.tracked_projects = projects.clone();

    save_project_config(directory, &data);
}

pub fn save_engine_versions_to_config(
    directory: &ConfigDirectoryService,
    engine_versions: &Vec<GodotEngineVersion>,
) {
    let mut data: ProjectConfig = get_project_config(directory);

    data.tracked_godot_versions = engine_versions.clone();

    save_project_config(directory, &data);
}

pub fn save_tracked_directories_to_config(
    directory: &ConfigDirectoryService,
    directories: &Vec<String>,
) {
    let mut data: ProjectConfig = get_project_config(directory);

    data.tracked_directories = directories.clone();

    save_project_config(directory, &data);
}

fn write_existing_projects_to_config(
    directory: &ConfigDirectoryService,
    projects: &Vec<ProjectData>,
) {
    directory.create_config_path_if_not_exsits(directory.storage_path());

    let contents = serde_json::to_string(projects).expect("Could not serialize data");

    let path = directory.config_file_path();

    fs::write(path, contents).expect("Could not write to file");
}
