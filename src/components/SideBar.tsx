import { PageEnum } from "../data/PageEnum";
import { ProjectData } from "../data/ProjectData";
import styles from "./SideBar.module.css"

interface SideBarProps {
  setPage: React.Dispatch<React.SetStateAction<PageEnum>>;
  projectCount: number;
  engineCount: number;
  newsCount: number;
  projects: ProjectData[];
}

function SideBar(props: SideBarProps) {

  return (
    <div className={styles.container}>
      <div>
        <h3>Godot Manager</h3>
      </div>

      <nav className={styles.nav}>
        <div>
          <button onClick={() => props.setPage(PageEnum.Projects)}>Projects</button>
          <span className={styles.countBox}>{props.projectCount}</span>
        </div>
        <div>
          <button onClick={() => props.setPage(PageEnum.Engines)}>Versions</button>
          <span className={styles.countBox}>{props.engineCount}</span>
        </div>
        <div>
          <button onClick={() => props.setPage(PageEnum.News)}>News</button>
          <span className={styles.countBox}>{props.newsCount}</span>
        </div>
      </nav>

      <div className={styles.mostRecents}>
        {props.projects.map(data =>
          <div key={data.projectName}>{data.projectName}</div>
        )}

      </div>

    </div>
  );
}

export default SideBar;
