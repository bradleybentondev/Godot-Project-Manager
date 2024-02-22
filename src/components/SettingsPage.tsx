import { useEffect, useState } from "react";
import styles from "../css-modules/SettingsPage.module.css"
import AddIcon from '@mui/icons-material/Add';
import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api";
import { appDataDir } from '@tauri-apps/api/path';
import RemoveCircleOutlineIcon from '@mui/icons-material/RemoveCircleOutline';

interface SettingsPageProps {
    initialProjectPaths: string[]
    refreshProjects: () => void
}

function SettingsPage(props: SettingsPageProps) {
    const [projectPaths, setProjectPaths] = useState<string[]>([])

    useEffect(() => {
        if (props.initialProjectPaths) {
            setProjectPaths(props.initialProjectPaths)
        }
    }, [props.initialProjectPaths])

    async function openDialog() {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: await appDataDir(),
        });

        if (selected) {
            console.log("selected", selected);
            invoke<string[]>("save_project_path", { projectDirectory: selected as string }).then(paths => {
                setProjectPaths(paths)
                props.refreshProjects();
            })
        }
    }

    function removeProjectDirectory(projectPath: string) {
        invoke<string[]>("remove_project_path", { projectDirectory: projectPath }).then(response => setProjectPaths(response));
    }

    return (
        <div>
            <div>
                <div className={styles.flex}>
                    <h3>Project Paths</h3>
                    <button onClick={() => openDialog()}><AddIcon /></button>
                </div>

                {projectPaths.length > 0 ?
                    <ul>
                        {projectPaths.map(path =>
                            <li>
                                {path}
                                <button onClick={() => removeProjectDirectory(path)}>
                                    <RemoveCircleOutlineIcon />
                                </button>
                            </li>
                        )}
                    </ul>
                    : null}
            </div>

        </div>
    );

}

export default SettingsPage;

