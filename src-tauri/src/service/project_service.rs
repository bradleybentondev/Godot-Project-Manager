use std::path::PathBuf;

use tauri::plugin::Plugin;

use crate::{directory::project_data::ProjectData, godot_service::godot_engine_version::GodotEngineVersion};


pub fn project_reconciliation(existing_projects: Vec<ProjectData>, found_projects: Vec<ProjectData>) -> Vec<ProjectData> {
    let mut reconciled_projects: Vec<ProjectData> = existing_projects;

    for project in found_projects {
        if !reconciled_projects.iter().any(|p| p.project_name == project.project_path) {
            reconciled_projects.push(project.clone());
        }
    }

    check_project_paths(&mut reconciled_projects);

    return reconciled_projects;
}

pub fn check_project_paths(projects: &mut Vec<ProjectData>) {
    for project in projects {
        if !PathBuf::from(&project.project_path).exists() {
            project.valid = false;
        }
    }
}

pub fn check_godot_version(projects: &mut Vec<ProjectData>, godot_versions: Vec<GodotEngineVersion>){
        let godot_versions_iter = godot_versions.iter();
    for project in projects {
        // let godot_version = godot_versions_iter.find(|godot| godot
        // if !PathBuf::from(&project.project_path).exists() {
        //     project.valid = false;
        // }
    }
}

pub fn open_project(project_path: PathBuf){
    if !project_path.exists() {
        return;
    }


}

mod tests {
    use crate::directory::project_data::ProjectData;
    use super::project_reconciliation;

    #[test]
    fn test_project_reconciliation() {
        let existing = vec![
            ProjectData {
                project_name: "test".to_string(),
                project_path: "test".to_string(),
                project_version: "V1".to_string(),
                last_date_opened: "".to_string(),
                valid: true
            }
        ];

        let found = vec![
            ProjectData {
                project_name: "test".to_string(),
                project_path: "test".to_string(),
                project_version: "".to_string(),
                last_date_opened: "".to_string(),
                valid: true
            },
            ProjectData {
                project_name: "test2".to_string(),
                project_path: "test2".to_string(),
                project_version: "".to_string(),
                last_date_opened: "".to_string(),
                valid: true
            }
        ];

        let reconciled = project_reconciliation(existing, found);

        assert!(reconciled.len() == 2);
        assert!(reconciled.first().unwrap().project_version == "V1");
        assert!(reconciled.first().unwrap().valid == false);
    }
}