
export const EXECUTABLE_OS = {
    WINDOWS: 'win32',
    DARWIN: 'darwin',
    LINUX: 'linux',
    ANDROID: 'android',
    IOS: 'ios',
} as const;

export const GAME_LIST_SOURCE = {
    DISCORD: 'discord' as const,
    MARKTERENCE_GH_MIRROR: 'markterence_gh_mirror' as const,
} as const;