
export class GodotEngineVersion {
    engineName: string;
    engineVersion: string;
    installationPath: String;
    updatedAt: number;

    constructor(engineName: string, engineVersion: string, installationPath: String, updatedAt: string) {
        this.engineName = engineName;
        this.engineVersion = engineVersion;
        this.installationPath = installationPath;
        this.updatedAt = Date.parse(updatedAt);
    }
}