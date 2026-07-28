import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { DiscordQuest } from '@/types/types';
import { useGlobalState } from './app-state';

export interface DiscordUserProfile {
  id: string;
  username: string;
  global_name?: string;
  avatar?: string;
}

export function getQuestAppId(q: any): string | null {
  if (!q) return null;
  const appId =
    q.config?.application_id ||
    q.config?.application?.id ||
    q.application_id ||
    q.application?.id ||
    null;
  return appId ? String(appId) : null;
}

export function getQuestGameTitle(q: any): string {
  if (!q) return 'Discord Quest Game';
  return (
    q.config?.messages?.game_title ||
    q.config?.application?.name ||
    q.config?.application_name ||
    q.config?.title ||
    'Discord Quest Game'
  );
}

export function isPlayableWindowsGameQuest(q: any): boolean {
  if (!q || !q.config) return false;
  
  // 1. Must have an application ID (game app)
  const appId = getQuestAppId(q);
  if (!appId) return false;

  // 2. Filter out completed or claimed quests
  const status = q.user_status;
  if (status?.completed_at || status?.claimed_at) return false;

  // 3. Filter out expired quests
  if (q.config?.expires_at) {
    const expiryDate = new Date(q.config.expires_at);
    if (!isNaN(expiryDate.getTime()) && expiryDate <= new Date()) {
      return false;
    }
  }

  // 4. Filter out video / stream watch quests (only playable games on Windows)
  const questName = (q.config?.messages?.quest_name || q.config?.title || '').toLowerCase();
  const taskType = String(q.config?.task_config?.task_type || q.config?.task_type || '').toUpperCase();
  const tasksObj = q.config?.task_config?.tasks || {};

  if (taskType.includes('VIDEO') || taskType.includes('WATCH') || taskType.includes('STREAM_WATCH')) {
    return false;
  }
  
  if (tasksObj['WATCH_VIDEO'] || tasksObj['WATCH_STREAM']) {
    return false;
  }

  if (questName.includes('watch') || questName.includes('video') || questName.includes('episode') || questName.includes('stream')) {
    if (!tasksObj['PLAY_ON_DESKTOP'] && !tasksObj['PLAY_ON_DESKTOP_V2']) {
      return false;
    }
  }

  return true;
}

const TOKEN_STORAGE_KEY = 'discord_user_token_v1';
const isAccountModalOpen = ref(false);
const userProfile = ref<DiscordUserProfile | null>(null);

export function useUserQuests() {
  const { addLog } = useGlobalState();
  const token = ref<string>('');
  const quests = ref<DiscordQuest[]>([]);
  const isLoading = ref<boolean>(false);
  const errorMessage = ref<string | null>(null);

  const avatarUrl = computed(() => {
    return `https://cdn.discordapp.com/embed/avatars/0.png`;
  });

  function setToken(newToken: string) {
    token.value = newToken.trim();
    if (!token.value) {
      userProfile.value = null;
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
        fetchUserProfile();
      }
    });
  });

  async function fetchUserProfile() {
    if (!token.value) return;
    try {
      const raw = await invoke<string>('fetch_user_profile', { token: token.value });
      userProfile.value = JSON.parse(raw);
      addLog('info', `Logged in as Discord user: ${userProfile.value?.global_name || userProfile.value?.username}`);
    } catch (err) {
      console.warn('Failed to fetch user profile:', err);
    }
  }

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
      fetchUserProfile();
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





  // Filter quests that are playable Windows games, NOT completed yet, and NOT expired
  const activeUnfinishedQuests = computed(() => {
    return quests.value.filter((q: any) => isPlayableWindowsGameQuest(q));
  });

  // Extract application_ids of games to auto-add
  const unfinishedGameAppIds = computed(() => {
    return activeUnfinishedQuests.value
      .map((q) => getQuestAppId(q))
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
        await fetchUserProfile();
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
    userProfile,
    avatarUrl,
    isLoading,
    errorMessage,
    setToken,
    openLoginWindow,
    openDefaultBrowser,
    autoDetectLocalToken,
    fetchQuests,
    fetchUserProfile,
    activeUnfinishedQuests,
    unfinishedGameAppIds,
  };
}
