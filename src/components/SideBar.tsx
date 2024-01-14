import { PageEnum } from "../data/PageEnum";
import { ProjectData } from "../data/ProjectData";
import styles from "./SideBar.module.css"

interface SideBarProps {
  setPage: React.Dispatch<React.SetStateAction<PageEnum>>;
  projectCount: number;
  engineCount: number;
  newsCount: number;
}

function SideBar(props: SideBarProps) {

  function testData(): ProjectData[] {
    return [
      new ProjectData("Some project", "test", "2023-08-19", "4.1.2 Mono", false),
      new ProjectData("Some project 2", "test2", "2023-05-19", "3.1.2", false),
      new ProjectData("Some project 3", "test3", "2023-07-19", "4.2.2", false),
      new ProjectData("Some project 4", "test4", "2023-06-19", "3.3.2 Mono", false),
    ].sort((a, b) => b.lastOpened - a.lastOpened);
  }

  return (
    <div className={styles.container}>
      <div>
        <h3>Godot Manager</h3>
      </div>

      <nav className={styles.nav}>
        <div>
          <a href="">Projects</a>
          <span className={styles.countBox}>{props.projectCount}</span>
        </div>
        <div>
          <a href="">Versions</a>
          <span className={styles.countBox}>{props.engineCount}</span>
        </div>
        <div>
          <a href="">News</a>
          <span className={styles.countBox}>{props.newsCount}</span>
        </div>
      </nav>

      <div className={styles.mostRecents}>
        {testData().map(data =>
          <div>{data.projectName}</div>
        )}

      </div>

    </div>
  );
}

export default SideBar;
