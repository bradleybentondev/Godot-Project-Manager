use std::{path::{Path, PathBuf}, fs};

use directories::BaseDirs;

use super::project_data::ProjectData;

pub struct DirectoryService {
    storage_path: PathBuf,
    engine_storage_path: PathBuf,
    config_file_path: PathBuf,
}

pub struct TestDirectoryService {
    storage_path: PathBuf,
    engine_storage_path: PathBuf,
    config_file_path: PathBuf,
}

pub trait Directories {
    fn storage_path(&self) -> &Path;

    fn engine_storage_path(&self) -> &Path;

    fn config_file_path(&self) -> &Path;

    fn engine_version_path(&self, engine_version_name: &str) -> PathBuf;

    fn load_existing_projects_from_config(&self) -> Vec<ProjectData>;

    fn write_existing_projects_to_config(&self, projects: &Vec<ProjectData>);
}

impl TestDirectoryService {
    const TEST_PATH: &'static str = ".\\test_data";
    
    pub fn new() -> impl Directories {
        TestDirectoryService {
            storage_path: Self::get_storage_path(),
            engine_storage_path: Self::get_engine_dir_path(),
            config_file_path: Self::get_config_file_path()
        }
    }

    /// Gets the app's storage path
    ///
    /// # Panics
    ///
    /// Panics if no valid home directory can be found
    fn get_storage_path() -> PathBuf {
        let mut path = PathBuf::from(&Self::TEST_PATH);
        path.push("godot_project_manager");
        path
    }

    /// Gets the storage path of the engines
    ///
    /// # Panics
    ///
    /// Panics if no valid home directory can be found
    fn get_engine_dir_path() -> PathBuf {
        let mut path = Self::get_storage_path();
        path.push("engines");
        path
    }

    /// Gets the path of the config file
    ///
    /// # Panics
    ///
    /// Panics if no valid home directory can be found
    fn get_config_file_path() -> PathBuf{
        let mut path = Self::get_storage_path();
        path.push("config.json");
        path
    }
}

impl DirectoryService {
    pub fn new() -> impl Directories {
        DirectoryService {
            storage_path: Self::get_storage_path(),
            engine_storage_path: Self::get_engine_dir_path(),
            config_file_path: Self::get_config_file_path()
        }
    }

    /// Gets the app's storage path
    ///
    /// # Panics
    ///
    /// Panics if no valid home directory can be found
    fn get_storage_path() -> PathBuf {
        let mut path = BaseDirs::new().unwrap().config_dir().to_path_buf();
        path.push("godot_project_manager");
        path
    }

    /// Gets the storage path of the engines
    ///
    /// # Panics
    ///
    /// Panics if no valid home directory can be found
    fn get_engine_dir_path() -> PathBuf {
        let mut path = Self::get_storage_path();
        path.push("engines");
        path
    }

    /// Gets the path of the config file
    ///
    /// # Panics
    ///
    /// Panics if no valid home directory can be found
    fn get_config_file_path() -> PathBuf{
        let mut path = Self::get_storage_path();
        path.push("config.json");
        path
    }
}

impl Directories for DirectoryService {
    fn storage_path(&self) -> &Path {
        &self.storage_path
    }

    fn engine_storage_path(&self) -> &Path {
        &self.engine_storage_path
    }

    fn config_file_path(&self) -> &Path {
        &self.config_file_path
    }

    fn engine_version_path(&self, engine_version_name: &str) -> PathBuf {
        let mut path = PathBuf::from(&self.engine_storage_path);
        path.push(engine_version_name);
        path
    }

    fn load_existing_projects_from_config(&self) -> Vec<ProjectData> { 
        if !&self.storage_path().exists() {
            fs::create_dir_all(&self.storage_path()).unwrap();
        }
    
        let path = &self.config_file_path();    
    
        let contents = fs::read_to_string(&path).expect(format!("Could not read data from file at path {}", path.display()).as_str());
        let data: Vec<ProjectData> = serde_json::from_str(contents.as_str()).unwrap();
    
        return data;
    }
    
    fn write_existing_projects_to_config(&self, projects: &Vec<ProjectData>) {
        if !&self.storage_path().exists() {
            fs::create_dir_all(&self.storage_path()).unwrap();
        }
    
        let contents = serde_json::to_string(projects).expect("Could not serialize data");
    
        let path = &self.config_file_path();
    
        fs::write(path, contents).expect("Could not write to file");
    }
}

impl Directories for TestDirectoryService {
    fn storage_path(&self) -> &Path {
        &self.storage_path
    }

    fn engine_storage_path(&self) -> &Path {
        &self.engine_storage_path
    }

    fn config_file_path(&self) -> &Path {
        &self.config_file_path
    }

    fn engine_version_path(&self, engine_version_name: &str) -> PathBuf {
        let mut path = PathBuf::from(&self.engine_storage_path);
        path.push(engine_version_name);
        path
    }

    fn load_existing_projects_from_config(&self) -> Vec<ProjectData> { 
        if !&self.storage_path().exists() {
            fs::create_dir_all(&self.storage_path()).unwrap();
        }
    
        let path = &self.config_file_path();    
    
        let contents = fs::read_to_string(&path).expect(format!("Could not read data from file at path {}", path.display()).as_str());
        let data: Vec<ProjectData> = serde_json::from_str(contents.as_str()).unwrap();
    
        return data;
    }
    
    fn write_existing_projects_to_config(&self, projects: &Vec<ProjectData>) {
        if !&self.storage_path().exists() {
            fs::create_dir_all(&self.storage_path()).unwrap();
        }
    
        let contents = serde_json::to_string(projects).expect("Could not serialize data");
    
        let path = &self.config_file_path();
    
        fs::write(path, contents).expect("Could not write to file");
    }
}