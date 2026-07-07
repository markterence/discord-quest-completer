<script setup lang="ts">
import type { Game } from '@/types/types'
import IconVerified from '@/components/IconVerified.vue'

interface Props {
  games: Game[]
  selectedGameId: string | null
}

interface Emits {
  (e: 'select', game: Game): void
  (e: 'remove', game: Game): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

function selectGame(game: Game) {
  emit('select', game)
}

function removeGame(game: Game) {
  emit('remove', game)
}
</script>

<template>
  <div class="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
    <h2
      class="text-xl font-bold text-gray-900 dark:text-white mb-4 sticky top-0 bg-white dark:bg-gray-800 py-2 z-10"
    >
      Games
    </h2>

    <!-- Empty state -->
    <div
      v-if="games.length === 0"
      class="text-gray-500 dark:text-gray-400 text-center py-8"
    >
      No games selected. Search and add games from the search bar.
    </div>

    <!-- Games list -->
    <div v-else class="space-y-4">
      <div
        v-for="game in games"
        :key="game.uid"
        class="p-3 border border-gray-200 dark:border-gray-700 rounded-lg
        hover:bg-gray-100 dark:hover:bg-gray-700/50 transition-colors 
        duration-200 ease-in-out cursor-pointer"
        :class="{
          'ring-1 ring-violet-500/40 shadow-[0px_0px_8px_2px_#8e51ff50] bg-gray-100 dark:bg-gray-700/40':
            selectedGameId === game.uid,
        }"
        @click="selectGame(game)"
      >
        <div class="flex justify-between items-center">
          <div class="flex items-center gap-1">
            <div class="font-medium text-gray-800 dark:text-white">
              {{ game.name }}
            </div>
            <div class="relative inline-flex items-center">
              <div
                class="w-2 h-2 bg-white absolute rounded-full"
                style="left: 50%; top: 50%; transform: translate(-50%, -50%)"
              ></div>
              <div class="relative inline-block">
                <IconVerified class="w-5 h-5 text-indigo-500 dark:text-indigo-400" />
              </div>
            </div>
          </div>
          <button
            v-if="!game.is_running"
            class="text-red-300 hover:text-red-400"
            @click.stop="removeGame(game)"
          >
            Remove
          </button>
        </div>
        <div class="flex space-x-2 mt-2">
          <div v-if="game.is_running" class="text-sm text-green-500 dark:text-green-400">
            Running
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

