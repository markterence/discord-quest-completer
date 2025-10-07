<template>
  <Transition
    enter-active-class="transition-opacity duration-300 ease-in-out"
    leave-active-class="transition-opacity duration-500 ease-in-out"
    enter-from-class="opacity-0 translate-y-2"
    enter-to-class="opacity-100 translate-y-0"
  >
    <div v-if="showNotification" :class="containerClass">
      <slot />
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, watch, onUnmounted } from 'vue';

interface Props {
  isReady: boolean;
  duration?: number; // Duration in milliseconds
  containerClass?: string;
}

const props = withDefaults(defineProps<Props>(), {
  duration: 2500,
  containerClass: ''
});

const showNotification = ref(false);
let timeoutId: ReturnType<typeof setTimeout> | null = null;

watch(() => props.isReady, (newValue) => {
  if (newValue) {
    showNotification.value = true;
    
    // Clear any existing timeout
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
    
    // Set timeout to hide notification
    timeoutId = setTimeout(() => {
      showNotification.value = false;
      timeoutId = null;
    }, props.duration);
  }
});

onUnmounted(() => {
  if (timeoutId) {
    clearTimeout(timeoutId);
  }
});
</script>