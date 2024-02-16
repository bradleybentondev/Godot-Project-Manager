use std::process::Command;

use crate::{
    godot_service::{godot_engine_service, godot_engine_version::GodotEngineVersion},
    project::project_data::ProjectData,
};

pub fn open_project(project: ProjectData, engine: GodotEngineVersion) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args([engine.path, project.project_path])
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
