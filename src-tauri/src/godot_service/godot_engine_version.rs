use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GodotEngineVersion {
    #[serde(rename(serialize = "engineName"))]
    pub version_name: String,
    #[serde(rename(serialize = "engineVersion"))]
    pub version_number: String,
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: String,
    #[serde(rename(serialize = "installationPath"))]
    pub path: String,
}
