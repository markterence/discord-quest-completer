import { createGlobalState } from '@vueuse/core'
import { computed, shallowRef } from 'vue'
import { useRouter } from 'vue-router'

export const Pages = {
    HOME: 'home',
    PLAYGROUND: 'playground',
    SETTINGS: 'settings',
} as const
export type Pages = typeof Pages[keyof typeof Pages]

export const useGlobalState = createGlobalState(
  () => {
    const router = useRouter();
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
      if (newPage === Pages.HOME) {
        router.push({ path: '/' })
      } else if (newPage === Pages.PLAYGROUND) {
        router.push('/Playground')
      } else if (newPage === Pages.SETTINGS) {
        router.push('/Settings')
      }
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