import { useEffect, useState } from "react";
import { GodotEngineVersion } from "../data/GodotEngineVersion";
import styles from "./EnginePage.module.css";

interface EnginePageProps {
    allGodotEngines: GodotEngineVersion[];
    installedGodotEngines: GodotEngineVersion[];
    downloadEngineFunc: (engineName: string) => void;
    deleteVersion: (engineName: string) => void
}

function EnginePage(props: EnginePageProps) {
    const [availableEngines, setAvailableEngines] = useState<GodotEngineVersion[]>([]);

    useEffect(() => {
        let availableEngines = props.allGodotEngines.filter(engine => props.installedGodotEngines.find(installedEngine => {
            return engine.engineName === installedEngine.engineName
        }) === undefined);

        setAvailableEngines(availableEngines);
        console.log("availableEngines", availableEngines);
        console.log("installed", props.installedGodotEngines);

    }, [props.allGodotEngines, props.installedGodotEngines]);

    return (
        <div style={{ display: "flex", flexDirection: "column" }}>
            {props.installedGodotEngines.length > 0 ?
                <div className={styles.widthFull}>
                    <div className={styles.width95 + " " + styles.tableContainer}>
                        <table cellSpacing={"0"} className={styles.widthFull}>
                            <tr>
                                <th>Name</th>
                                <th>Version</th>
                                <th>Date Uploaded</th>
                                <th>DButton</th>
                            </tr>
                            {props.installedGodotEngines.map(engine => (
                                <tr>
                                    <td>{engine.engineName}</td>
                                    <td>{engine.engineVersion}</td>
                                    <td>{new Date(engine.updatedAt).toDateString()}</td>
                                    <td><button onClick={() => props.deleteVersion(engine.engineName)}>Delete</button></td>
                                </tr>
                            ))}

                        </table>
                    </div>
                </div>
                : null}

            <div className={styles.widthFull}>
                <div className={styles.width95 + " " + styles.tableContainer}>
                    <table cellSpacing={"0"} className={styles.widthFull}>
                        <tr>
                            <th>Name</th>
                            <th>Version</th>
                            <th>Date Uploaded</th>
                            <th>DButton</th>
                        </tr>
                        {availableEngines.map(engine => (
                            <tr>
                                <td>{engine.engineName}</td>
                                <td>{engine.engineVersion}</td>
                                <td>{new Date(engine.updatedAt).toDateString()}</td>
                                <td><button onClick={() => props.downloadEngineFunc(engine.engineName)}>Download</button></td>
                            </tr>
                        ))}

                    </table>
                </div>
            </div>
        </div>
    );

}

export default EnginePage;

