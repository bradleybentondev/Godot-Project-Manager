import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { GodotEngineVersion } from "../../data/GodotEngineVersion";
import { ProjectData } from "../../data/ProjectData";
import styles from "../../css-modules/ProjectPage.module.css";
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
import { Dialog, DialogContent, DialogTitle, IconButton, List, ListItem, ListItemButton, ListItemText } from "@mui/material";
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import { useState } from "react";

interface ProjectTableProps {
    setProjectEngineVersion: (projectName: string, engineName: string) => void;
    installedGodotEngines: GodotEngineVersion[];
    allProjects: ProjectData[];
    setAllProjects: (projects: ProjectData[]) => void;
}

export default function ProjectTable(props: ProjectTableProps) {
    const [openDialog, setOpenDialog] = useState(false);

    function findEngineVersion(engineName: string): GodotEngineVersion | null {
        return props.installedGodotEngines.find(engine => engine.engineName == engineName) ?? null;
    }

    async function launch(project: ProjectData) {
        const [name, time] = await invoke<[string, number]>("open_project", { projectName: project.projectName });
        let p = props.allProjects.find(p => p.projectName === name);
        if (p) {
            p.lastDateOpened = time;
        }
        props.setAllProjects([...props.allProjects]);
    }

    function getImagePath(project: ProjectData): string {
        const indexLastSlah = project.projectPath.lastIndexOf("\\");
        const path = project.projectPath.substring(0, indexLastSlah);
        return convertFileSrc(path + "\\icon.png");
    }

    function getOtherImagePath(project: ProjectData): string {
        const indexLastSlah = project.projectPath.lastIndexOf("\\");
        const path = project.projectPath.substring(0, indexLastSlah);
        return convertFileSrc(path + "\\icon.svg");
    }

    function launchEngine(engineName: string) {
        invoke("open_engine", { engineName: engineName });
    }

    function selectVersion(projectData: ProjectData, event: React.FormEvent<HTMLSelectElement>): void {
        props.setProjectEngineVersion(projectData.projectName, event.currentTarget.value)
    }

    function formatDate(lastDateOpened: number): string {
        if (lastDateOpened > 0) {
            const date = new Date(lastDateOpened);
            const mm = String(date.getMonth() + 1).padStart(2, '0'); // Months are zero-based
            const dd = String(date.getDate()).padStart(2, '0');
            const yyyy = date.getFullYear();

            return `${mm}/${dd}/${yyyy}`;
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

    return <div className={styles.widthFull}>
        <div className={styles.tableContainer}>
            <table cellSpacing="0" cellPadding="0" className={styles.table}>
                <thead>
                    <tr>
                        <th></th>
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
                                <img className={styles.image} src={getImagePath(project)} onError={(e) => e.currentTarget.src = getOtherImagePath(project)} />
                            </td>
                            <td>
                                <span className={styles.bold}>{project.projectName}</span><br />
                                <span className={styles.projectPath}>{project.projectPath}</span>
                            </td>
                            <td>{formatDate(project.lastDateOpened)}</td>
                            <td>{engineVersionDropdown(findEngineVersion(project.engineVersion), project)}</td>
                            <td>
                                {project.engineValid && (
                                    <IconButton className={styles.outerIconButton} onClick={() => launch(project)}>
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
            <button className={styles.newProjectButton} onClick={() => setOpenDialog(true)}>New Project</button>
        </div>

        <Dialog open={openDialog} onClose={() => setOpenDialog(false)} sx={{
            '& .MuiDialog-paper': {
                backgroundColor: 'var(--background-dark)',
                color: 'var(--text-main)',
            },
        }}>
            <DialogTitle>Choose Engine Version for New Project</DialogTitle>
            <DialogContent>
                <List>
                    {props.installedGodotEngines.map(engine =>
                        <ListItem key={engine.engineName} sx={{ '&:hover': { backgroundColor: 'var(--background-dark-hover)' } }}>
                            <ListItemButton onClick={() => { launchEngine(engine.engineName); setOpenDialog(false) }}>
                                <ListItemText primary={engine.engineName} />
                            </ListItemButton>
                        </ListItem>
                    )}
                </List>
            </DialogContent>
        </Dialog>
    </div>
}