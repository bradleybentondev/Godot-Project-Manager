import { GodotEngineVersion } from "./GodotEngineVersion";

export class GodotEngineResponse {
    allVersions: GodotEngineVersion[];
    installedVersions: GodotEngineVersion[];

    constructor(allVersions: GodotEngineVersion[], installedVersions: GodotEngineVersion[]) {
        this.allVersions = allVersions;
        this.installedVersions = installedVersions;
    }
}