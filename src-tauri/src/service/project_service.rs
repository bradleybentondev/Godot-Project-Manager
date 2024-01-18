use std::path::PathBuf;

use tauri::plugin::Plugin;

use crate::{
    directory::project_data::ProjectData, godot_service::godot_engine_version::GodotEngineVersion,
};

/// Takes in the existing_projects (which are cached in the config, with possibly an associated engine and other data),
/// the found_projects (which are newly scanned projects from the directories to be tracked), and all_godot_versions (which are all versions downloaded),
/// and reconciles them. This will combine the existing projects and found projects while validating their valid_path and valid_engine properties.
pub fn project_reconciliation(
    existing_projects: Vec<ProjectData>,
    found_projects: Vec<ProjectData>,
    all_godot_versions: Vec<GodotEngineVersion>,
) -> Vec<ProjectData> {
    let mut reconciled_projects: Vec<ProjectData> = existing_projects;

    for project in found_projects {
        if !reconciled_projects
            .iter()
            .any(|p| p.project_name == project.project_path)
        {
            reconciled_projects.push(project.clone());
        }
    }

    validate_project_paths(&mut reconciled_projects);
    validate_godot_versions(&mut reconciled_projects, all_godot_versions);

    return reconciled_projects;
}

/// Checks each ProjectData object and sets path_valid based on if the path it holds is valid or not
pub fn validate_project_paths(projects: &mut Vec<ProjectData>) {
    for project in projects {
        project.path_valid = PathBuf::from(&project.project_path).exists();
    }
}

/// Checks each ProjectData object and sets engine_valid based on if there is a matching engine version and the path to the engine version is valid
pub fn validate_godot_versions(
    projects: &mut Vec<ProjectData>,
    godot_versions: Vec<GodotEngineVersion>,
) {
    let mut godot_versions_iter = godot_versions.iter();
    for project in projects {
        let godot_version =
            godot_versions_iter.find(|godot| godot.version_name == project.project_version);
        project.engine_valid =
            godot_version.is_some() && PathBuf::from(&godot_version.unwrap().path).exists();
    }
}

pub fn open_project(project_path: PathBuf) {
    if !project_path.exists() {
        return;
    }
}

mod tests {
    use super::project_reconciliation;
    use crate::{
        directory::project_data::ProjectData,
        godot_service::godot_engine_version::GodotEngineVersion,
    };

    #[test]
    fn test_project_reconciliation() {
        let existing = vec![ProjectData {
            project_name: "test".to_string(),
            project_path: "test".to_string(),
            project_version: "V1".to_string(),
            last_date_opened: "".to_string(),
            path_valid: true,
            engine_valid: false,
        }];

        let found = vec![
            ProjectData {
                project_name: "test".to_string(),
                project_path: "test".to_string(),
                project_version: "".to_string(),
                last_date_opened: "".to_string(),
                path_valid: true,
                engine_valid: false,
            },
            ProjectData {
                project_name: "test2".to_string(),
                project_path: "test2".to_string(),
                project_version: "".to_string(),
                last_date_opened: "".to_string(),
                path_valid: true,
                engine_valid: false,
            },
        ];

        let godot_projects = vec![GodotEngineVersion {
            version_name: "godot 1".to_string(),
            version_number: "1".to_string(),
            path: "".to_string(),
            updated_at: "".to_string(),
        }];

        let reconciled = project_reconciliation(existing, found, godot_projects);

        assert!(reconciled.len() == 2);
        assert!(reconciled.first().unwrap().project_version == "V1");
        assert!(reconciled.first().unwrap().path_valid == false);
        assert!(reconciled.first().unwrap().engine_valid == false);
    }
}
