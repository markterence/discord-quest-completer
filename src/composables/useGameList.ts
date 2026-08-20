import { createGlobalState, useStorage, watchOnce } from '@vueuse/core'
import { computed, ref, watch, type Ref } from 'vue'
import type { Game, GameExecutable } from '@/types/types'
import { randomString } from '@/utils/random-string'
import { APP_SETTINGS_KEYS } from '@/constants/storage-keys'

export interface UseGameListReturn {
  gameList: Ref<Game[]>
  selectedGameId: Ref<string | null>
  selectedGame: Ref<Game | null>
  addGame: (game: Game) => void
  removeGame: (game: Game) => void
  selectGame: (game: Game | null) => void,
  getExecutableByName: (game: Game | null | undefined, exeName: string) => GameExecutable | undefined,
  getExecutableByPath: (game: Game | null | undefined, execPath: string) => GameExecutable | undefined,
  initializeWithGameDB: (gameDB: Ref<Game[]>, allFetchDone: Ref<boolean>) => void
  updateExecutableRunStatus: (execPath: string, runStatus: boolean) => void
}

const toPathId = (p: string) => p.split(/\\|\//).join('');

export const useGameList = createGlobalState(() => {
  // Persistent storage for game IDs
  const savedGameIds = useStorage<string[]>(APP_SETTINGS_KEYS.savedGameIds, [])

  // Game list state
  const gameList = ref<Game[]>([])
  const selectedGameId = ref<string | null>(null)

  // Computed selected game
  const selectedGame = computed(() => {
    if (!selectedGameId.value) return null
    return gameList.value.find(g => g.uid === selectedGameId.value) || null
  })

  // Add game to list
  function addGame(game: Game) {
    // Check for duplicates by game ID
    if (gameList.value.some(g => g.id === game.id)) {
      return
    }

    // Add game with unique UID
    gameList.value.push({
      uid: randomString(),
      ...game
    })
  }

  // Remove game from list
  function removeGame(game: Game) {
    const gameUid = game.uid
    gameList.value = gameList.value.filter(g => g.uid !== gameUid)

    // Clear selection if removing selected game
    if (selectedGameId.value === gameUid) {
      selectedGameId.value = null
    }
  }

  function getExecutableByName(game: Game | null | undefined, exeName: string) {
    return game?.executables.find(exe => exe.name === exeName);
  }

  function getGameByExecutable(execPath: string) {
    console.log('getGameByExecutable', toPathId(execPath))
    return gameList.value.find(g => g.executables.some(exe => {
      console.log(`getGameByExecutable..name`, toPathId(exe.name))
      return toPathId(exe.name) === toPathId(execPath);
    }));
  }

  function getExecutableByPath(game: Game | null | undefined, execPath: string) {
    console.log('getExecutableByPath', {
      execPath,
      toPathId: toPathId(execPath),
      game,
    })
    const gameExec = game?.executables.find(exe => toPathId(exe.name!) === toPathId(execPath));

    return gameExec;
  }
  function updateExecutableRunStatus(execPath: string, runStatus: boolean) {
    const gameToUpdate = getGameByExecutable(execPath);
    console.log('updateExecutableRunStatus.gameToUpdate', gameToUpdate)
    if (gameToUpdate) {
      try {
        console.log('updateExecutableRunStatus.execPath', execPath)
        const executable = getExecutableByPath(gameToUpdate, execPath);
        console.log('updateExecutableRunStatus.executable', executable)
        if (executable) {
          executable['is_running'] = runStatus;
        }
        console.log('executable after update', executable)
      } catch (error) {
        console.error('Error updating executable run status:', error);
      }
    }
  }

  // Select a game
  function selectGame(game: Game | null) {
    selectedGameId.value = game?.uid || null
  }

  // Restore games from storage
  function restoreFromStorage(gameDB: Game[]) {
    // If no saved games, add default dev game
    if (savedGameIds.value.length === 0) {
      savedGameIds.value = ['1328876348361412619'] // Default dev game
    }

    // Restore games from saved IDs
    savedGameIds.value.forEach(id => {
      const game = gameDB.find(g => g.id === id)
      if (game) {
        addGame(game)
      }
    })
  }

  // Initialize with gameDB and setup auto-restore
  function initializeWithGameDB(gameDB: Ref<Game[]>, allFetchDone: Ref<boolean>) {
    // Watch for gameDB to be ready
    watchOnce(allFetchDone, (isDone) => {
      if (isDone && gameDB.value.length > 0) {
        restoreFromStorage(gameDB.value)
      }
    })
  }



  // Auto-save game IDs when game list changes
  watch(
    gameList,
    (newGameList) => {
      savedGameIds.value = newGameList.map(g => g.id)
    },
    { deep: true }
  )

  return {
    gameList,
    selectedGameId,
    selectedGame,
    addGame,
    removeGame,
    selectGame,
    initializeWithGameDB,
    getExecutableByName,
    getExecutableByPath,
    updateExecutableRunStatus
  } as UseGameListReturn
})
