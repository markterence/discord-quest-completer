<script setup lang="ts">
import { provide, watch } from 'vue'
import { useErrorDialogQueue, useSharedErrorDialogQueue } from '../composables/useErrorDialogQueue'
import { useConfirmDialog } from '@vueuse/core' 
import { DialogInjectionKey } from '../constants/constants'

export interface DialogContext {
  handleClose: () => Promise<void>
}

const { currentDialog, removeFromQueue } = useSharedErrorDialogQueue()

const { isRevealed, reveal, cancel } = useConfirmDialog()

watch(() => currentDialog.value, (dialog) => {
  if (dialog) {
    reveal()
  } else {
    cancel()
  }
}, { immediate: true })

const handleClose = async () => {
  if (currentDialog.value?.onClose) {
    await currentDialog.value.onClose()
  }
  
  if (currentDialog.value) {
    removeFromQueue(currentDialog.value.meta.id)
  }
  
  cancel()
}

const handleOverlayClick = () => {
  if (currentDialog.value?.meta?.backdropDismiss) {
    handleClose();
  }
}

provide(DialogInjectionKey, {handleClose})
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div v-if="isRevealed && currentDialog" class="dialog-overlay" @click.self="handleOverlayClick">
        <Transition name="dialog-slide">
          <div v-if="isRevealed" class="dialog-wrapper">
            <component 
              :is="currentDialog.component" 
              v-bind="currentDialog.props"
              @close="handleClose"
            />
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(2px);
}

.dialog-wrapper {
  max-width: 90vw;
  max-height: 90vh;
  overflow: auto;
}

/* Transitions */
.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: opacity 0.3s ease;
}

.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;
}

.dialog-slide-enter-active,
.dialog-slide-leave-active {
  transition: transform 0.3s ease, opacity 0.3s ease;
}

.dialog-slide-enter-from {
  transform: translateY(-20px);
  opacity: 0;
}

.dialog-slide-leave-to {
  transform: translateY(20px);
  opacity: 0;
}
</style>