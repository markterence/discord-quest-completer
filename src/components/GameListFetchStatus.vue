<script setup lang="ts">
import TimedNotification from '@/components/TimedNotification.vue'
import { computed } from 'vue'

interface Props {
  isLoadingGH: boolean
  isReadyGH: boolean
  isLoadingDiscord: boolean
  isReadyDiscord: boolean
  isLoadingBundled: boolean
  isReadyBundled: boolean
  allFetchDone: boolean
}

const props = defineProps<Props>()

const shouldShowNotificationContainer = computed(() => {
  return (
    props.isLoadingGH ||
    props.isLoadingDiscord ||
    props.isLoadingBundled ||
    props.isReadyGH ||
    props.isReadyDiscord ||
    props.isReadyBundled
  )
})
</script>

<template>
  <Transition
    enter-active-class="transition-opacity duration-300 delay-100 ease-in-out"
    leave-active-class="transition-opacity duration-600 delay-100 ease-in-out"
    enter-from-class="opacity-0 translate-y-2 ease-in-out"
    enter-to-class="opacity-100 translate-y-0 ease-in-out"
  >
    <div
      v-if="shouldShowNotificationContainer && !allFetchDone"
      class="absolute top-20 left-4 z-20"
    >
      <!-- Fetching from GitHub mirror loading indicator -->
      <Transition
        enter-active-class="transition-opacity duration-300 delay-100 ease-in-out"
        leave-active-class="transition-opacity duration-600 delay-100 ease-in-out"
        enter-from-class="opacity-0 translate-y-2 ease-in-out"
        enter-to-class="opacity-100 translate-y-0 ease-in-out"
      >
        <div v-if="isLoadingGH" class="text-sm text-gray-500 dark:text-gray-400">
          Fetching game list from GitHub mirror...
          <div
            class="border-full h-2 w-2 bg-green-500 rounded-full inline-block ml-2 animate-pulse"
          ></div>
        </div>
      </Transition>

      <TimedNotification
        :is-ready="isReadyGH"
        :duration="1500"
        container-class="text-sm text-gray-500 dark:text-gray-400"
      >
        Game list from mirror fetched <span class="text-green-400">✓</span>
      </TimedNotification>

      <!-- Fetching from Discord loading indicator -->
      <Transition
        enter-active-class="transition-opacity duration-300 delay-100 ease-in-out"
        leave-active-class="transition-opacity duration-600 delay-100 ease-in-out"
        enter-from-class="opacity-0 translate-y-2 ease-in-out"
        enter-to-class="opacity-100 translate-y-0 ease-in-out"
      >
        <div v-if="isLoadingDiscord" class="text-sm text-gray-500 dark:text-gray-400">
          Fetching game list directly from Discord...
          <div
            class="border-full h-2 w-2 bg-green-500 rounded-full inline-block ml-2 animate-pulse"
          ></div>
        </div>
      </Transition>

      <TimedNotification
        :is-ready="isReadyDiscord"
        :duration="1500"
        container-class="text-sm text-gray-500 dark:text-gray-400"
      >
        Game list from Discord fetched <span class="text-green-400">✓</span>
      </TimedNotification>

      <!-- Fetching from bundled loading indicator -->
      <Transition
        enter-active-class="transition-opacity duration-300 delay-100 ease-in-out"
        leave-active-class="transition-opacity duration-600 delay-100 ease-in-out"
        enter-from-class="opacity-0 translate-y-2 ease-in-out"
        enter-to-class="opacity-100 translate-y-0 ease-in-out"
      >
        <div v-if="isLoadingBundled" class="text-sm text-gray-500 dark:text-gray-400">
          Fetching game list from bundled game list...
          <div
            class="border-full h-2 w-2 bg-green-500 rounded-full inline-block ml-2 animate-pulse"
          ></div>
        </div>
      </Transition>

      <TimedNotification
        :is-ready="isReadyBundled"
        :duration="1500"
        container-class="text-sm text-gray-500 dark:text-gray-400"
      >
        Game list from bundle pre-loaded <span class="text-green-400">✓</span>
      </TimedNotification>
    </div>
  </Transition>
</template>

