import { Game } from "@/types/types"

export interface FuseGameListData extends Game {
  searchName: string
  searchAliases: string | string[]
  searchExecutables: string | string[]
}
// NOTE: Not needed? search still works
// Symbols to ignore in search
const COPYRIGHT_SYMBOL = '\u00A9'
const TRADEMARK_SYMBOL = '\u2122'
const REGISTERED_SYMBOL = '\u00AE'
const ignoredSymbols = [COPYRIGHT_SYMBOL, TRADEMARK_SYMBOL, REGISTERED_SYMBOL]
export const ignoredSymbolsRegex = new RegExp(`[${ignoredSymbols.join('')}]`, 'g')

/**
 * Remove special symbols from a game object's searchable properties when searching
 * This function is used for Fuse.js on `useGameSearch.ts` composable
 */
export function sanitizeGameForSearch(game: Game): FuseGameListData {
  // Clean name
  const searchName = game.name?.replace(ignoredSymbolsRegex, '') || ''

  // Clean aliases
  let searchAliases: string | string[] = ''
  if (Array.isArray(game?.aliases)) {
    searchAliases = game?.aliases
      .filter((a): a is string => typeof a === 'string')
      .map(a => a.replace(ignoredSymbolsRegex, ''))
  } else if (typeof game?.aliases === 'string') {
    searchAliases = (game?.aliases as string).replace(ignoredSymbolsRegex, '')
  }

  // Clean executables
  let searchExecutables: string | string[] = ''
  if (Array.isArray(game.executables)) {
    searchExecutables = game.executables
      .map(e => e?.name)
      .filter((name): name is string => typeof name === 'string')
      .map(name => name.replace(ignoredSymbolsRegex, ''))
  } else if (game.executables && typeof game.executables === 'object' && 'name' in game.executables) {
    const execName = (game.executables as any).name
    if (typeof execName === 'string') {
      searchExecutables = execName.replace(ignoredSymbolsRegex, '')
    }
  }

  return {
    ...game,
    searchName,
    searchAliases,
    searchExecutables,
  }
}