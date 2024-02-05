use crate::godot_service::godot_engine_version::GodotEngineVersion;

use super::project_data::ProjectData;
use directories::BaseDirs;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct ProjectDirectoryService {
    base_path: String,
}

impl ProjectDirectoryService {
    pub fn new(base_path: &str) -> ProjectDirectoryService {
        ProjectDirectoryService {
            base_path: base_path.to_string(),
        }
    }

    pub fn find_projects(&self) -> Vec<ProjectData> {
        let projects = Self::scan_path(PathBuf::from(&self.base_path));

        return projects;
    }

    fn scan_path(path: PathBuf) -> Vec<ProjectData> {
        let mut project_paths: Vec<ProjectData> = vec![];
        let mut nested_paths: Vec<PathBuf> = vec![];

        let paths = fs::read_dir(path).unwrap();

        for path in paths {
            let p = path.unwrap().path();
            if p.display().to_string().contains("project.godot") {
                let project = Self::create_project_data(p);
                project_paths.push(project);
                return project_paths;
            }

            if p.display().to_string() == ".." {
                continue;
            }

            if p.is_dir() {
                nested_paths.push(p);
            }
        }

        for path in nested_paths {
            let mut result = Self::scan_path(path.to_owned());
            project_paths.append(&mut result)
        }

        return project_paths;
    }

    fn create_project_data(path: PathBuf) -> ProjectData {
        let path = path.to_str().unwrap().replace("\\", "/");
        let split: Vec<&str> = path.split("/").collect();

        let folder_name = split[split.len() - 2];

        return ProjectData {
            project_name: folder_name.to_string(),
            project_path: path,
            project_version: "".to_string(),
            last_date_opened: "".to_string(),
            path_valid: false,
            engine_valid: false,
        };
    }
}

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

#[cfg(test)]
mod tests {
    use directories::BaseDirs;
    use std::{
        fs::{self, File},
        path::PathBuf,
    };

    use crate::{
        godot_service::godot_engine_version::GodotEngineVersion,
        project::{
            project_data::ProjectData,
            project_service::{project_reconciliation, ProjectDirectoryService},
        },
    };

    const TEST_PATH: &str = "./test_data/projects/";

    fn setup() {
        let mut path1 = PathBuf::from(TEST_PATH);
        path1.push("project1");
        fs::create_dir_all(&path1).unwrap();
        path1.push("project.godot");

        let mut path2 = PathBuf::from(TEST_PATH);
        path2.push("project2");
        fs::create_dir_all(&path2).unwrap();
        path2.push("godot2.nope");

        let mut path3 = PathBuf::from(TEST_PATH);
        path3.push("project3");
        fs::create_dir_all(&path3).unwrap();
        path3.push("project.godot");

        File::create(path1).unwrap();
        File::create(path2).unwrap();
        File::create(path3).unwrap();
    }

    fn cleanup() {
        fs::remove_dir_all("./test_data").unwrap();
    }

    #[test]
    fn test_find_projects() {
        setup();
        let path = TEST_PATH;
        println!("searching {}", &path);
        let project_directory = ProjectDirectoryService::new(path);

        let projects = project_directory.find_projects();
        println!("Found {} projects", projects.len());
        for project in &projects {
            println!("{} - {}", project.project_name, project.project_path)
        }
        assert!(projects.len() == 2);
        cleanup();
    }

    #[test]
    fn test_project_dir() {
        let dir = BaseDirs::new().unwrap();
        println!("path: {}", dir.config_dir().display());
        // assert!(dir)
    }

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
            download_url: "".to_string(),
        }];

        let reconciled = project_reconciliation(existing, found, godot_projects);

        assert!(reconciled.len() == 2);
        assert!(reconciled.first().unwrap().project_version == "V1");
        assert!(reconciled.first().unwrap().path_valid == false);
        assert!(reconciled.first().unwrap().engine_valid == false);
    }
}
