use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData{
    pub project_name: String,
    pub project_path: String,
    pub project_version: String,
    pub last_date_opened: String,
    pub path_valid: bool,
    pub engine_valid: bool
} 

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectConfig{
    pub tracked_directories: Vec<String>,
    pub tracked_projects: Vec<ProjectData>,
    pub tracked_godot_versions: Vec<ProjectData>
} 