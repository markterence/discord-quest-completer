import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { DiscordQuest } from '@/types/types';
import { useGlobalState } from './app-state';

const TOKEN_STORAGE_KEY = 'discord_user_token_v1';
const isAccountModalOpen = ref(false);

export function useUserQuests() {
  const { addLog } = useGlobalState();
  const token = ref<string>(localStorage.getItem(TOKEN_STORAGE_KEY) || '');
  const quests = ref<DiscordQuest[]>([]);
  const isLoading = ref<boolean>(false);
  const errorMessage = ref<string | null>(null);

  function setToken(newToken: string) {
    token.value = newToken.trim();
    if (token.value) {
      localStorage.setItem(TOKEN_STORAGE_KEY, token.value);
    } else {
      localStorage.removeItem(TOKEN_STORAGE_KEY);
    }
  }

  async function openLoginWindow() {
    isLoading.value = true;
    errorMessage.value = null;
    addLog('info', 'Opening Discord Browser Login Window...');
    try {
      await invoke('open_discord_login_window');
    } catch (err: any) {
      const msg = typeof err === 'string' ? err : err?.message || 'Failed to open login window.';
      errorMessage.value = msg;
      addLog('error', `Error opening login window: ${msg}`);
      isLoading.value = false;
    }
  }

  onMounted(() => {
    listen<{ token: string }>('discord_token_captured', (event) => {
      if (event.payload?.token) {
        addLog('info', 'Successfully captured Discord session from browser login!');
        setToken(event.payload.token);
        fetchQuests();
      }
    });
  });

  async function fetchQuests(): Promise<DiscordQuest[]> {
    if (!token.value) {
      errorMessage.value = 'Please log in with Discord first.';
      return [];
    }

    isLoading.value = true;
    errorMessage.value = null;
    addLog('info', 'Fetching active quests from user Discord account...');

    try {
      const rawResult = await invoke<string>('fetch_user_quests', { token: token.value });
      const parsed = JSON.parse(rawResult);
      
      const questList: DiscordQuest[] = Array.isArray(parsed) 
        ? parsed 
        : (parsed.quests || []);

      quests.value = questList;
      addLog('info', `Fetched ${questList.length} quests from user account.`);
      return questList;
    } catch (err: any) {
      const msg = typeof err === 'string' ? err : err?.message || 'Failed to fetch quests.';
      errorMessage.value = msg;
      addLog('error', `Error fetching user quests: ${msg}`);
      return [];
    } finally {
      isLoading.value = false;
    }
  }

  // Filter quests that are NOT completed yet
  const activeUnfinishedQuests = computed(() => {
    return quests.value.filter((q) => {
      if (!q.config || !q.config.application_id) return false;
      const status = q.user_status;
      // If completed_at is present or claimed_at, skip
      if (status?.completed_at || status?.claimed_at) return false;
      return true;
    });
  });

  // Extract application_ids of games to auto-add
  const unfinishedGameAppIds = computed(() => {
    return activeUnfinishedQuests.value
      .map((q) => q.config.application_id)
      .filter((id): id is string => Boolean(id));
  });

  async function openDefaultBrowser(url = 'https://discord.com/login') {
    addLog('info', `Opening default system browser at ${url}...`);
    try {
      await invoke('open_default_browser', { url });
    } catch (err: any) {
      const msg = typeof err === 'string' ? err : err?.message || 'Failed to open browser.';
      errorMessage.value = msg;
      addLog('error', `Error opening system browser: ${msg}`);
    }
  }

  async function autoDetectLocalToken() {
    isLoading.value = true;
    errorMessage.value = null;
    addLog('info', 'Auto-detecting active Discord account token from local system...');
    try {
      const detectedToken = await invoke<string>('auto_detect_discord_token');
      if (detectedToken) {
        setToken(detectedToken);
        addLog('info', 'Successfully auto-detected active Discord account session!');
        await fetchQuests();
      }
    } catch (err: any) {
      const msg = typeof err === 'string' ? err : err?.message || 'Could not auto-detect token.';
      addLog('warning', `Local token auto-detect status: ${msg}`);
    } finally {
      isLoading.value = false;
    }
  }

  return {
    isAccountModalOpen,
    token,
    quests,
    isLoading,
    errorMessage,
    setToken,
    openLoginWindow,
    openDefaultBrowser,
    autoDetectLocalToken,
    fetchQuests,
    activeUnfinishedQuests,
    unfinishedGameAppIds,
  };
}
