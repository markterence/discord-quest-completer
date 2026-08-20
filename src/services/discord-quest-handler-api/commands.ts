import { invoke } from "@tauri-apps/api/core"; 
import { RunBackgroundProcessPayload } from "./types";


const COMMANDS = {
    runBackgroundProcess (payload: RunBackgroundProcessPayload) {    
        return invoke('launch_executable', payload);
    },
    launchExecutable (payload: RunBackgroundProcessPayload) {
        return invoke('launch_executable', payload);
    },

    stopExecutable (appId: number) {
        return invoke('stop_executable', { app_id: appId.toString() });
    },

    getEmbeddedGamelist(): Promise<string> {
        return invoke('get_embedded_gamelist');
    },

    fetchGameListFromGithubPages() {
        return invoke("fetch_gamelist_gh_mirror")
    },

    fetchGameListFromDiscord() {
        return invoke("fetch_gamelist_from_discord")
    }
}

export default COMMANDS;