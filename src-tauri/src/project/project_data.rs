use core::fmt;
use std::path::Display;

use serde::{Deserialize, Serialize};

use crate::godot_service::godot_engine_version::GodotEngineVersion;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData {
    pub project_name: String,
    pub project_path: String,
    pub project_version: String,
    pub last_date_opened: String,
    pub path_valid: bool,
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
