import { ReactEventHandler, useEffect, useState } from "react";
import { GodotEngineVersion } from "../data/GodotEngineVersion";
import { ProjectData } from "../data/ProjectData";
import styles from "./EnginePage.module.css";
import { invoke } from "@tauri-apps/api";

interface ProjectPageProps {
    installedGodotEngines: GodotEngineVersion[];
    projects: ProjectData[];
}

function ProjectPage(props: ProjectPageProps) {
    const [engines, setEngines] = useState<GodotEngineVersion[]>([]);

    useEffect(() => {
        setEngines(props.installedGodotEngines);
    }, [props.installedGodotEngines]);

    function selectVersion(projectData: ProjectData, event: React.FormEvent<HTMLSelectElement>): void {
        invoke("set_engine_version_for_project", { projectName: projectData.projectName, engineName: event.currentTarget.value })
    }

    function engineVersionDropdown(selectedValue: GodotEngineVersion | null, projectData: ProjectData): JSX.Element {

        return (
            <select defaultValue={selectedValue?.engineName} name="engines" id="engines" onSelect={(evt) => selectVersion(projectData, evt)}>
                {engines.map(engine =>
                    <option value={engine.engineName}>{engine.engineName}</option>
                )}
            </select>
        )
    }

    function findEngineVersion(engineName: string): GodotEngineVersion | null {
        return engines.find(engine => engine.engineName == engineName) ?? null;
    }

    return (
        <div className={styles.widthFull}>
            <div className={styles.width95 + " " + styles.tableContainer}>
                <table cellSpacing={"0"} className={styles.widthFull}>
                    <tr>
                        <th>Last Opened</th>
                        <th>Name</th>
                        <th>Engine Version</th>
                        <th>Button</th>
                    </tr>
                    {props.projects.map(project => (
                        <tr>
                            <td>{(new Date(project.lastOpened).toDateString())}</td>
                            <td>{project.projectName}</td>
                            <td>{engineVersionDropdown(findEngineVersion(project.engineVersion), project)}</td>
                            <td>Button</td>
                        </tr>
                    ))}

                </table>
            </div>
        </div>
    );

}

export default ProjectPage;

