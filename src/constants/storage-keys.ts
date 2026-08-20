/**
 * Local Storage prefix
 */
export const APP_STORAGE_PREFIX = 'markterence.discord-quest-completer' as const;

/**
 * Local Storage keys for app settings
 */
export const APP_SETTINGS_KEYS = { 
    gameListUrl: `${APP_STORAGE_PREFIX}.gameListUrl` as const, 
    executableSource: `${APP_STORAGE_PREFIX}.executableSource` as const,
    gameListSource: `${APP_STORAGE_PREFIX}.gameListSource` as const,
    theme: `${APP_STORAGE_PREFIX}.theme` as const,
    savedGameIds: `${APP_STORAGE_PREFIX}.savedGameIds` as const,
} as const;