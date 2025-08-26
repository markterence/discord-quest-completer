<script setup lang="ts">
import { Pages, useGlobalState } from '@/composables/app-state';
import IconRustLang from './IconRustLang.vue';
import IconVueJs from './IconVueJs.vue';
import { RouterView } from 'vue-router';

// Layout component for consistent page structure

const appState = useGlobalState();
const { page, setPage } = appState;

</script>

<template>
  <div class="flex flex-col h-dvh overflow-hidden bg-gray-100 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow-md">
      <div class="container mx-auto px-4 py-4 flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <img src="/logo.svg" alt="Logo" class="h-8 w-8" />
          <h2 class="text-xl font-bold text-gray-900 dark:text-white"></h2>
        </div>
        <nav>
          <ul class="flex space-x-6">
            <li>
              <a href="#" 
                class="text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400"
                :class="{ 'text-indigo-600 dark:text-indigo-400': page === Pages.HOME }"
                @click.prevent="setPage(Pages.HOME)"
              >
                Home
              </a>
            </li> 
            <li>
              <a href="#" 
                class="text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400"
                :class="{ 'text-indigo-600 dark:text-indigo-400': page === Pages.PLAYGROUND }"
                @click.prevent="setPage(Pages.PLAYGROUND)"
              >
                Playground
              </a>
            </li>
            <li>
              <a href="#" class="text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400"
                :class="{ 'text-indigo-600 dark:text-indigo-400': page === Pages.SETTINGS }"
                @click.prevent="setPage(Pages.SETTINGS)"
              >
                Settings</a>
            </li>
          </ul>
        </nav>
      </div>
    </header>
    
    <main class="flex-grow overflow-y-auto">
      <RouterView />
    </main>
    
    <footer class="bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 mt-auto">
     
      <div class="container mx-auto px-4 py-4 text-center text-sm text-gray-600 dark:text-gray-400">
        &copy; 2025 Built with <IconRustLang class="inline-block h-4 w-4 text-red-200 dark:text-white mx-0.5"/> Rust and <IconVueJs class="h-4 w-4 inline-block text-[#4FC08D] mx-0.5"/> Vue.js
      </div>
    </footer>
  </div>
</template>