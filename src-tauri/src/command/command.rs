use std::process::Command;

use crate::{
    godot_service::godot_engine_version::GodotEngineVersion, project::project_data::ProjectData,
};

pub fn open_project(project: &ProjectData, engine: &GodotEngineVersion) {
    if cfg!(target_os = "windows") {
        Command::new(&engine.executable_path)
            .arg(&project.project_path)
            .spawn()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .spawn()
    };
}

pub fn open_engine(engine: &GodotEngineVersion) {
    let output = if cfg!(target_os = "windows") {
        Command::new(&engine.executable_path).spawn().unwrap()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .spawn()
            .unwrap()
    };
}

mod tests {
    use std::process::Command;

    #[tokio::test]
    async fn test_command() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
        };
    }
}
