import { createGlobalState } from '@vueuse/core'
import { computed, onScopeDispose, ref, type Ref } from 'vue'
import type { Game, GameExecutable } from '@/types/types'
import { invoke } from '@tauri-apps/api/core'
import { path } from '@tauri-apps/api'
import { emit, listen } from '@tauri-apps/api/event'
import { useGameList } from './useGameList'
import { useGlobalState } from './app-state'

export interface UseGameActionsReturn {
  currentlyPlaying: Ref<string | null>
  isConnectedToRPC: Ref<boolean>
  isConnecting: Ref<boolean>
  playGame: (payload: { game: Game; executable: GameExecutable }) => Promise<void>
  stopPlaying: (payload: { game: Game; executable: GameExecutable }) => Promise<void>
  createDummyGame: (game: Game, executable: GameExecutable) => Promise<boolean | undefined>
  installAndPlay: (payload: { game: Game; executable: GameExecutable }) => Promise<void>
  handleTestRPC: (game: Game | null) => Promise<void>
  continueRPCRisk: (game: Game | null) => Promise<void>
  canPlayGame: (game: Game | null) => boolean
  canCreateDummyGame: (game: Game | null) => boolean
  isGameInstalled: (game: Game | null) => boolean
  isExecutableRunning: (executable: GameExecutable) => boolean
  isGameExecutableInstalled: (executable: GameExecutable) => boolean
}

