<script setup lang="ts">
import { ref, computed } from 'vue';
import gameListData from '../assets/x.json';

interface Game {
  id: string;
  name: string;
  executables: Array<{
    is_launcher: boolean;
    name: string;
    os: string;
  }>;
  aliases?: string[];
  themes?: string[];
}

// Game list from JSON file
const gameList = ref<Game[]>(gameListData);

// Search functionality
const searchQuery = ref('');
const searchResults = computed(() => {
  if (!searchQuery.value) return [];
  const query = searchQuery.value.toLowerCase();
  return gameList.value.filter(game => 
    game.name.toLowerCase().includes(query) || 
    game.aliases?.some(alias => alias.toLowerCase().includes(query))
  );
});

// Selected games list
const selectedGames = ref<Game[]>([]);

// Function to add a game to the selected list
function addGameToList(game: Game) {
  if (!selectedGames.value.some(g => g.id === game.id)) {
    selectedGames.value.push(game);
  }
}

// Function to remove a game from the selected list
function removeGameFromList(gameId: string) {
  selectedGames.value = selectedGames.value.filter(game => game.id !== gameId);
}

// Create a dummy game
function createDummyGame() {
  const dummyId = `dummy-${Date.now()}`;
  const dummyGame: Game = {
    id: dummyId,
    name: "Dummy Game",
    executables: [{
      is_launcher: false,
      name: "dummy.exe",
      os: "win32"
    }]
  };
  selectedGames.value.push(dummyGame);
}

// Game status
const currentlyPlaying = ref<string | null>(null);

// Play game function
async function playGame(gameId: string) {
  try {
    console.log(`Playing game: ${gameId}`);
    currentlyPlaying.value = gameId;
    // In a real app, this would invoke a Tauri command to launch the game
    // await invoke('launch_game', { id: gameId });
  } catch (error) {
    console.error('Failed to launch game:', error);
  }
}

// Stop playing
function stopPlaying() {
  console.log('Stopped playing game');
  currentlyPlaying.value = null;
}

function getExecutables(game: Game) {
  return game.executables.map(exe => exe.name)
}
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-6 text-center">Game Management</h1>
    
    <!-- Search Bar -->
    <div class="mb-8">
      <div class="relative">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search games..."
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:text-white"
        />
        <div v-if="searchResults.length > 0" class="absolute z-10 mt-1 w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg shadow-lg max-h-60 overflow-y-auto">
          <div v-for="game in searchResults" :key="game.id" class="p-3 hover:bg-gray-100 dark:hover:bg-gray-700 border-b border-gray-200 dark:border-gray-700 last:border-b-0">
            <div class="flex justify-between items-center">
              <div>
                <div class="font-medium text-gray-800 dark:text-white">{{ game.name }}</div>
                <div class="text-sm text-gray-500 dark:text-gray-400">ID: {{ game.id }}</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  Executables:
                    <ul class="list-disc list-inside">
                        <li v-for="exe in game.executables" :key="exe.name" class="text-gray-500 dark:text-gray-400">
                        <span class="font-mono">{{ exe.name }} ({{ exe.os }})</span>
                        </li>
                    </ul>
                </div>
              </div>
              <button
                @click="addGameToList(game)"
                class="ml-2 px-3 py-1 text-sm bg-indigo-600 hover:bg-indigo-700 text-white rounded-md"
              >
                Add game to list
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Two-Column Layout -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- Left Column: Selected Games -->
      <div class="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">Selected Games</h2>
        <div v-if="selectedGames.length === 0" class="text-gray-500 dark:text-gray-400 text-center py-8">
          No games selected. Search and add games from the search bar.
        </div>
        <div v-else class="space-y-4">
          <div v-for="game in selectedGames" :key="game.id" class="p-3 border border-gray-200 dark:border-gray-700 rounded-lg">
            <div class="flex justify-between items-center">
              <div>
                <div class="font-medium text-gray-800 dark:text-white">{{ game.name }}</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  {{ game.executables.map(exe => exe.name).join(', ') }}
                </div>
              </div>
              <button
                @click="removeGameFromList(game.id)"
                class="text-red-600 hover:text-red-800"
              >
                Remove
              </button>
            </div>
            <div class="flex space-x-2 mt-2">
              <button
                @click="playGame(game.id)"
                :disabled="currentlyPlaying === game.id"
                :class="[
                  'px-3 py-1 text-sm rounded-md flex-1',
                  currentlyPlaying === game.id 
                    ? 'bg-gray-400 cursor-not-allowed' 
                    : 'bg-green-600 hover:bg-green-700 text-white'
                ]"
              >
                {{ currentlyPlaying === game.id ? 'Playing...' : 'Play' }}
              </button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Right Column: Game Actions -->
      <div class="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">Game Actions</h2>
        <div class="space-y-4">
          <button
            @click="createDummyGame"
            class="w-full py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg"
          >
            Create Dummy Game
          </button>
          
          <button
            @click="stopPlaying"
            :disabled="!currentlyPlaying"
            :class="[
              'w-full py-2 rounded-lg',
              !currentlyPlaying 
                ? 'bg-gray-400 cursor-not-allowed text-gray-200' 
                : 'bg-red-600 hover:bg-red-700 text-white'
            ]"
          >
            Stop Playing
          </button>
        </div>
        
        <div class="mt-6 p-4 border border-gray-200 dark:border-gray-700 rounded-lg">
          <h3 class="font-medium text-gray-800 dark:text-white mb-2">Status</h3>
          <div v-if="currentlyPlaying" class="text-green-600">
            Currently playing: {{ selectedGames.find(g => g.id === currentlyPlaying)?.name }}
          </div>
          <div v-else class="text-gray-500 dark:text-gray-400">
            Not playing any game
          </div>
        </div>
      </div>
    </div>
  </div>
</template>