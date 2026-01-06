import { ReactNode, useEffect, useState } from "react";
import { GodotEngineVersion } from "../data/GodotEngineVersion";
import styles from "../css-modules/EnginePage.module.css";
import DownloadIcon from '@mui/icons-material/Download';
import DeleteForeverIcon from '@mui/icons-material/DeleteForever';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import { IconButton } from "@mui/material";
import { invoke } from "@tauri-apps/api/core"

interface EnginePageProps {
    allGodotEngines: GodotEngineVersion[];
    installedGodotEngines: GodotEngineVersion[];
    downloadEngineFunc: (engineName: string) => void;
    deleteVersion: (engineName: string) => void
}

function EnginePage(props: EnginePageProps) {
    const [availableEngines, setAvailableEngines] = useState<GodotEngineVersion[]>([]);
    const [downloadStatusList, setDownloadStatusList] = useState<[string, number][]>([]);

    useEffect(() => {
        let availableEngines = props.allGodotEngines.filter(engine => props.installedGodotEngines.find(installedEngine => {
            return engine.engineName === installedEngine.engineName
        }) === undefined);

        setAvailableEngines(availableEngines);

    }, [props.allGodotEngines, props.installedGodotEngines]);

    useEffect(() => {
        let timer = setInterval(() => {
            invoke<[string, number][]>("poll_download_status_list").then(data => setDownloadStatusList(data));
        }, 100);

        // this will clear Timeout
        // when component unmount like in willComponentUnmount
        // and show will not change to true
        return () => {
            clearTimeout(timer);
        };
    }, []);

    function downloadStatusOrButton(engine: GodotEngineVersion): ReactNode {
        let status = downloadStatusList.find(value => value[0] === engine.engineName);
        if (status) {
            return <progress className={styles.progressBar} value={status[1] / 100} />
        } else {
            return (
                <IconButton onClick={() => props.downloadEngineFunc(engine.engineName)}>
                    <DownloadIcon color={"primary"} />
                </IconButton>
            )
        }
    }

    function launch(engineName: string) {
        invoke("open_engine", { engineName: engineName });
    }

    function table(engines: GodotEngineVersion[], buttonType: "install" | "delete"): ReactNode {

        function getButton(engine: GodotEngineVersion) {
            return buttonType === "install" ? (
                <div>
                    <IconButton onClick={() => props.deleteVersion(engine.engineName)}>
                        <DeleteForeverIcon color={"error"} />
                    </IconButton>
                    <IconButton onClick={() => launch(engine.engineName)}>
                        <PlayArrowIcon color="primary" />
                    </IconButton>
                </div>

            ) : downloadStatusOrButton(engine)
        }


        return <table cellSpacing={"0"} className={styles.widthFull}>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Version</th>
                    <th>Date Uploaded</th>
                    <th style={{ width: "100px" }}></th>
                </tr>
            </thead>
            <tbody>
                {engines.map((engine, idx) => (
                    <tr key={idx}>
                        <td>{engine.engineName}</td>
                        <td>{engine.engineVersion}</td>
                        <td>{new Date(engine.updatedAt).toDateString()}</td>
                        <td>
                            {getButton(engine)}
                        </td>
                    </tr>
                ))}
            </tbody>

        </table>
    }


    return (
        <div style={{ display: "flex", flexDirection: "column", width: "100%" }}>
            {props.installedGodotEngines.length > 0 ?
                <div className={styles.widthFull}>
                    <div className={styles.width95 + " " + styles.tableContainer}>
                        {table(props.installedGodotEngines, "install")}
                    </div>
                </div>
                : null}

            <div className={styles.widthFull}>
                <div className={styles.width95 + " " + styles.tableContainer}>
                    {table(availableEngines, "delete")}
                </div>
            </div>
        </div>
    );

}

export default EnginePage;

