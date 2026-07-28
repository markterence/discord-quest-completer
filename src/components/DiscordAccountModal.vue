<template>
  <Transition
    enter-active-class="transition opacity-100 duration-200"
    leave-active-class="transition opacity-0 duration-150"
  >
    <div
      v-if="isOpen"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-xs p-4"
    >
      <div
        class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl shadow-2xl w-full max-w-lg overflow-hidden flex flex-col max-h-[90vh]"
      >
        <!-- Header -->
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center bg-gray-50 dark:bg-gray-800/80">
          <div class="flex items-center gap-2">
            <span class="text-xl">🌐</span>
            <h3 class="text-lg font-bold text-gray-900 dark:text-white">
              Discord Account Quests
            </h3>
          </div>
          <button
            @click="close"
            class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 text-xl font-bold p-1 rounded-lg cursor-pointer"
          >
            ✕
          </button>
        </div>

        <!-- Content -->
        <div class="p-6 overflow-y-auto space-y-5 flex-1">
          <!-- Main System Browser Login Action -->
          <div class="text-center p-4 bg-indigo-500/5 dark:bg-indigo-500/10 border border-indigo-500/20 rounded-xl space-y-3">
            <p class="text-xs text-gray-600 dark:text-gray-300">
              Open Discord in your <strong>default system browser (Chrome/Edge/Firefox)</strong> for maximum trust & security.
            </p>

            <div class="flex flex-col gap-2">
              <button
                @click="openDefaultBrowser('https://discord.com/login')"
                class="w-full py-2.5 px-4 bg-[#5865F2] hover:bg-[#4752C4] text-white font-semibold text-xs rounded-lg shadow-md transition-all flex items-center justify-center gap-2 cursor-pointer"
              >
                <span class="text-base">🌐</span>
                <span>Open Discord in System Browser (Chrome / Edge)</span>
              </button>

              <button
                @click="openLoginWindow"
                :disabled="isLoading"
                class="w-full py-2 px-4 bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-800 dark:text-gray-200 font-medium text-xs rounded-lg transition-colors flex items-center justify-center gap-2 cursor-pointer"
              >
                <span v-if="isLoading" class="animate-spin">🌀</span>
                <span class="text-sm">🪟</span>
                <span>{{ isLoading ? 'Waiting for Login...' : 'Or Open Webview Popup (Auto-Capture)' }}</span>
              </button>
            </div>

            <p v-if="token" class="text-xs text-emerald-600 dark:text-emerald-400 font-medium">
              ✓ Session Token active
            </p>
          </div>

          <!-- Token Input Section -->
          <div class="space-y-2">
            <label class="block text-xs font-semibold text-gray-700 dark:text-gray-300">
              Session Token / Fetch Quests
            </label>
            <div class="flex gap-2">
              <input
                v-model="inputToken"
                type="password"
                placeholder="Paste Token here to sync active quests..."
                class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg text-xs bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-indigo-500 focus:outline-hidden"
              />
              <button
                @click="handleSaveAndFetch"
                :disabled="isLoading || !inputToken"
                class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 text-white font-semibold text-xs rounded-lg transition-colors flex items-center gap-1.5 cursor-pointer"
              >
                <span v-if="isLoading" class="animate-spin">🌀</span>
                <span>{{ isLoading ? 'Syncing...' : 'Fetch Quests' }}</span>
              </button>
            </div>

            <!-- Quick Copy Script Helper -->
            <div class="flex items-center justify-between text-[11px] text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-700/40 p-2 rounded-lg border border-gray-200 dark:border-gray-600/50">
              <span>💡 Need token from browser? Copy 1-line script for DevTools (F12):</span>
              <button
                @click="copyDevToolsScript"
                class="ml-2 px-2 py-1 bg-gray-200 dark:bg-gray-600 hover:bg-gray-300 dark:hover:bg-gray-500 text-gray-800 dark:text-gray-200 font-medium rounded text-[10px] transition-colors whitespace-nowrap cursor-pointer"
              >
                {{ scriptCopied ? 'Copied! ✓' : 'Copy Script 📋' }}
              </button>
            </div>
          </div>

          <!-- Error Alert -->
          <div v-if="errorMessage" class="p-3 bg-red-500/10 border border-red-500/30 text-red-500 dark:text-red-400 text-xs rounded-lg">
            ⚠️ {{ errorMessage }}
          </div>

          <!-- Quests List -->
          <div v-if="quests.length > 0" class="space-y-3">
            <div class="flex justify-between items-center">
              <h4 class="text-sm font-semibold text-gray-800 dark:text-gray-200">
                Active Quests ({{ activeUnfinishedQuests.length }} unfinished)
              </h4>
              <button
                @click="emitSync"
                class="text-xs bg-emerald-600 hover:bg-emerald-700 text-white font-medium px-3 py-1.5 rounded-md transition-colors cursor-pointer"
              >
                ⚡ Auto-Add Games To List
              </button>
            </div>

            <div class="space-y-2 max-h-60 overflow-y-auto pr-1">
              <div
                v-for="quest in quests"
                :key="quest.id"
                class="p-3 border rounded-lg text-xs bg-gray-50 dark:bg-gray-700/40 border-gray-200 dark:border-gray-700 flex flex-col gap-1.5"
              >
                <div class="flex justify-between items-start">
                  <div>
                    <span class="font-bold text-gray-900 dark:text-white">
                      {{ quest.config?.messages?.game_title || quest.config?.application_name || 'Game Quest' }}
                    </span>
                    <p class="text-gray-500 dark:text-gray-400 text-[11px]">
                      {{ quest.config?.messages?.quest_name || quest.config?.title || 'Quest' }}
                    </p>
                  </div>

                  <span
                    :class="[
                      'px-2 py-0.5 rounded-full text-[10px] font-semibold',
                      isCompleted(quest) 
                        ? 'bg-green-500/20 text-green-600 dark:text-green-400' 
                        : 'bg-amber-500/20 text-amber-600 dark:text-amber-400'
                    ]"
                  >
                    {{ isCompleted(quest) ? 'Completed ✓' : 'In Progress ⏳' }}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div class="px-6 py-3 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/80 flex justify-end gap-2">
          <button
            @click="close"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 text-sm font-medium rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors cursor-pointer"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useUserQuests } from '@/composables/use-user-quests';
