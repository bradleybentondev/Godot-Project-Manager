import { useEffect, useState } from "react";
import styles from "./App.module.css";
import SideBar from "./components/SideBar";
import { PageEnum } from "./data/PageEnum";
import { GodotEngineVersion } from "./data/GodotEngineVersion";
import { invoke } from "@tauri-apps/api";
import { GodotEngineResponse } from "./data/GodotEngineResponse";
import { ProjectData } from "./data/ProjectData";
import ProjectPage from "./components/ProjectPage";
import EnginePage from "./components/EnginePage";
import SettingsPage from "./components/SettingsPage";

function App() {
  const [page, setPage] = useState(PageEnum.Projects);
  const [allEngines, setAllEngines] = useState<GodotEngineVersion[]>([]);
  const [installedEngines, setInstalledEngines] = useState<GodotEngineVersion[]>([]);
  const [projects, setProjects] = useState<ProjectData[]>([]);
  const [projectPaths, setProjectPaths] = useState<string[]>([]);

  async function init() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // invoke()

    // Get available godot releases - can be async

    // Get local godot versions
    // Get config stored projects
    // get local scanned projects

    invoke<GodotEngineResponse>("get_engine_versions").then(response => {
      console.log(response);
      setAllEngines(response.allVersions);
    })

    invoke<GodotEngineVersion[]>("get_installed_versions").then(response => {
      setInstalledEngines(response);
    })

    getAllProjects();

    invoke<string[]>("get_project_paths").then(response => {
      console.log("get_project_paths", response)
      setProjectPaths(response);
    })
  }

  function testData(): ProjectData[] {
    return [
      new ProjectData("Some project", "test", "2023-08-19", "4.1.2 Mono", false),
      new ProjectData("Some project 2", "test2", "2023-05-19", "3.1.2", false),
      new ProjectData("Some project 3", "test3", "2023-07-19", "4.2.2", false),
      new ProjectData("Some project 4", "test4", "2023-06-19", "3.3.2 Mono", false),
    ].sort((a, b) => b.lastOpened - a.lastOpened);
  }

  function getAllProjects() {
    invoke<ProjectData[]>("get_all_projects").then(response => {
      setProjects(response);
      console.log(response);
    })
  }

  function deleteVersion(engineName: string) {
    invoke<GodotEngineVersion[]>("remove_installed_version", { engineVersionName: engineName }).then(response => {
      console.log(response);
      setInstalledEngines(response);
    })
  }

  function downloadEngine(engineName: string) {
    invoke("download_engine_version", { engineName: engineName }).then(response => {
      console.log(response);
      invoke<GodotEngineVersion[]>("get_installed_versions").then(response => {
        setInstalledEngines(response);
      })
    })
  }

  useEffect(() => {
    init();
  }, []);

  return (
    <div className={styles.mainContainer}>
      <SideBar setPage={setPage} projectCount={projects.length} engineCount={allEngines.length} newsCount={12} projects={projects} />

      {page == PageEnum.Projects ? (
        <ProjectPage installedGodotEngines={installedEngines} projects={projects} />
      ) : page == PageEnum.Engines ? (
        <EnginePage allGodotEngines={allEngines} installedGodotEngines={installedEngines} downloadEngineFunc={downloadEngine} deleteVersion={deleteVersion} />
      ) : page == PageEnum.Settings ? (
        <SettingsPage initialProjectPaths={projectPaths} refreshProjects={getAllProjects} />
      ) : null}
    </div>
  );
}

export default App;
