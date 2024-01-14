
export class ProjectData {
    projectName: string;
    path: string;
    lastOpened: number;
    engineVersion: String;
    favorite: boolean;

    constructor(projectName: string, path: string, lastOpened: string, engineVersion: String, favorite: boolean){
        this.projectName = projectName;
        this.path = path;
        this.lastOpened = Date.parse(lastOpened);
        this.engineVersion = engineVersion;
        this.favorite = favorite;
    }
}