
export class ProjectData {
    projectName: string;
    path: string;
    lastDateOpened: number;
    engineVersion: string;
    favorite: boolean;
    engineValid: boolean;

    constructor(projectName: string, path: string, lastOpened: string, engineVersion: string, favorite: boolean, engineValid: boolean) {
        this.projectName = projectName;
        this.path = path;
        this.lastDateOpened = Date.parse(lastOpened);
        this.engineVersion = engineVersion;
        this.favorite = favorite;
        this.engineValid = engineValid;
    }
}