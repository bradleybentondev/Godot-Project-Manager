import { PageEnum } from "../data/PageEnum";
import { ProjectData } from "../data/ProjectData";
import styles from "../css-modules/SideBar.module.css"

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
        <div className={styles.navButton} onClick={() => props.setPage(PageEnum.Projects)}>
          <span> Projects </span>
          <span className={styles.countBox}>{props.projectCount}</span>
        </div>
        <div className={styles.navButton} onClick={() => props.setPage(PageEnum.Engines)}>
          <span >Versions</span>
          <span className={styles.countBox}>{props.engineCount}</span>
        </div>
        <div className={styles.navButton} onClick={() => props.setPage(PageEnum.News)}>
          <span >News</span>
          <span className={styles.countBox}>{props.newsCount}</span>
        </div>
        <div className={styles.navButton} onClick={() => props.setPage(PageEnum.Settings)}>
          <span >Settings</span>
        </div>
      </nav>

      {/* <div className={styles.mostRecents}>
        {props.projects.sort((p1, p2) => p1.lastDateOpened - p2.lastDateOpened).slice(0, Math.min(3, props.projects.length)).map(data =>
          <div key={data.projectName}>{data.projectName}</div>
        )}

      </div> */}

    </div>
  );
}

export default SideBar;
