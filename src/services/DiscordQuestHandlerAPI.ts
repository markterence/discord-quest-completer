import { invoke } from '@tauri-apps/api/core';

const EVENTS = {
    run_background_process_exited: 'run_background_process_exited',
    background_process_result: 'background_process_result',
} as const;

export type RunBackgroundProcessResultEvent = {
    app_id: string;
    executable_name: string;
    full_executable_path: string; 
    pid: number;
    status: number;
    running: boolean;
    error?: string | null;
}
export type RunBackgroundProcessPayload = {
    path?: string;
    executable_name?: string;
    path_len?: Number | undefined;
    app_id: number;
    exec_path: string;
}

const DiscordQuestHandlerAPI = { 
    EVENTS,
    runBackgroundProcess (payload: RunBackgroundProcessPayload) {    
        return invoke('run_background_process', payload);
    }
}
export default DiscordQuestHandlerAPI;