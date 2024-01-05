use std::{fs, path::PathBuf};
use super::project_data::ProjectData;
use directories::BaseDirs;

pub fn get_storage_path() -> PathBuf {
    let mut path = BaseDirs::new().unwrap().config_dir().to_path_buf();
    path.push("godot_project_manager");
    path
}

pub fn get_file_path() -> PathBuf{
    let mut path = BaseDirs::new().unwrap().config_dir().to_path_buf();
    path.push("godot_project_manager");
    path.push("config.json");
    path
}

pub fn load_existing_projects() -> Vec<ProjectData> { 
    if !get_storage_path().exists() {
        mkdir();
    }

    let path = get_file_path();    

    let contents = fs::read_to_string(&path).expect(format!("Could not read data from file at path {}", path.display()).as_str());
    let data: Vec<ProjectData> = serde_json::from_str(contents.as_str()).unwrap();

    return data;
}

pub fn write_existing_projects(projects: &Vec<ProjectData>) {
    if !get_storage_path().exists() {
        mkdir();
    }

    let contents = serde_json::to_string(projects).expect("Could not serialize data");

    let path = get_file_path();

    fs::write(path, contents).expect("Could not write to file");
}

pub fn find_projects(path: String) -> Vec<ProjectData> {
    let projects = scan_path(PathBuf::from(path));

    return projects;
}

fn scan_path(path: PathBuf) -> Vec<ProjectData>{
    let mut project_paths: Vec<ProjectData> = vec![];
    let mut nested_paths: Vec<PathBuf> = vec![];

    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let p =  path.unwrap().path();
        if p.display().to_string().contains("project.godot") {
            let project = create_project_data(p);
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
        let mut result = scan_path(path.to_owned());
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
        valid: false
    }
}

fn mkdir() {
    let path = get_storage_path();
    // fs::create_dir_all(path);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use directories::BaseDirs;

    use crate::directory::directory::{scan_path, find_projects};

    #[test]
    fn it_works() {
        let path = PathBuf::from("C:/Users/bbent/Desktop/AllProjectFiles/godot");

        let result = scan_path(path);
        assert!(result.len() >= 1)
    }

    #[test]
    fn test_find_projects() {
        let path ="C:/Users/bbent/Desktop/AllProjectFiles/godot";

        let projects = find_projects(path.to_string());
        for project in &projects {
            println!("{} - {}", project.project_name, project.project_path)
        }
        assert!(projects.len() >= 1)
    }

    #[test]
    fn test_project_dir() {
        let dir = BaseDirs::new().unwrap();
        println!("path: {}", dir.config_dir().display());
        // assert!(dir)
    }
}
