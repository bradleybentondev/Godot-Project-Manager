use std::{fs, path::{PathBuf, Path}};
use super::{project_data::ProjectData, config_directory_service::DirectoryService};
use directories::BaseDirs;

pub struct ProjectDirectory {
    base_path: String
}

impl ProjectDirectory {
    pub fn new(base_path: &str) -> ProjectDirectory {
        ProjectDirectory {
            base_path: base_path.to_string()
        }
    }

    pub fn find_projects(&self) -> Vec<ProjectData> {
        let projects = Self::scan_path(PathBuf::from(&self.base_path));
    
        return projects;
    }
    
    fn scan_path(path: PathBuf) -> Vec<ProjectData>{
        let mut project_paths: Vec<ProjectData> = vec![];
        let mut nested_paths: Vec<PathBuf> = vec![];
    
        let paths = fs::read_dir(path).unwrap();
    
        for path in paths {
            let p =  path.unwrap().path();
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
        let split:Vec<&str> = path.split("/").collect();
    
        let folder_name = split[split.len()-2];
    
        return ProjectData {
            project_name: folder_name.to_string(),
            project_path: path,
            project_version: "".to_string(),
            last_date_opened: "".to_string(),
            path_valid: false,
            engine_valid: false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, fs::{self, File}};
    use directories::BaseDirs;

    use crate::directory::directory::ProjectDirectory;

    const TEST_PATH: &str = "./test_data/projects/";

    fn setup(){
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

    fn cleanup(){
        fs::remove_dir_all("./test_data").unwrap();
    }

    #[test]
    fn test_find_projects() {
        setup();
        let path = TEST_PATH;
        println!("searching {}", &path);
        let project_directory = ProjectDirectory::new(path);

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
}
