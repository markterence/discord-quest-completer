<template>
    <div class="text-gray-500 dark:text-gray-400">
        <h3>
            The game has multiple platform executables. Please select one to launch:
        </h3>

        <div v-if="filteredExecutables.length === 0" class="text-sm mt-2 text-yellow-500">
            No executables available for your platform ({{ currentPlatform }})
        </div>

        <div class="text-xs mt-2">
            <div v-for="(executable) in filteredExecutables" :key="executable.name"
                class="grid grid-cols-[auto_1fr_auto] gap-2 items-center mb-2 w-full">
                <div class="w-14 max-w-[80px]">
                    <div class="bg-gray-200 dark:bg-gray-700 rounded-full px-2 py-1 w-fit">
                        {{ executable.os }}
                    </div>
                </div>

                <!-- Sections / Breadcrumbs must fade when too long -->
                <div class="relative overflow-hidden" :title="executable.name">
                    <div class="flex flex-nowrap overflow-x-auto scrollbar-none max-w-full pr-4 fade-right">
                        <!-- <div v-for="(section, i) in splitExecutableName(executable)" :key="i"
                            class="text-center border border-gray-300 dark:border-gray-700 rounded-md px-2 py-1 mr-1 whitespace-nowrap">
                            <span>{{ section }}</span>
                        </div> -->
                        <div v-for="(section, i) in splitExecutableName(executable)" :key="i"
                            class="flex items-center whitespace-nowrap">
                            <span 
                                v-if="i  > 0" 
                            >/</span>
                            <span class="px-0.5 py-0.5 whitespace-nowrap">{{ section }}</span>
                             
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
import { EXECUTABLE_OS, GameActionsKey, getCurrentOS } from '@/constants/constants';
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

const currentPlatform = getCurrentOS();

const filteredExecutables = computed(() => {
    return props.game.executables.filter(executable => {
        // Filter for current platform only
        return executable.os === currentPlatform && !isValidPath(executable.name);
    });
});

function splitExecutableName(executable: GameExecutable) {
    const allSections = executable.name.split(/\\|\//);
    return allSections;
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

function isValidPath(path: string) {
    const illegalChars = ['>', '<', ':', '"', '|', '?', '*'];
    return illegalChars.some(char => path.includes(char));
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

.thin-scrollbar {
    scrollbar-width: thin;
    scrollbar-color: rgba(255, 255, 255, 0.2) transparent;
}

.thin-scrollbar::-webkit-scrollbar {
    width: 3px;
    height: 3px;
}

.thin-scrollbar::-webkit-scrollbar-track {
    background: transparent;
}

.thin-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
    border-radius: 9999px;
}

.thin-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.35);
}
</style>