export const useGameActions = createGlobalState(() => {
  // Get game list state
  const { gameList } = useGameList()
  const { addLog } = useGlobalState()
  
  // RPC and game state
  const currentlyPlaying = ref<string | null>(null)
  const isConnectedToRPC = ref(false)
  const isConnecting = ref(false)
  
  // Helper functions
  function canCreateDummyGame(game: Game | null) {
    if (!game) return false
    // we can only create a dummy game if the game is not installed or game is not running
    return !game.is_installed
  }
  
  function canPlayGame(game: Game | null) {
    if (!game) return false
    // we can only play a game if the game is installed and not running
    return (game.is_installed && !game.is_running) ?? false
  }
  
  function isExecutableRunning(executable: GameExecutable) {
    return executable.is_running ?? false
  }
  
  function isGameExecutableInstalled(executable: GameExecutable) {
    return executable.is_installed ?? false
  }
  
  function isGameInstalled(game: Game | null) {
    if (!game) return false
    return game.is_installed ?? false
  }
  
  // Create a dummy game
  async function createDummyGame(game: Game | null, executable: GameExecutable) {
    if (!game) return
    
    const gameUid = game.uid
    const gameToInstall = gameList.value.find(g => g.uid === gameUid)
    const executableItem = gameToInstall?.executables.find(exe => exe.name === executable.name)
    
    if (gameToInstall && executableItem) {
      const payload = {
        path: executable.path,
        executable_name: executable.filename,
        path_len: executable.segments,
        app_id: Number(gameToInstall.id),
      }
      console.log(payload)
      const result = await invoke('create_fake_game', payload)
      console.log('Game created:', result)
      gameToInstall.is_installed = true
      executableItem.is_installed = true
      return true
    }
  }
  
  // Install and play
  async function installAndPlay({ game, executable }: { game: Game; executable: GameExecutable }) {
    if (!game) return
    
    const gameCreated = await createDummyGame(game, executable)
    if (gameCreated) {
      playGame({ game, executable })
    } else {
      console.error('Failed to create game')
      addLog('error', 'Failed to create game')
    }
  }
  
  // Play game function
  async function playGame({ game, executable }: { game: Game; executable: GameExecutable }) {
    if (!game) return
    
    const gameUid = game.uid
    try {
      console.log(`Playing game: ${gameUid}`)
      addLog('info', `Playing game: ${game.name}`)
      addLog('info', `Executable: ${executable.name}`)
      currentlyPlaying.value = game.id
      
      // find the game in the list
      const gameToPlay = gameList.value.find(g => g.uid === gameUid)
      const executableItem = gameToPlay?.executables.find(exe => exe.name === executable.name)
      
      if (gameToPlay && executableItem) {
        const payload = {
          name: game.name,
          path: executable.path,
          executable_name: executable.filename,
          path_len: executable.segments,
          app_id: Number(gameToPlay.id),
          exec_path: path.join(executable.path!, executable.filename!),
        }
        await invoke('run_background_process', payload)
        gameToPlay.is_running = true
        executableItem.is_running = true
      }
    } catch (error) {
      console.error('Failed to launch game:', error)
    }
  }
  
  // Stop playing
  async function stopPlaying({ game, executable }: { game: Game; executable: GameExecutable }) {
    if (!game) return
    
    console.log('Stopped playing game')
    const gameUid = game.uid
    
    currentlyPlaying.value = null
    
    const gameToPlay = gameList.value.find(g => g.uid === gameUid)
    const executableItem = gameToPlay?.executables.find(exe => exe.name === executable.name)
    
    if (gameToPlay && executableItem) {
      try {
        await invoke('stop_process', {
          exec_name: executable.filename!,
        })
        addLog('info', `Stopped game process: ${game.name}`)
        addLog('info', `Stopped Executable: ${executable.name}`)
      } catch (error) {
        console.error('Failed to stop game process:', error)
        const errorMessage = error instanceof Error ? error.message : String(error)
        addLog('error', 'Failed to stop game process' + errorMessage)
        // Even if stopping fails, we still update the state
        gameToPlay.is_running = false
        executableItem.is_running = false
      } finally {
        gameToPlay.is_running = false
        executableItem.is_running = false
      }
    }
  }
  
  // Handle Test RPC
  async function handleTestRPC(game: Game | null) {
    const state = isConnectedToRPC.value ? 'disconnect' : 'connect'
    
    console.log('Testing RPC for game:', game)
    
    if (!game && state === 'connect') {
      // Caller should show dialog for 'no_game_selected'
      throw new Error('no_game_selected')
    }
    
    if (state === 'disconnect' || isConnecting.value) {
      emit('event_disconnect')
      
      isConnectedToRPC.value = false
      if (game) {
        game.is_running = false
      }
      currentlyPlaying.value = null
      isConnecting.value = false
      return
    }
    
    // Caller should show dialog for 'rpc_message_1'
    throw new Error('rpc_message_1')
  }
  
  // Continue RPC Risk
  async function continueRPCRisk(game: Game | null) {
    if (!game) return
    
    const gameUid = game.uid
    const gameToTest = gameList.value.find(g => g.uid === gameUid)
    
    if (gameToTest) {
      console.log('Testing RPC for game:', gameToTest)
      isConnecting.value = true
      
      invoke('connect_to_discord_rpc_3', {
        activity_json: JSON.stringify({
          app_id: gameToTest.id,
        }),
        action: 'connect',
      }).then(() => {
        isConnectedToRPC.value = true
        gameToTest.is_running = true
        currentlyPlaying.value = gameToTest.id
        isConnecting.value = false
      })
    }
  }
  
  // Setup Tauri event listeners
  const unlistenFns: (() => void)[] = []
  
  // Listen for background process result events
  listen('background_process_result', (event: any) => {
    if (!event.payload) {
      console.error('No payload received from background process exit event')
      return
    }
    console.log('Background process status:', event.payload)
    const payload = event.payload
    
    // Update executable running status based on full_executable_path
    gameList.value.forEach(game => {
      game.executables.forEach(exe => {
        const fullPath = path.join(exe.path || '', exe.filename || '')
        if (fullPath === payload.full_executable_path) {
          exe.is_running = payload.running
        }
      })
    })
  }).then(unlisten => {
    unlistenFns.push(unlisten)
  })
  
  // Clean up on scope dispose
  onScopeDispose(() => {
    unlistenFns.forEach(fn => fn())
  })
  
  return {
    currentlyPlaying,
    isConnectedToRPC,
    isConnecting,
    playGame,
    stopPlaying,
    createDummyGame,
    installAndPlay,
    handleTestRPC,
    continueRPCRisk,
    canPlayGame,
    canCreateDummyGame,
    isGameInstalled,
    isExecutableRunning,
    isGameExecutableInstalled,
  } as UseGameActionsReturn
})
