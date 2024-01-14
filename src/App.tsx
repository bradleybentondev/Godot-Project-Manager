import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import styles from "./App.module.css";
import SideBar from "./components/SideBar";
import { PageEnum } from "./data/PageEnum";

function App() {
  const [page, setPage] = useState(PageEnum.Projects);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  }

  return (
    <div className={styles.mainContainer}>
      <SideBar setPage={setPage} projectCount={6} engineCount={32} newsCount={12} />

      {page == PageEnum.Projects ? (
        <div></div>
      ) : page == PageEnum.Engines ? (
        <div></div>
      ) : (
        <div></div>
      )}
    </div>
  );
}

export default App;
