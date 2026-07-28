<script setup lang="ts">
import { Pages, useGlobalState } from '@/composables/app-state';
import { useUserQuests } from '@/composables/use-user-quests';
import IconRustLang from './IconRustLang.vue';
import IconVueJs from './IconVueJs.vue';

// Layout component for consistent page structure

const appState = useGlobalState();
const { page, setPage } = appState;
const { isAccountModalOpen, token, activeUnfinishedQuests, autoDetectLocalToken, userProfile, avatarUrl } = useUserQuests();

</script>

<template>
  <div class="flex flex-col h-dvh overflow-hidden bg-gray-100 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow-md">
      <div class="container mx-auto px-4 py-3 flex items-center justify-between">
        <div class="flex items-center space-x-3">
          <img src="/logo.svg" alt="Logo" class="h-8 w-8" />
          <h2 class="text-lg font-bold text-gray-900 dark:text-white">Discord Quest Completer</h2>
        </div>
        <nav>
          <ul class="flex items-center space-x-5">
            <li>
              <a href="#" 
                class="text-sm font-medium text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors"
                :class="{ 'text-indigo-600 dark:text-indigo-400 font-semibold': page === Pages.HOME }"
                @click.prevent="setPage(Pages.HOME)"
              >
                Home
              </a>
            </li> 
            <li>
              <a href="#" 
                class="text-sm font-medium text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors"
                :class="{ 'text-indigo-600 dark:text-indigo-400 font-semibold': page === Pages.PLAYGROUND }"
                @click.prevent="setPage(Pages.PLAYGROUND)"
              >
                Playground
              </a>
            </li>
            <li>
              <button
                @click="token ? (isAccountModalOpen = true) : autoDetectLocalToken()"
                class="flex items-center gap-2 px-3.5 py-1.5 rounded-xl bg-[#5865F2] hover:bg-[#4752C4] text-white font-semibold text-xs shadow-md shadow-indigo-500/20 transition-all duration-200 hover:scale-105 active:scale-95 cursor-pointer"
              >
                <img v-if="token && avatarUrl" :src="avatarUrl" class="w-5 h-5 rounded-full border border-white/40" />
                <svg v-else class="w-4 h-4 fill-current" viewBox="0 0 127.14 96.36">
                  <path d="M107.7,8.07A105.15,105.15,0,0,0,81.47,0a72.06,72.06,0,0,0-3.36,6.83A97.68,97.68,0,0,0,49,6.83,72.37,72.37,0,0,0,45.64,0,105.89,105.89,0,0,0,19.39,8.09C2.79,32.65-1.71,56.6.54,80.21h0A105.73,105.73,0,0,0,32.71,96.36,77.7,77.7,0,0,0,39.6,85.25a68.42,68.42,0,0,1-10.85-5.18c.91-.66,1.8-1.34,2.66-2a75.57,75.57,0,0,0,64.32,0c.87.71,1.76,1.39,2.66,2a68.68,68.68,0,0,1-10.87,5.19,77,77,0,0,0,6.89,11.1A105.25,105.25,0,0,0,126.6,80.22c2.7-27.42-4.81-51.17-18.9-72.15ZM42.45,65.69C36.18,65.69,31,60,31,53s5-12.74,11.43-12.74S54,45.91,53.87,53,48.8,65.69,42.45,65.69Zm42.24,0C78.41,65.69,73.25,60,73.25,53s5-12.74,11.44-12.74S96.23,45.91,96.1,53,91.08,65.69,84.69,65.69Z"/>
                </svg>
                <span>{{ token ? (userProfile?.global_name || userProfile?.username || 'Discord Account') : 'Sync Discord App' }}</span>
                <span v-if="token && activeUnfinishedQuests.length > 0" class="ml-1 px-1.5 py-0.5 bg-emerald-500 text-white rounded-full text-[10px] font-bold">
                  {{ activeUnfinishedQuests.length }}
                </span>
              </button>
            </li>
          </ul>
        </nav>
      </div>
    </header>
    
    <main class="flex-grow overflow-y-auto">
      <slot></slot>
    </main>
    
    <footer class="bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 mt-auto">
      <div class="container mx-auto px-4 py-4 text-center text-sm text-gray-600 dark:text-gray-400">
        &copy; 2025 Built with <IconRustLang class="inline-block h-4 w-4 text-red-200 dark:text-white mx-0.5"/> Rust and <IconVueJs class="h-4 w-4 inline-block text-[#4FC08D] mx-0.5"/> Vue.js
      </div>
    </footer>
  </div>
</template>