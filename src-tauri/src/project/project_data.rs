use core::fmt;
use std::path::Display;

use serde::{Deserialize, Serialize};

use crate::godot_service::godot_engine_version::GodotEngineVersion;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData {
    #[serde(rename(serialize = "projectName", deserialize = "projectName"))]
    pub project_name: String,
    #[serde(rename(serialize = "projectPath", deserialize = "projectPath"))]
    pub project_path: String,
    #[serde(rename(serialize = "projectVersion", deserialize = "projectVersion"))]
    pub project_version: String,
    #[serde(rename(serialize = "lastDateOpened", deserialize = "lastDateOpened"))]
    pub last_date_opened: String,
    #[serde(rename(serialize = "pathValid", deserialize = "pathValid"))]
    pub path_valid: bool,
    #[serde(rename(serialize = "engineValid", deserialize = "engineValid"))]
    pub engine_valid: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub tracked_directories: Vec<String>,
    pub tracked_projects: Vec<ProjectData>,
    pub tracked_godot_versions: Vec<GodotEngineVersion>,
}

impl fmt::Display for ProjectConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "tracked_directories: {:?}, tracked_projects: {:?}, tracked_godot_versions: {:?}",
            self.tracked_directories, self.tracked_projects, self.tracked_godot_versions
        )
    }
}

impl fmt::Display for ProjectData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "project_name: {}, project_path: {}, project_version: {}, last_date_opened: {}, path_valid: {}, engine_valid: {}",
            self.project_name, self.project_path, self.project_version, self.last_date_opened, self.path_valid, self.engine_valid
        )
    }
}

impl fmt::Debug for ProjectData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "project_name: {}, project_path: {}, project_version: {}, last_date_opened: {}, path_valid: {}, engine_valid: {}",
            self.project_name, self.project_path, self.project_version, self.last_date_opened, self.path_valid, self.engine_valid
        )
    }
}
