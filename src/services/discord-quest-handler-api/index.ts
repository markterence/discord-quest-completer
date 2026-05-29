import COMMANDS from "./commands";
import { EVENTS } from "./events";

export const DiscordQuestHandlerAPI = {
    EVENTS,
    ...COMMANDS,
}

export * from "./types";