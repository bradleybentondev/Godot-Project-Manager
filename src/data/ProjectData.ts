
export class ProjectData {
    projectName: string;
    path: string;
    lastOpened: number;
    engineVersion: string;
    favorite: boolean;

    constructor(projectName: string, path: string, lastOpened: string, engineVersion: string, favorite: boolean) {
        this.projectName = projectName;
        this.path = path;
        this.lastOpened = Date.parse(lastOpened);
        this.engineVersion = engineVersion;
        this.favorite = favorite;
    }
}