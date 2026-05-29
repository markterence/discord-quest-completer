import { listen } from '@tauri-apps/api/event';
import { EventName, EventPayloadMap } from './types';

export const EVENTS = {
    run_background_process_exited: 'run_background_process_exited',
    background_process_result: 'background_process_result',
} as const satisfies Record<EventName, EventName>;

export function listenToEvent<K extends EventName>(
    eventName: K,
    handler: (event: { payload: EventPayloadMap[K] }) => void
) {
    return listen<EventPayloadMap[K]>(eventName, handler);
}