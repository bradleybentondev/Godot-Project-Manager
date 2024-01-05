use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData{
    pub project_name: String,
    pub project_path: String,
    pub project_version: String,
    pub last_date_opened: String,
    pub valid: bool
} 