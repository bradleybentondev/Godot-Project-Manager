import { useEffect, useState } from "react";
import styles from "../css-modules/SettingsPage.module.css"
import AddIcon from '@mui/icons-material/Add';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from "@tauri-apps/api/core"
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
        <div className={styles.settingsContainer}>
            <h1 className={styles.settingsTitle}>Settings</h1>

            <section className={styles.settingsSection}>
                <h2 className={styles.sectionTitle}>Project Paths</h2>
                <div className={styles.projectPathsContainer}>
                    <div className={styles.addPathRow}>
                        <button className={styles.addButton} onClick={() => openDialog()}>
                            <AddIcon /> Add Project Path
                        </button>
                    </div>

                    {projectPaths.length > 0 ? (
                        <ul className={styles.pathList}>
                            {projectPaths.map((path, index) => (
                                <li key={index} className={styles.pathItem}>
                                    <span className={styles.pathText}>{path}</span>
                                    <button
                                        className={styles.removeButton}
                                        onClick={() => removeProjectDirectory(path)}
                                    >
                                        <RemoveCircleOutlineIcon />
                                    </button>
                                </li>
                            ))}
                        </ul>
                    ) : (
                        <p className={styles.noPathsMessage}>No project paths added yet.</p>
                    )}
                </div>
            </section>
        </div>
    );

}

export default SettingsPage;

