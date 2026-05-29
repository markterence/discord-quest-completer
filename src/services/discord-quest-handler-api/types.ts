// Commands Payloads
export type RunBackgroundProcessPayload = {
    path?: string;
    executable_name?: string;
    path_len?: Number | undefined;
    app_id: number;
    exec_path: string;
}

// Events
export type RunBackgroundProcessResultEvent = {
    app_id: string;
    executable_name: string;
    full_executable_path: string; 
    pid: number;
    status: number;
    running: boolean;
    error?: string | null;
}

export type EventPayloadMap = {
    run_background_process_exited: RunBackgroundProcessResultEvent;
    background_process_result: RunBackgroundProcessResultEvent;
}

export type EventName = keyof EventPayloadMap;
