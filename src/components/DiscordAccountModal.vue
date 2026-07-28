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
          <!-- Main Login Actions -->
          <div class="text-center p-5 bg-indigo-500/5 dark:bg-indigo-500/10 border border-indigo-500/20 rounded-xl space-y-4">
            <!-- Account Profile Badge when logged in -->
            <div v-if="token && userProfile" class="flex items-center gap-3 p-3 bg-white dark:bg-gray-700/60 border border-gray-200 dark:border-gray-600/60 rounded-xl shadow-xs">
              <img :src="avatarUrl" class="w-11 h-11 rounded-full border-2 border-indigo-500 shadow-sm" />
              <div class="flex-1 text-left">
                <div class="font-bold text-sm text-gray-900 dark:text-white">
                  {{ userProfile.global_name || userProfile.username }}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  @{{ userProfile.username }}
                </div>
              </div>
              <button
                @click="handleLogout"
                class="text-xs bg-red-500/10 hover:bg-red-500/20 text-red-600 dark:text-red-400 px-3 py-1.5 rounded-lg font-semibold transition-colors cursor-pointer"
              >
                Đổi tài khoản
              </button>
            </div>

            <div v-else class="flex flex-col gap-2.5">
              <p class="text-xs text-gray-600 dark:text-gray-300">
                Đăng nhập tài khoản Discord bằng Cửa sổ Trình duyệt Web (hỗ trợ Mật khẩu, 2FA & QR Code). Phiên đăng nhập sẽ được lưu tự động!
              </p>

              <button
                @click="openLoginWindow"
                :disabled="isLoading"
                class="w-full py-3 px-4 bg-[#5865F2] hover:bg-[#4752C4] disabled:opacity-50 text-white font-bold text-xs rounded-xl shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all flex items-center justify-center gap-2 cursor-pointer"
              >
                <span v-if="isLoading" class="animate-spin">🌀</span>
                <span class="text-base">🌐</span>
                <span>{{ isLoading ? 'Đang đợi đăng nhập...' : 'Đăng nhập tài khoản Discord (Web Window)' }}</span>
              </button>

              <button
                @click="autoDetectLocalToken"
                :disabled="isLoading"
                class="w-full py-2.5 px-4 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-50 text-white font-semibold text-xs rounded-lg shadow-md transition-colors flex items-center justify-center gap-2 cursor-pointer"
              >
                <span class="text-sm">⚡</span>
                <span>Hoặc Nhận diện tự động phiên Discord trên máy</span>
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
                Active Quests ({{ activeUnfinishedQuests.length }} chưa hoàn thành)
              </h4>
              <button
                @click="emitSync"
                class="text-xs bg-emerald-600 hover:bg-emerald-700 text-white font-medium px-3 py-1.5 rounded-md transition-colors cursor-pointer"
              >
                ⚡ Thêm tự động Game vào danh sách
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
                      {{ getQuestGameTitle(quest) }}
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
                    {{ isCompleted(quest) ? 'Đã xong ✓' : 'Đang làm ⏳' }}
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
            Đóng
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { watch } from 'vue';
import { useUserQuests, getQuestGameTitle } from '@/composables/use-user-quests';
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
  userProfile,
  avatarUrl,
  isLoading,
  errorMessage,
  setToken,
  openLoginWindow,
  autoDetectLocalToken,
  fetchQuests,
  activeUnfinishedQuests,
  unfinishedGameAppIds,
} = useUserQuests();

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    if (!token.value) {
      autoDetectLocalToken();
    } else if (quests.value.length === 0) {
      fetchQuests();
    }
  }
});

function close() {
  emit('close');
}

function handleLogout() {
  setToken('');
  quests.value = [];
  openLoginWindow();
}

function emitSync() {
  emit('sync-games', unfinishedGameAppIds.value);
}

function isCompleted(quest: DiscordQuest): boolean {
  return Boolean(quest.user_status?.completed_at || quest.user_status?.claimed_at);
}
</script>
