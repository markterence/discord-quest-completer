<template>
    <div class="text-gray-500 dark:text-gray-400">
        <p v-if="isMac" class="mb-3 text-xs text-gray-500 dark:text-gray-400">
            A small game window opens while playing. Discord on macOS usually only detects apps with a visible window.
        </p>

        <h3 v-if="filteredExecutables.length > 0">
            Select an executable to launch:
        </h3>

        <p v-if="usingCrossPlatformFallback" class="text-xs mt-2 text-amber-600 dark:text-amber-400">
            No macOS executable is registered for this game. Using the Windows executable name as a fallback.
        </p>

        <p v-if="filteredExecutables.length === 0" class="text-sm mt-2 text-yellow-500">
            Either Discord has not registered any launchable executables for this game.
            or there are no executables available for your platform ({{ currentPlatform }})
        </p>

        <div class="text-xs mt-2">
            <div v-for="(executable) in filteredExecutables" :key="executable.name"
                class="grid grid-cols-[auto_1fr_auto] gap-2 items-center mb-2 w-full">
                <div class="w-14 max-w-[80px]">
                    <div class="bg-gray-200 dark:bg-gray-700 rounded-full px-2 py-1 w-fit">
                        {{ executable.os }}
                    </div>
                </div>

                <!-- Sections / Breadcrumbs must fade when too long -->
                <div class="relative overflow-hidden ">
                    <div class="flex flex-nowrap overflow-x-auto scrollbar-none max-w-full pr-4 fade-right">
                        <div v-for="(section, i) in splitExecutableName(executable)" :key="i"
                            class="text-center border border-gray-300 dark:border-gray-700 rounded-md px-2 py-1 mr-1 whitespace-nowrap">
                            <span>{{ section }}</span>
                        </div>
                    </div>
                </div>

                <div class="justify-self-end">
                    <button class="text-white rounded-md px-3 py-1"
                    :class="[
                        {
                            'bg-blue-500 hover:bg-blue-600': !gameActions?.isExecutableRunning(executable),
                            'bg-red-500 hover:bg-red-600': gameActions?.isExecutableRunning(executable),
                        },
                    ]"
                        @click="handleLaunch(executable)"
                    >
                        {{ gameActions?.isExecutableRunning(executable) ? 'Stop' : 'Play' }}
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { EXECUTABLE_OS, GameActionsKey, getCurrentOS, isMacOS } from '@/constants/constants';
import { GameActionsProvider, type Game, type GameExecutable } from '@/types/types';
import { path } from '@tauri-apps/api';
import { computed, inject } from 'vue';

const props = defineProps<{
    game: Game
}>();

const emit = defineEmits<{
    play: [{game: Game, executable: GameExecutable}]
    stop: [{game: Game, executable: GameExecutable}]
    install_and_play: [{game: Game, executable: GameExecutable}]
}>();

const gameActions = inject<GameActionsProvider>(GameActionsKey);

function isValidPath(name: string) {
    const illegalChars = ['>', '<', ':', '"', '|', '?', '*'];
    return !illegalChars.some(char => name.includes(char));
}

const validExecutables = computed(() =>
    props.game.executables.filter(executable => isValidPath(executable.name))
);

const currentPlatform = getCurrentOS();

const isMac = isMacOS();

const filteredExecutables = computed(() => {
    const platformMatches = validExecutables.value.filter(
        executable => executable.os === currentPlatform
    );

    if (platformMatches.length > 0) {
        return platformMatches;
    }

    return validExecutables.value;
});

const usingCrossPlatformFallback = computed(() => {
    if (validExecutables.value.length === 0) {
        return false;
    }

    return !validExecutables.value.some(executable => executable.os === currentPlatform);
    // return props.game.executables.filter(executable => {
    //     // Filter for current platform only
    //     return executable.os === currentPlatform && !isValidPath(executable.name);
    // });
});

function splitExecutableName(executable: GameExecutable) {
    const allSections = executable.name.split(/\\|\//);
    
    const last = executable.name.split(/\\|\//).pop();
    // remove file extension if there was none, just return the last section
    const name = last?.split('.').slice(0, -1).join('.') || last;
    return [
        ...allSections.slice(0, -1),
        name,
    ];
}

function getExecutablePath(executable: GameExecutable) {
    const allSections = executable.name.split(/\\|\//);
    const last = executable.name.split(/\\|\//).pop();
    // remove file extension if there was none, just return the last section
    const name = last?.split('.').slice(0, -1).join('.') || last;
    return [
        ...allSections.slice(0, -1)
    ].join(path.sep())
}

function getFilename(executable: GameExecutable) {
    const last = executable.name.split(/\\|\//).pop();
    // remove file extension if there was none, just return the last section
    return last;
}

function handleLaunch(executable: GameExecutable) {
    // Handle the launch logic here
    console.log('Launching game:', props.game);
    if(executable.is_running) {
        emit('stop', {
            game: props.game,
            executable: {
                path: getExecutablePath(executable),
                segments: splitExecutableName(executable).length,
                filename: getFilename(executable),
                ...executable
            },
        });
    } else {
        if (!gameActions?.isGameExecutableInstalled(executable)) {
            emit('install_and_play', {
                game: props.game,
                executable: {
                    path: getExecutablePath(executable),
                    segments: splitExecutableName(executable).length,
                    filename: getFilename(executable),
                    ...executable
                },
            });
        } else {
            emit('play', {
                game: props.game,
                executable: {
                    path: getExecutablePath(executable),
                    segments: splitExecutableName(executable).length,
                    filename: getFilename(executable),
                    ...executable
                },
            });
        }
     
    }
    
}

</script>

<style scoped>
.fade-right {
    -webkit-mask-image: linear-gradient(to right, black 85%, transparent 100%);
    mask-image: linear-gradient(to right, black 85%, transparent 100%);
}

.scrollbar-none {
    scrollbar-width: none;
    -ms-overflow-style: none;
}

.scrollbar-none::-webkit-scrollbar {
    display: none;
}
</style>
