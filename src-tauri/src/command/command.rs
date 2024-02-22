use std::process::Command;

use crate::{
    godot_service::godot_engine_version::GodotEngineVersion, project::project_data::ProjectData,
};

pub fn open_project(project: &ProjectData, engine: &GodotEngineVersion) {
    // Command::new("C:\\Users\\bbent\\AppData\\Roaming\\godot_project_manager\\engines\\Godot_v4.2.1-stable_mono_win64\\Godot_v4.2.1-stable_mono_win64.exe").
    // arg("C:\\Users\\bbent\\Desktop\\AllProjectFiles\\godot\\arenagame\\project.godot").spawn().unwrap();

    println!(
        "running command {} {}",
        &engine.executable_path, &project.project_path
    );
    let output = if cfg!(target_os = "windows") {
        Command::new(&engine.executable_path)
            .arg(&project.project_path)
            .spawn()
            .unwrap()
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
