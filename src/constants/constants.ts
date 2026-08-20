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

// Helper to get current OS
export function getCurrentOS(): string {
    // Detect OS from user agent or platform
    const platform = (window as any).__TAURI_INTERNALS__?.platform || navigator.platform.toLowerCase();
    
    if (platform.includes('linux')) return EXECUTABLE_OS.LINUX;
    if (platform.includes('win')) return EXECUTABLE_OS.WINDOWS;
    if (platform.includes('mac')) return EXECUTABLE_OS.DARWIN;
    
    return EXECUTABLE_OS.LINUX; // Default fallback
}

export function isMacOS(): boolean {
    return getCurrentOS() === EXECUTABLE_OS.DARWIN;
}