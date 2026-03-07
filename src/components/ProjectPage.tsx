import { GodotEngineVersion } from "../data/GodotEngineVersion";
import { ProjectData } from "../data/ProjectData";
import { Box, Button, Typography } from "@mui/material";
import ProjectTable from "./projectpage/ProjectTable";
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from "@tauri-apps/api/core"
import { appDataDir } from '@tauri-apps/api/path';

interface ProjectPageProps {
    installedGodotEngines: GodotEngineVersion[];
    allProjects: ProjectData[];
    setAllProjects: (projects: ProjectData[]) => void;
    setProjectEngineVersion: (projectName: string, engineName: string) => void;
    refreshProjects: () => void
}

function ProjectPage(props: ProjectPageProps) {
    // const [projects, setProjects] = useState(props.allProjects);

     async function openDialog() {
            const selected = await open({
                directory: true,
                multiple: false,
                defaultPath: await appDataDir(),
            });
    
            if (selected) {
                invoke<string[]>("save_project_path", { projectDirectory: selected as string }).then(paths => {
                    props.refreshProjects();
                })
            }
        }

    return (
        props.allProjects.length > 0 ?
            <ProjectTable
                setProjectEngineVersion={props.setProjectEngineVersion}
                installedGodotEngines={props.installedGodotEngines}
                allProjects={props.allProjects}
                setAllProjects={props.setAllProjects}
            />
            :
            <Box sx={{ fontStyle: 'italic', height: '100%', display: 'flex', alignItems: 'center', justifyContent: 'center', flexDirection: 'column', gap: 2 }}>
                <Typography component={'div'} variant={'h6'} sx={{ textAlign: 'center', color: 'rgba(255, 255, 255, 0.5)' }}>No projects found</Typography>
                <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', flexDirection: 'row', gap: 2 }}>
                    <Typography component={'div'} variant={'h6'} sx={{ textAlign: 'center', color: 'rgba(255, 255, 255, 0.5)' }}>Try</Typography>
                    <Button variant={'outlined'} onClick={() => openDialog()}>adding</Button>
                    <Typography component={'div'} variant={'h6'} sx={{ textAlign: 'center', color: 'rgba(255, 255, 255, 0.5)' }}>a project path</Typography>
                </Box>

            </Box>

    );

}

export default ProjectPage;

