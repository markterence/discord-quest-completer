import { createSharedComposable } from "@vueuse/core";
import { ref, computed, shallowRef, markRaw } from "vue"; 
import type { Component } from "vue";

export interface ErrorDialogMeta {
  id: string;
  timestamp: number;
  /**
   * Wether to allow dismissing the dialog by clicking outside of it or pressing escape. Defaults to `true`.
   */
  backdropDismiss?: boolean;
}

export interface ErrorDialogItem<T = any> {
  meta: ErrorDialogMeta;
  component: Component;
  props?: T;
  onClose?: () => void | Promise<void>;
}

export function useErrorDialogQueue() {
  const queue = ref<ErrorDialogItem[]>([]);
  const isProcessing = ref(false);

  const currentDialog = computed(() => {
    if (queue.value.length === 0) return null;
    return [...queue.value].sort((a, b) => {
      return a.meta.timestamp - b.meta.timestamp;
    })[0];
  });

  const addToQueue = <T = any>(
    item: Omit<ErrorDialogItem<T>, "meta"> & {
      meta?: Partial<ErrorDialogMeta>;
    },
  ) => {
    const { component, ...rest } = item;
    const dialog: ErrorDialogItem<T> = {
      component: markRaw(component),
      ...rest,
      meta: {
        id: item.meta?.id || `${Date.now()}-${Math.random()}`,
        timestamp: item.meta?.timestamp || Date.now(),
        backdropDismiss: rest?.meta?.backdropDismiss ?? true, 
      },
    };

    queue.value.push(dialog);

    if (!isProcessing.value) {
      isProcessing.value = true;
    }

    return dialog.meta.id;
  };

  const removeFromQueue = (id?: string) => {
    if (id) {
      const index = queue.value.findIndex((item) => item.meta.id === id);
      if (index !== -1) {
        queue.value.splice(index, 1);
      }
    } else {
      queue.value.shift();
    }

    if (queue.value.length === 0) {
      isProcessing.value = false;
    }
  };

  const clear = () => {
    queue.value = [];
    isProcessing.value = false;
  };

  return {
    queue: computed(() => queue.value),
    currentDialog,
    isProcessing: computed(() => isProcessing.value),
    queueLength: computed(() => queue.value.length),
    addToQueue,
    removeFromQueue,
    clear,
  };
}
export const useSharedErrorDialogQueue = createSharedComposable(useErrorDialogQueue);