import { GodotEngineVersion } from "../data/GodotEngineVersion";
import { ProjectData } from "../data/ProjectData";
import styles from "../css-modules/EnginePage.module.css";
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import { IconButton } from "@mui/material";
import { invoke } from "@tauri-apps/api";

interface ProjectPageProps {
    installedGodotEngines: GodotEngineVersion[];
    allProjects: ProjectData[];
    setProjectEngineVersion: (projectName: string, engineName: string) => void;
}

function ProjectPage(props: ProjectPageProps) {

    console.log("All projects", props.allProjects)

    function selectVersion(projectData: ProjectData, event: React.FormEvent<HTMLSelectElement>): void {
        props.setProjectEngineVersion(projectData.projectName, event.currentTarget.value)
    }

    function formatDate(lastDateOpened: number): string {
        console.log("last date opened", lastDateOpened);
        if (lastDateOpened > 0) {
            return new Date(lastDateOpened).toDateString()
        } else {
            return "N/A"
        }
    }

    function engineVersionDropdown(selectedValue: GodotEngineVersion | null, projectData: ProjectData): JSX.Element {
        return (
            <select defaultValue={selectedValue?.engineName} name="engines" id="engines" onChange={(evt) => selectVersion(projectData, evt)}>
                <option value="NA" key={"NA"}>N/A</option>
                {props.installedGodotEngines.map(engine =>
                    <option value={engine.engineName} key={engine.engineName}>{engine.engineName}</option>
                )}
            </select>
        )
    }

    function findEngineVersion(engineName: string): GodotEngineVersion | null {
        return props.installedGodotEngines.find(engine => engine.engineName == engineName) ?? null;
    }

    function launch(project: ProjectData) {
        invoke("open_project", { projectName: project.projectName });
    }

    return (
        <div className={styles.widthFull}>
            <div className={styles.width95 + " " + styles.tableContainer}>
                <table cellSpacing={"0"} className={styles.widthFull}>
                    <thead>
                        <tr>
                            <th>Last Opened</th>
                            <th>Name</th>
                            <th>Engine Version</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        {props.allProjects.sort((a, b) => b.lastDateOpened - a.lastDateOpened).map(project => (
                            <tr key={project.projectName}>
                                <td>{formatDate(project.lastDateOpened)}</td>
                                <td>{project.projectName}</td>
                                <td>{engineVersionDropdown(findEngineVersion(project.engineVersion), project)}</td>
                                <td>
                                    {project.engineValid ?
                                        <IconButton onClick={() => launch(project)}>
                                            <PlayArrowIcon color="primary" />
                                        </IconButton>
                                        : null}
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );

}

export default ProjectPage;

