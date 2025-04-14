import { Game } from "@/types/types";
import { InjectionKey } from "vue";

export const EXECUTABLE_OS = {
    WINDOWS: 'win32',
    DARWIN: 'darwin',
    LINUX: 'linux',
    ANDROID: 'android',
    IOS: 'ios',
} as const;

export const GameActionsKey = Symbol() as InjectionKey<string>;