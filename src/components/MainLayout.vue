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
                @click="isAccountModalOpen = true"
                class="flex items-center gap-2.5 px-3.5 py-1.5 rounded-xl bg-[#5865F2] hover:bg-[#4752C4] text-white font-semibold text-xs shadow-md shadow-indigo-500/20 transition-all duration-200 hover:scale-105 active:scale-95 cursor-pointer"
              >
                <img :src="avatarUrl || 'https://cdn.discordapp.com/embed/avatars/0.png'" class="w-5 h-5 rounded-full border border-white/40 shadow-xs" />
                <span>{{ userProfile?.global_name || userProfile?.username || 'Discord Account' }}</span>
                <span v-if="activeUnfinishedQuests.length > 0" class="ml-0.5 px-1.5 py-0.5 bg-emerald-500 text-white rounded-full text-[10px] font-bold">
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