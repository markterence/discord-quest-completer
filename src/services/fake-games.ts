import { randomString } from '@/utils/random-string';
import gameListData from '../assets/gamelist.json';
import type { Game } from '@/types/types';

export const fakeGames = async () => {
  return new Promise<Game[]>((resolve) => {
    const games = gameListData.slice(0, 30);
    const sorted = games.sort((a, b) => a.name.localeCompare(b.name));

    resolve(sorted.map((game: Game) => {
        game['uid'] = randomString()
        return game
    }))
 
  });
}