import type { DiscordQuest } from '@/types/types';

const props = defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'sync-games', appIds: string[]): void;
}>();

const {
  token,
  quests,
  isLoading,
  errorMessage,
  setToken,
  openLoginWindow,
  openDefaultBrowser,
  fetchQuests,
  activeUnfinishedQuests,
  unfinishedGameAppIds,
} = useUserQuests();

const inputToken = ref(token.value);
const scriptCopied = ref(false);

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    inputToken.value = token.value;
    if (token.value && quests.value.length === 0) {
      fetchQuests();
    }
  }
});

function close() {
  emit('close');
}

async function handleSaveAndFetch() {
  setToken(inputToken.value);
  const result = await fetchQuests();
  if (result.length > 0) {
    emitSync();
  }
}

function emitSync() {
  emit('sync-games', unfinishedGameAppIds.value);
}

function copyDevToolsScript() {
  const script = `window.webpackChunkdiscord_app.push([[Math.random()],{},e=>{for(const c of Object.values(e.c))if(c?.exports?.default?.getToken)console.log(c.exports.default.getToken())}])`;
  navigator.clipboard.writeText(script);
  scriptCopied.value = true;
  setTimeout(() => {
    scriptCopied.value = false;
  }, 2500);
}

function isCompleted(quest: DiscordQuest): boolean {
  return Boolean(quest.user_status?.completed_at || quest.user_status?.claimed_at);
}
</script>
