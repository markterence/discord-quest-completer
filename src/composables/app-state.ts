import { createGlobalState } from '@vueuse/core'
import { computed, shallowRef } from 'vue'

export const Pages = {
    HOME: 'home',
    PLAYGROUND: 'playground',
} as const
export type Pages = typeof Pages[keyof typeof Pages]

export const useGlobalState = createGlobalState(
  () => {
    // state
    const page = shallowRef<Pages>(Pages.HOME)

    const count = shallowRef(0)

    // getters
    const doubleCount = computed(() => count.value * 2)

    // actions
    function increment() {
      count.value++
    }

    function setPage(newPage: Pages) {
      page.value = newPage
    }

    return { 
        page,
        count, 
        doubleCount,
        setPage, 
        increment
    }
  }
)