import { GodotEngineVersion } from "../data/GodotEngineVersion";
import { ProjectData } from "../data/ProjectData";
import styles from "../css-modules/ProjectPage.module.css";
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import { IconButton } from "@mui/material";
import { invoke } from "@tauri-apps/api/core"
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';

interface ProjectPageProps {
    installedGodotEngines: GodotEngineVersion[];
    allProjects: ProjectData[];
    setProjectEngineVersion: (projectName: string, engineName: string) => void;
}

function ProjectPage(props: ProjectPageProps) {
    // const [projects, setProjects] = useState(props.allProjects);


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
            <div className={styles.dropdown} >
                <select defaultValue={selectedValue?.engineName} name="engines" id="engines" onChange={(evt) => selectVersion(projectData, evt)}>
                    <option value="NA" key={"NA"}>N/A</option>
                    {props.installedGodotEngines.map(engine =>
                        <option value={engine.engineName} key={engine.engineName}>{engine.engineName}</option>
                    )}
                </select>
                <ArrowDropDownIcon className={styles.dropdownIcon} />
            </div >
        )
    }

    function findEngineVersion(engineName: string): GodotEngineVersion | null {
        return props.installedGodotEngines.find(engine => engine.engineName == engineName) ?? null;
    }

    async function launch(project: ProjectData) {
        const [name, time] = await invoke<[string, number]>("open_project", { projectName: project.projectName });
        // let p = projects.find(p => p.projectName === name);
        // if (p) {
        //     p.lastDateOpened = time;
        // }
        // setProjects([...projects]);
    }

    return (
        <div className={styles.widthFull}>
            <div className={styles.tableContainer}>
                <table cellSpacing="0" className={styles.table}>
                    <thead>
                        <tr>
                            <th>Name</th>
                            <th>Last Opened</th>
                            <th>Engine Version</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        {props.allProjects.sort((a, b) => b.lastDateOpened - a.lastDateOpened).map(project => (
                            <tr key={project.projectName} className={styles.tableRow}>
                                <td>
                                    <span className={styles.bold}>{project.projectName}</span><br />
                                    <span className={styles.projectPath}>{project.projectPath}</span>
                                </td>
                                <td>{formatDate(project.lastDateOpened)}</td>
                                <td>{engineVersionDropdown(findEngineVersion(project.engineVersion), project)}</td>
                                <td>
                                    {project.engineValid && (
                                        <IconButton onClick={() => launch(project)}>
                                            <PlayArrowIcon className={styles.iconButton} />
                                        </IconButton>
                                    )}
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>

            {/* Create a floating 'new project' button that will always be anchored to the bottom right of the page */}
            <div className={styles.newProjectButtonContainer}>
                <button className={styles.newProjectButton}>New Project</button>
            </div>
        </div>
    );

}

export default ProjectPage;

