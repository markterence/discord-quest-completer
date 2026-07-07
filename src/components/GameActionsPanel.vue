<script setup lang="ts">
import type { Game, GameExecutable } from '@/types/types'
import GameExecutables from '@/components/GameExecutables.vue'

interface Props {
  selectedGame: Game | null
  isConnectedToRPC: boolean
  isConnecting: boolean
  currentlyPlaying: string | null
  games: Game[]
}

interface Emits {
  (e: 'testRPC', game: Game | null): void
  (e: 'play', payload: { game: Game; executable: GameExecutable }): void
  (e: 'stop', payload: { game: Game; executable: GameExecutable }): void
  (e: 'installAndPlay', payload: { game: Game; executable: GameExecutable }): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

function handleTestRPC() {
  emit('testRPC', props.selectedGame)
}

function handlePlay(payload: { game: Game; executable: GameExecutable }) {
  emit('play', payload)
}

function handleStop(payload: { game: Game; executable: GameExecutable }) {
  emit('stop', payload)
}

function handleInstallAndPlay(payload: { game: Game; executable: GameExecutable }) {
  emit('installAndPlay', payload)
}
</script>

<template>
  <div class="bg-white dark:bg-gray-800 p-4 rounded-lg shadow md:sticky md:top-4 self-start">
    <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">Game Actions</h2>

    <div class="space-y-4">
      <!-- No game selected message -->
      <div
        v-if="!selectedGame || selectedGame === null"
        class="text-gray-500 dark:text-gray-400 mb-2 text-sm"
      >
        Select a game from the left to perform actions.
      </div>

      <!-- Game info -->
      <div v-if="selectedGame" class="text-gray-500 dark:text-gray-400 mb-4 text-sm">
        <strong>Name:</strong> {{ selectedGame.name }}<br />
        <strong>ID:</strong> {{ selectedGame.id }}<br />
        <strong v-if="selectedGame.aliases && selectedGame.aliases.length > 0">Aliases:</strong>
        <ul v-if="selectedGame.aliases && selectedGame.aliases.length > 0" class="list-disc list-inside">
          <li
            v-for="alias in selectedGame.aliases"
            :key="alias"
            class="text-gray-500 dark:text-gray-400"
          >
            <span class="font-mono">{{ alias }}</span>
          </li>
        </ul>
      </div>

      <!-- Test RPC button -->
      <button
        class="w-full py-2 rounded-lg bg-gray-600 hover:bg-gray-700 text-white"
        @click="handleTestRPC"
      >
        {{ isConnecting || isConnectedToRPC ? 'Disconnect to Discord Gateway' : 'Test RPC' }}
      </button>

      <!-- Divider -->
      <div class="border-t border-gray-200 dark:border-gray-700 my-4"></div>

      <!-- Game executables -->
      <GameExecutables
        v-if="selectedGame"
        :game="selectedGame"
        @play="handlePlay"
        @stop="handleStop"
        @install_and_play="handleInstallAndPlay"
      />
    </div>

    <!-- Divider -->
    <div class="border-t border-gray-200 dark:border-gray-700 my-5"></div>

    <!-- Status section -->
    <div class="mt-6 p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
      <h3 class="font-medium text-gray-800 dark:text-white mb-2">Status</h3>
      <div class="text-sm text-gray-500 dark:text-gray-400 mb-2">
        Check Discord to see if it displays that you are playing a game.
      </div>
      <div v-if="currentlyPlaying" class="text-gray-500 dark:text-gray-400">
        Currently playing: 
        <span class="text-green-600">
          {{ games.find(g => g.id === currentlyPlaying)?.name }}
        </span>
      </div>
      <div v-else class="text-gray-500 dark:text-gray-400">
        Not playing any game
      </div>
    </div>
  </div>
</template>
