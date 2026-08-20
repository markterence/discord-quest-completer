I<script setup lang="ts">
import { DialogInjectionKey } from '@/constants/constants';
import { useConfirmDialog } from '@vueuse/core'
import { inject } from 'vue';
import { DialogContext } from '../Dialog.vue';

const props = defineProps<{
  errorType: 'github_gamelist_fetch_error' | 'discord_gamelist_fetch_error' | 'local_gamelist_fetch_error'
}>()

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()

const { handleClose } = inject(DialogInjectionKey) as DialogContext;

const handleConfirm = async () => {
  await handleClose();
  emit('confirm');
}

const handleCancel = async () => {
  await handleClose();
  emit('cancel');
}
</script>

<template>
  <div class="error-dialog">
    <div class="error-dialog-header">
      <h2>Game List Error</h2>
      <button class="close-btn" @click="handleCancel">&times;</button>
    </div>
    <div class="error-dialog-body"> 
      <p v-if="errorType === 'github_gamelist_fetch_error'">
        There was an error fetching the game list from GitHub. This could be due to network issues (e.g. Custom DNS resolution failure, timeout, ISP DNS/Network). 
        Game list will try to load from Discord directly or fallback to bundled game list.
      </p>
      <p v-else-if="errorType === 'discord_gamelist_fetch_error'">
        There was an error fetching the game list from Discord. This could be due to network issues (e.g. Custom DNS resolution failure, timeout, ISP DNS/Network). 
        Other issues may include changes to the Discord API (Nothing can be done about this for now).
      </p>
      <p v-else-if="errorType === 'local_gamelist_fetch_error'">
        There was an error loading the bundled game list.
        If issue persists, please file an issue on 
        <a href="https://github.com/markterence/discord-quest-completer/issues"></a>
      </p>
      <div class="error-dialog-footer" style="margin-top: 20px;">  
        <button class="btn-primary" @click="handleConfirm">OK</button> 
      </div>
    </div>
  </div>
</template>
<style scoped>
.error-dialog {
  background: var(--bg-gray-800);
  border-radius: 12px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  width: 500px;
  max-width: 90vw;
}

.error-dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #e0e0e0;
}

.error-dialog-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  color: #666;
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: background 0.2s;
}

.close-btn:hover {
  background: #f0f0f0;
}

.error-dialog-body {
  padding: 24px;
}

.error-icon {
  font-size: 48px;
  text-align: center;
  margin-bottom: 16px;
}

.error-message {
  text-align: center;
  color: #333;
  font-size: 16px;
  line-height: 1.5;
  margin: 0 0 16px 0;
}

.error-details {
  background: #f8f9fa;
  border-radius: 8px;
  padding: 16px;
  margin: 16px 0;
}

.error-details h3 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: #555;
}

.error-details ul {
  margin: 0;
  padding-left: 20px;
}

.error-details li {
  margin: 8px 0;
  color: #666;
  font-size: 14px;
}

.error-technical {
  margin-top: 16px;
  font-size: 12px;
}

.error-technical summary {
  cursor: pointer;
  color: #666;
  user-select: none;
}

.error-technical pre {
  background: #f5f5f5;
  padding: 12px;
  border-radius: 4px;
  overflow-x: auto;
  margin: 8px 0;
  font-size: 11px;
  color: #333;
}

.error-dialog-footer {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  padding: 16px 24px;
  border-top: 1px solid #e0e0e0;
}

button {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
}

.btn-primary {
  background: #0066cc;
  color: white;
}

.btn-primary:hover {
  background: #0052a3;
}

.btn-secondary {
  background: #e0e0e0;
  color: #333;
}

.btn-secondary:hover {
  background: #d0d0d0;
}
</style>