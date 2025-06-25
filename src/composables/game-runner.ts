import { Game, GameExecutable } from "@/types/types";
import { randomString } from "@/utils/random-string";
import { createGlobalState } from "@vueuse/core";
import { ref, shallowRef } from "vue";

const toPathId = (p: string) => p.split(/\\|\//).join(''); 

export const useGameRunnerStore = createGlobalState(() => {
    const gameList = ref<Game[]>([]);
    const selectedGame = ref<Game | null>();

    function addGameToList(game: Game) {
        if (!gameList.value.some(g => g.id === game.id)) {
            gameList.value.push({
                uid: randomString(),
                ...game
            });
        }
    }

    function removeGameFromList(game: Game) {
        const gameId = game.uid;
        gameList.value = gameList.value.filter(game => game.uid !== gameId);
        if (selectedGame.value?.uid === gameId) {
            selectedGame.value = null;
        }
    }

    function canPlayGame(game: Game | null | undefined) {
        if(!game) {
            return false
        }

        return (game.is_installed && !game.is_running) ?? false;
    }

    /**
     * Check if the executable is running
     */
    function isExecutableRunning(executable: GameExecutable) {
        return executable.is_running ?? false;
    }

    /**
     * Check if the executable is installed
     */
    function isGameExecutableInstalled(executable: GameExecutable) {
        return executable.is_installed ?? false;
    }

    function isGameInstalled(game: Game | null) {
        if (!game) {
            return false;
        }
        // we can only play a game if the game is installed and not running
        return game.is_installed ?? false;
    }

    function getGameByUID(uid: string) {
        return gameList.value.find(g => g.uid === uid);
    }

    function getExecutableByName(game: Game | null | undefined, exeName: string) {
        return game?.executables.find(exe => exe.name === exeName);
    }

    function getExecutableByPath(game: Game | null | undefined, execPath: string) {
        return game?.executables.find(exe => toPathId(exe.path!) === toPathId(execPath));
    }

    function getGameByExecutable(execPath: string) {
        console.log('getGameByExecutable', toPathId(execPath))
        return gameList.value.find(g => g.executables.some(exe => {
            console.log(`getGameByExecutable..name`,toPathId(exe.name))
            return toPathId(exe.name) === toPathId(execPath);
        }));
    }

    function updateExecutableRunStatus(execPath: string, runStatus: boolean) {
        const gameToUpdate = getGameByExecutable(execPath);
        console.log('updateExecutableRunStatus.gameToUpdate', gameToUpdate)
        if(gameToUpdate) {
            console.log('updateExecutableRunStatus.execPath', execPath)
            const executable = getExecutableByPath(gameToUpdate, execPath);
            console.log('updateExecutableRunStatus.executable', executable)
            if (executable) {
                executable['is_running'] = runStatus;
            }
        }
    }

    function updateGameByUID(uid: string, data: Partial<Game>) {
        let game = getGameByUID(uid);
        if(game) {
            game = {
                ...game,
                ...data,
            }
        }
    }

    function updateExecutableByName(game: Game, execName: string, data: Partial<GameExecutable>) {
        let execItem = getExecutableByName(game, execName);
        if (execItem) {
            execItem = {
                ...execItem,
                ...data,
            }
        }
    }

    return {
        gameList,
        selectedGame,

        addGameToList,
        removeGameFromList,
        updateExecutableRunStatus,
        updateExecutableByName,
        updateGameByUID,

        canPlayGame,
        isGameExecutableInstalled,
        isExecutableRunning,
        isGameInstalled,

        getGameByUID,
        getGameByExecutable,
        getExecutableByPath,
        getExecutableByName

    }
})