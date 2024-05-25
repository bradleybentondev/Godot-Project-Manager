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
import { NewsEntry } from "./data/NewsEntry";
import NewsPage from "./components/NewsPage";

function App() {
  const [page, setPage] = useState(PageEnum.Projects);
  const [allEngines, setAllEngines] = useState<GodotEngineVersion[]>([]);
  const [installedEngines, setInstalledEngines] = useState<GodotEngineVersion[]>([]);
  const [projects, setProjects] = useState<ProjectData[]>([]);
  const [projectPaths, setProjectPaths] = useState<string[]>([]);
  const [newsEntries, setNewsEntries] = useState<NewsEntry[]>([]);


  async function init() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // invoke()

    // Get available godot releases - can be async

    // Get local godot versions
    // Get config stored projects
    // get local scanned projects

    let allEngines = await invoke<GodotEngineResponse>("get_engine_versions");
    setAllEngines(allEngines.allVersions);
    console.log("get all engines", allEngines);


    let installedVersions = await invoke<GodotEngineVersion[]>("get_installed_versions");
    setInstalledEngines(installedVersions);
    console.log("get installed engines", installedVersions);


    getAllProjects();

    invoke<string[]>("get_project_paths").then(response => {
      setProjectPaths(response);
    })

    invoke<NewsEntry[]>("get_news_entries").then(response => {
      setNewsEntries(response);
    })
  }


  function getAllProjects() {
    invoke<ProjectData[]>("get_all_projects").then(response => {
      console.log("get all projects", response);
      setProjects(response);
    })
  }

  function setProjectEngineVersion(projectName: string, engineName: string) {
    invoke<ProjectData[]>("set_engine_version_for_project", { projectName: projectName, engineName: engineName }).then(projects => {
      setProjects(projects);
    })
  }

  function deleteVersion(engineName: string) {
    invoke<GodotEngineVersion[]>("remove_installed_version", { engineVersionName: engineName }).then(response => {
      setInstalledEngines(response);
    })
  }

  function downloadEngine(engineName: string) {
    console.log("downloading version");
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
        <ProjectPage installedGodotEngines={installedEngines} allProjects={projects} setProjectEngineVersion={setProjectEngineVersion} />
      ) : page == PageEnum.Engines ? (
        <EnginePage allGodotEngines={allEngines} installedGodotEngines={installedEngines} downloadEngineFunc={downloadEngine} deleteVersion={deleteVersion} />
      ) : page == PageEnum.Settings ? (
        <SettingsPage initialProjectPaths={projectPaths} refreshProjects={getAllProjects} />
      ) : page == PageEnum.News ? (
        <NewsPage newsEntries={newsEntries} />
      ) : null}
    </div>
  );
}

export default App;
