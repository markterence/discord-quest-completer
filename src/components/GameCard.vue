<script setup lang="ts">
const props = defineProps<{
  title: string;
  description?: string;
  imageUrl?: string;
  launchId?: string;
}>();

const emit = defineEmits(['launch']);

function launchGame() {
  emit('launch', props.launchId);
}
</script>

<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden transition-transform hover:scale-105 hover:shadow-lg">
    <div class="relative h-48 bg-gray-200 dark:bg-gray-700">
      <img v-if="imageUrl" :src="imageUrl" alt="Game cover" class="w-full h-full object-cover" />
      <div v-else class="flex items-center justify-center w-full h-full">
        <span class="text-gray-400 dark:text-gray-500 text-lg">No image</span>
      </div>
    </div>
    <div class="p-4">
      <h3 class="text-lg font-semibold text-gray-800 dark:text-white mb-2">{{ title }}</h3>
      <p v-if="description" class="text-sm text-gray-600 dark:text-gray-300 mb-4">{{ description }}</p>
      <button 
        @click="launchGame" 
        class="w-full bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 px-4 rounded transition-colors"
      >
        Launch
      </button>
    </div>
  </div>
</template>