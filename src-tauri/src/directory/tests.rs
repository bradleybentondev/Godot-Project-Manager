mod tests {
    use std::fs;

    use crate::{
        directory::config_directory_service::{
            get_project_config, save_project_config, ConfigDirectoryService,
        },
        godot_service::godot_engine_version::GodotEngineVersion,
        project::project_data::{ProjectConfig, ProjectData},
    };

    #[tokio::test]
    async fn test_reading_empty_config() {
        let directory =
            ConfigDirectoryService::new_test(".\\test-data".to_string(), "test1.json".to_string());
        let config = get_project_config(&directory);

        println!("config: {}", config);

        assert!(config.tracked_directories.len() == 0);
        assert!(config.tracked_godot_versions.len() == 0);
        assert!(config.tracked_projects.len() == 0);

        fs::remove_dir_all(".\\test-data").expect("Could not find dir to delete");
    }

    #[tokio::test]
    async fn test_writing_tracked_directories_to_config() {
        let config = ProjectConfig {
            tracked_directories: vec!["test".to_string(), "test2".to_string()],
            tracked_godot_versions: vec![],
            tracked_projects: vec![],
        };

        let directory = ConfigDirectoryService::new_test(
            ".\\test-data-2".to_string(),
            "test2.json".to_string(),
        );

        save_project_config(&directory, &config);

        let config = get_project_config(&directory);

        assert!(config.tracked_directories.len() == 2);
        assert!(config.tracked_godot_versions.len() == 0);
        assert!(config.tracked_projects.len() == 0);

        fs::remove_dir_all(".\\test-data-2").unwrap();
    }

    #[tokio::test]
    async fn test_writing_tracked_projects_to_config() {
        let config = ProjectConfig {
            tracked_directories: vec![],
            tracked_godot_versions: vec![],
            tracked_projects: vec![ProjectData {
                project_name: "test".to_string(),
                project_path: "test".to_string(),
                project_version: "test".to_string(),
                last_date_opened: "test".to_string(),
                path_valid: false,
                engine_valid: false,
            }],
        };

        let directory = ConfigDirectoryService::new_test(
            ".\\test-data-3".to_string(),
            "test2.json".to_string(),
        );

        save_project_config(&directory, &config);

        let config = get_project_config(&directory);

        assert!(config.tracked_directories.len() == 0);
        assert!(config.tracked_godot_versions.len() == 0);
        assert!(config.tracked_projects.len() == 1);

        fs::remove_dir_all(".\\test-data-3").unwrap();
    }

    #[tokio::test]
    async fn test_writing_tracked_engines_to_config() {
        let config = ProjectConfig {
            tracked_directories: vec![],
            tracked_godot_versions: vec![GodotEngineVersion {
                version_name: "test".to_string(),
                version_number: "test".to_string(),
                updated_at: "test".to_string(),
                path: "test".to_string(),
                download_url: "test".to_string(),
            }],
            tracked_projects: vec![],
        };

        let directory = ConfigDirectoryService::new_test(
            ".\\test-data-10".to_string(),
            "test2.json".to_string(),
        );

        save_project_config(&directory, &config);

        let config = get_project_config(&directory);

        assert!(config.tracked_directories.len() == 0);
        assert!(config.tracked_godot_versions.len() == 1);
        assert!(config.tracked_projects.len() == 0);

        fs::remove_dir_all(".\\test-data-10").unwrap();
    }

    #[tokio::test]
    async fn test_writing_all_to_config() {
        let config = ProjectConfig {
            tracked_directories: vec!["test".to_string(), "test2".to_string()],
            tracked_godot_versions: vec![GodotEngineVersion {
                version_name: "test".to_string(),
                version_number: "test".to_string(),
                updated_at: "test".to_string(),
                path: "test".to_string(),
                download_url: "test".to_string(),
            }],
            tracked_projects: vec![ProjectData {
                project_name: "test".to_string(),
                project_path: "test".to_string(),
                project_version: "test".to_string(),
                last_date_opened: "test".to_string(),
                path_valid: false,
                engine_valid: false,
            }],
        };

        let directory = ConfigDirectoryService::new_test(
            ".\\test-data-5".to_string(),
            "test2.json".to_string(),
        );

        save_project_config(&directory, &config);

        let config = get_project_config(&directory);

        assert!(config.tracked_directories.len() == 2);
        assert!(config.tracked_godot_versions.len() == 1);
        assert!(config.tracked_projects.len() == 1);

        fs::remove_dir_all(".\\test-data-5").unwrap();
    }
}
