<script setup lang="ts">
import {  onClickOutside } from '@vueuse/core'
import { ref, useTemplateRef, toRef, type Ref } from 'vue'
import type { Game } from '@/types/types'
import { useGameSearch_V1, useGameSearch_V2 } from '@/composables/useGameSearch'

interface Props {
  gameDb: Game[]
}

interface Emits {
  (e: 'addGame', game: Game): void
  (e: 'refetch'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// Create a reactive ref for gameDB
const gameDBRef = toRef(props,'gameDb') as Ref<Game[]>

// Use search composable
const { searchQuery, searchResults } = useGameSearch_V2(gameDBRef)

// Search UI state
const searchResultsIsOpen = ref(false)
const isOnSearchResults = ref(false)
const searchResultContainerRef = useTemplateRef<HTMLElement>('searchResultContainerRef')

// Close dropdown when clicking outside
onClickOutside(searchResultContainerRef, () => {
  searchResultsIsOpen.value = false
})

function openSearchResults() {
  searchResultsIsOpen.value = true
}

function closeSearchResults() {
  searchResultsIsOpen.value = false
}

function handleSearchBlur() {
  setTimeout(() => {
    if (!isOnSearchResults.value) {
      searchResultsIsOpen.value = false
    }
  }, 200)
}

function addGameToList(game: Game) {
  emit('addGame', game)
  closeSearchResults()
}

function handleRefetch() {
  emit('refetch')
}
</script>

<template>
  <div class="mb-8">
    <div ref="searchResultContainerRef" class="relative">
      <div>
        <input
          v-model.trim="searchQuery"
          type="text"
          placeholder="Search Discord Verified games..."
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:text-white"
          @focus="openSearchResults"
          @blur="handleSearchBlur"
        />

        <!-- Refetch button -->
        <button
          class="absolute right-0 top-1/2 transform -translate-y-1/2 px-3 mr-2 py-1 text-sm bg-gray-200 dark:bg-gray-600 hover:bg-gray-300 dark:hover:bg-gray-500 text-gray-700 dark:text-white rounded-md"
          @click="handleRefetch"
        >
          <span class="wrap whitespace-nowrap text-xs">
            Refetch Game List
          </span>
        </button>
      </div>

      <!-- Search results dropdown -->
      <div
        v-if="searchResultsIsOpen"
        class="absolute z-50 mt-1 w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg shadow-lg max-h-60 overflow-y-auto"
        @click="isOnSearchResults = true"
      >
        <div v-if="searchResults.length > 0">
          <div
            v-for="game in searchResults"
            :key="game.item.id"
            class="p-3 hover:bg-gray-100 dark:hover:bg-gray-700 border-b border-gray-200 dark:border-gray-700 last:border-b-0"
          >
            <div class="flex justify-between items-center">
              <div>
                <div class="font-medium text-gray-800 dark:text-white">
                  {{ game.item.name }}
                </div>
                <div class="text-sm text-gray-500 dark:text-gray-400">
                  ID: {{ game.item.id }}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  Executables:
                  <ul class="list-disc list-inside">
                    <li
                      v-for="exe in game.item.executables"
                      :key="exe.name"
                      class="text-gray-500 dark:text-gray-400"
                    >
                      <span class="font-mono">
                        {{ exe.name }}
                        ({{ exe.os }})
                      </span>
                    </li>
                  </ul>
                </div>
              </div>
              <button
                class="ml-2 px-3 py-1 text-sm bg-indigo-600 hover:bg-indigo-700 text-white rounded-md"
                @click="addGameToList(game.item)"
              >
                Add game to list
              </button>
            </div>
          </div>
        </div>

        <!-- Help text when no results -->
        <div
          v-if="searchResults.length === 0"
          class="p-3 hover:bg-gray-100 dark:hover:bg-gray-700 border-b border-gray-200 dark:border-gray-700 last:border-b-0 text-gray-500 dark:text-gray-400"
        >
          Search for games by name. <br />
          Click "Add game to list" to add them to your selected games.
        </div>
      </div>
    </div>
  </div>
</template>
