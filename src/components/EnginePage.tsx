import { useEffect, useState } from "react";
import { GodotEngineVersion } from "../data/GodotEngineVersion";
import { ProjectData } from "../data/ProjectData";
import styles from "./EnginePage.module.css";

interface EnginePageProps {
    allGodotEngines: GodotEngineVersion[];
    installedGodotEngines: GodotEngineVersion[];
    projects: ProjectData[];
}

function EnginePage(props: EnginePageProps) {
    const [engines, setEngines] = useState<GodotEngineVersion[]>([]);

    useEffect(() => {
        setEngines(props.allGodotEngines);
    }, [props.allGodotEngines, props.installedGodotEngines]);

    function engineVersionDropdown(selectedValue: GodotEngineVersion | null): JSX.Element {

        return (
            <select name="engines" id="engines">
                {engines.map(engine =>
                    <option selected={selectedValue?.engineName === engine.engineName} value={engine.engineName}>{engine.engineName}</option>
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
                            <td>{engineVersionDropdown(findEngineVersion(project.engineVersion))}</td>
                            <td>Button</td>
                        </tr>
                    ))}

                </table>
            </div>
        </div>
    );

}

export default EnginePage;

