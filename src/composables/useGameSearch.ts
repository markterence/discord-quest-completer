import { refDebounced } from '@vueuse/core'
import { useFuse, type UseFuseOptions } from '@vueuse/integrations/useFuse'
import { computed, shallowRef, type Ref, ref} from 'vue'
import type { Game } from '@/types/types'
import Fuse from 'fuse.js'

export interface UseGameSearchReturn {
  /**
   * The search keyword
   */
  searchQuery: Ref<string>
  /**
   * Debounced value of the search keyword
   */
  debouncedSearchQuery: Ref<string>
  
  /**
   * Fuzzy search result
   */
  searchResults: Ref<Array<{ item: Game; refIndex: number; score?: number }>>
}

const SEARCH_RESULT_LIMIT = 12 as const;

const FUSE_DEFAULT_OPTIONS = {
  // Prioritize name, aliases, then exe names
  keys: [
    { name: 'name', weight: 0.7 },
    { name: 'aliases', weight: 0.2 },
    { name: 'executables.name', weight: 0.1 },
    // { name: 'searchName', weight: 0.7 },
    // { name: 'searchAliases', weight: 0.2 },
    // { name: 'searchExecutables', weight: 0.1 },
  ],
  /** getFn may not be needed? names with symbols are also included in result */
  // getFn: (obj: any, path: string[] | string) => {
  //   const value = Fuse.config.getFn(obj, path)
  //   return typeof value === 'string'
  //     ? value.replace(ignoredSymbolsRegex, '')
  //     : value
  // },
  isCaseSensitive: false,
  threshold: 0.4,
  // A score of 0 indicates a perfect match, while a score of 1 indicates a complete mismatch
  includeScore: true,
  includeMatches: false,
};

/**
 * Composable for game search functionality using fuzzy search
 * NOTE: Slow. typing and clearing input has some lag.
 * @param gameDB Ref to the game database array
 */
export function useGameSearch_V1(gameDB: Ref<Game[]>): UseGameSearchReturn {
  const searchQuery = shallowRef('')
  const debouncedSearchQuery = refDebounced(searchQuery, 300)
   
  const fuseIndex = computed(() => {
    if (!gameDB.value || gameDB.value.length === 0) return null
    // https://www.fusejs.io/performance.html#indexing
    return Fuse.createIndex(FUSE_DEFAULT_OPTIONS.keys, gameDB.value)
  })

  const fuseOptions = computed<UseFuseOptions<Game>>(() => ({
    fuseOptions: {
      ...FUSE_DEFAULT_OPTIONS,
      index: fuseIndex
    },
    resultLimit: SEARCH_RESULT_LIMIT,
    matchAllWhenSearchEmpty: false,
  }))
  
  const { results: searchResults } = useFuse(debouncedSearchQuery, gameDB.value, fuseOptions)
   
  return {
    searchQuery,
    debouncedSearchQuery,
    searchResults
  }
}

/**
 * Composable for game search functionality using fuzzy search.
 * Faster implementation.
 * @param gameDB Ref to the game database array
 */
export function useGameSearch_V2(gameDB: Ref<Game[]>): UseGameSearchReturn {
  const searchQuery = shallowRef('')
  const debouncedSearchQuery = refDebounced(searchQuery, 300)

  const fuseInstance = computed(() => {
    const rawList = gameDB.value || []
    if (rawList.length === 0) return null

    // const sanitizedList = rawList.map(sanitizeGameForSearch)
    const sanitizedList = rawList;
    const fuseIndex = Fuse.createIndex(FUSE_DEFAULT_OPTIONS.keys, sanitizedList)

    return new Fuse(
      sanitizedList,
      FUSE_DEFAULT_OPTIONS,
      fuseIndex
    )
  })

  const searchResults = computed(() => {
    const query = debouncedSearchQuery.value?.trim()
    const fuse = fuseInstance.value

    if (!fuse) return []

    const result = fuse.search(query, { limit: SEARCH_RESULT_LIMIT })
    return result
  })

  return {
    searchQuery,
    debouncedSearchQuery,
    searchResults
  }
}
