<template>
    <dialog v-if="isRevealed"  
        class="inset-0 bg-gray-800 bg-opacity-50
        backdrop:backdrop-blur-xs
        backdrop:bg-black/70
        border border-gray-300 dark:border-gray-600 rounded-lg
        transition-opacity duration-300 ease-in-out z-50
        " 
        style="left: 50%; top: 50%; transform: translate(-50%, -50%)"
        ref="dialogRef"
        >
        <div class="flex flex-col items-center justify-center p-6">
            <div class="mb-4 text-gray-500 dark:text-gray-400">
                <div v-if="dialogKey === 'rpc_message_1'">
                    <p class="font-bold">
                        This is only a developer feature.
                        This may flag your account as suspicious for self-botting.
                    </p>
                    <p class="my-3">
                        This uses Discord Rich Presence to show
                        that you are playing an official game as your Status, it 
                        uses the Discord Application ID of the official Game 
                        to communicate with the RPC,
                        but it will not work with Quests that require you to have the game running.
                    </p>
        
                </div>

                <div v-if="dialogKey === 'no_game_selected'">
                    <p>
                        No game selected. Please select a game from the list on the left.
                    </p>
                </div>
            </div>
            <div class="gap-2 flex">
                <button class="
                text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 
                border border-gray-300 dark:border-gray-600 rounded-lg px-4 py-1" @click="hideDialog()">
                    <span v-if="dialogKey == 'rpc_message_1'">
                        Cancel
                    </span>
                    <span v-else>OK</span>
                </button>

                <button v-if="dialogKey === 'rpc_message_1'" class="text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 
                border border-gray-300 dark:border-gray-600 rounded-lg px-4 py-1"
                    @click="onContinueRPCRisk">
                    Accept risk and continue
                </button>
            </div>
        </div>
    </dialog>
</template>

<script setup lang="ts">
import { useConfirmDialog } from '@vueuse/core';
import { Game } from '@/types/types';
import { nextTick, ref, useTemplateRef } from 'vue';

export type TestRPCDialogKey = 'rpc_message_1' | 'no_game_selected' | 'none'
const emit = defineEmits<{
    continueRPCRisk: []
}>()

const dialogKey = ref<TestRPCDialogKey>('none');
const dialogRef = useTemplateRef<HTMLDialogElement>('dialogRef');

const { isRevealed, reveal, cancel, confirm, onCancel, onConfirm, onReveal } = useConfirmDialog()

function showDialog(key: TestRPCDialogKey) {
    console.log('Showing dialog with key:', key);
    dialogKey.value = key;
    // console.log('dialogRef', dialogRef.value);
    // dialogRef.value?.showModal();
    reveal();
    
    nextTick(() => {
         dialogRef.value?.showModal();
    })
    
}
function hideDialog() {
    cancel();
}

defineExpose({
    showDialog,
    hideDialog,
    /**
     * Alias only. Use `showDialog` instead.
     */
    showModal: showDialog,
    /**
     * Alias only. Use `hideDialog` instead.
     */
    close: hideDialog,
})

</script>

<style scoped>
/* @reference "../theme/style.css";

.dialogStyle::backdrop {
    @apply bg-black/70 ;
    background: var(--bg-black-700/70);
} */
</style>