import { Game } from '@/types/types';
import { fetch, ClientOptions } from '@tauri-apps/plugin-http';
import { tryOnMounted, useAsyncState } from '@vueuse/core';
import { ExtractPropTypes, Prop, PropType, ref, watch } from 'vue';
import { message } from '@tauri-apps/plugin-dialog'; 
import { invoke } from '@tauri-apps/api/core';
import { useGlobalState } from './app-state';
import { useErrorDialogQueue, useSharedErrorDialogQueue } from './useErrorDialogQueue';
import GameListErrorDialog from '@/components/error-dialog/GameListErrorDialog.vue';

export function useFetchGameList() {
    const { addLog } = useGlobalState();
    async function fetchGameListGHMirror() {
        addLog('Fetching game list from GitHub mirror...'); 
        const response = await invoke('fetch_gamelist_gh_mirror').catch((err) => {
            console.debug('fetchGameListGHMirror', err);
            throw err;
        });
        console.debug('response', response)
        return response as Game[] | unknown[] | undefined;
    }
    async function fetchGameListFromDiscord (){
        addLog('Fetching game list directly from discord...'); 
        const response = await invoke('fetch_gamelist_from_discord');
        return response as Game[] | unknown[] | undefined;
    };

    // const fetchBundledGameList = fetch(window.location.origin+'/gamelist.json', { method: 'GET' });

    const { 
        state: gameListGHMirror,
        error: errorGH,
        isReady: isReadyGH,
        execute: executeGH,
        isLoading: isLoadingGH
    } = useAsyncState<Game[] | unknown[] | undefined>(fetchGameListGHMirror, [], {
            immediate: false,
            resetOnExecute: true,
        });
    const { 
        state: gameListFromDiscord, 
        error: errorDiscord,
        isReady: isReadyDiscord,
        execute: executeDiscord,
        isLoading: isLoadingDiscord
    } = useAsyncState(fetchGameListFromDiscord, [], {
        immediate: false,
        resetOnExecute: true,
    });
    const { 
        state: bundledGameList,
        error: errorBundled,
        isReady: isReadyBundled,
        execute: executeBundled,
        isLoading: isLoadingBundled
    } = useAsyncState(() => {
        const result = import('../assets/gamelist.json').then(res=>res.default);
        addLog('Fetching bundled game list for fallback...');
        return result;
    }, [], {
        immediate: false,
        resetOnExecute: true,
    });

    const { 
        addToQueue: addErrorToQueue
    } = useSharedErrorDialogQueue();

    const fetchError = ref<string | null>(null);

    const gameDB = ref<Game[]>([]);

    const allFetchDone = ref(false);

    function isValidGameList(data: any): boolean {
        return Array.isArray(data) && data[0] && 'aliases' in data[0] && 'name' in data[0] && 'executables' in data[0];
    }

    watch(() => isReadyGH.value, async (newVal) => {
        addLog('debug','isReadyGH: ' + newVal); 
    });

    watch(() => isReadyDiscord.value, async (newVal) => {
        addLog('debug','isReadyDiscord: ' + newVal);
    })
    
    watch(() => isReadyBundled.value, async (newVal) => {
        addLog('debug','isReadyBundled: ' + newVal); 
    });

    let timeoutId: ReturnType<typeof setTimeout> | null = null;
    async function fetchGameList() { 
        allFetchDone.value = false;
        addLog('Fetching game list...');
        // try fetching from the Github mirror first, then Discord. Use bundled as fallback.
        try {
           const res = await Promise.all([executeGH(), executeBundled()]);
           console.debug('all', res)
        } catch (e) {
            addLog('error', 'Error executing fetch for GitHub mirror or bundled game list.');
            console.error(`fetch`, e);
        }

        console.debug('errorGH', errorGH.value)
        if (errorGH.value) { 
            fetchError.value = 'Error fetching game list from GitHub mirror.';
            addLog('error','Error fetching game list from GitHub mirror');
            addErrorToQueue<ExtractPropTypes<typeof GameListErrorDialog>>({
                component: GameListErrorDialog,
                props: {
                    errorType: 'github_gamelist_fetch_error',
                },
                meta: {
                    backdropDismiss: false,
                }
            });
            // await message(
            //     `GitHub Mirror Fetch Error\n\n` +
            //     `There was an error fetching the game list from this repository's Github pages.\n\n` + 
            //     `A fallback attempt will now be made to fetch the game list directly from Discord.\n` +
            //     `If that also fails, the bundled game list will be used as a last resort.`,
            //     {
            //         title: 'Discord Quest Completer - Game List Fetch Error',
            //         kind: 'warning',
            //         buttons: {
            //             ok: 'OK'
            //         }
            //     }
            // )
            await executeDiscord();
            if (errorDiscord.value) {
                fetchError.value = 'Error fetching game list from Discord.';
                addLog('error','Error fetching game list from Discord:');
                addErrorToQueue<ExtractPropTypes<typeof GameListErrorDialog>>({
                    component: GameListErrorDialog,
                    props: {
                        errorType: 'discord_gamelist_fetch_error',
                    },
                    meta: {
                        backdropDismiss: false,
                    }
                });
                if (errorBundled.value) {
                    fetchError.value = 'Error fetching bundled game list.';
                    addLog('error','Error fetching bundled game list:');
                    addErrorToQueue<ExtractPropTypes<typeof GameListErrorDialog>>({
                        component: GameListErrorDialog,
                        props: {
                            errorType: 'local_gamelist_fetch_error',
                        },
                        meta: {
                            backdropDismiss: false,
                        }
                    });
                }
            }
        }
        // silently log error for bundled, as it's the last resort.
        if (errorBundled.value) {
            addLog('error','Error fetching bundled game list');
        }

        // if (fetchError.value) {
        //     await message('There was an error fetching the latest game list.' + fetchError.value, {
        //         title: 'Game List Fetch Error',
        //         kind: 'error',
        //         buttons: {
        //             ok: 'OK'
        //         }
        //     });
        // }

        if (gameListGHMirror.value && gameListGHMirror.value?.length > 0 && isValidGameList(gameListGHMirror.value)) {
            gameDB.value = gameListGHMirror.value as Game[] || [];
            addLog('Using game list from GitHub mirror. ' + gameListGHMirror.value.length + ' entries.');
        } else if (gameListFromDiscord.value && gameListFromDiscord.value?.length > 0 && isValidGameList(gameListFromDiscord.value)) {
            gameDB.value = gameListFromDiscord.value as Game[] || [];
            addLog('Using game list from Discord. ' + gameListFromDiscord.value.length + ' entries.');
        } else {
            // bundled is always present.
            addLog('Using bundled game list as fallback.' + bundledGameList.value.length + ' entries.');
            gameDB.value = bundledGameList.value;
        }

        // Set a timeout to delay setting allFetchDone to true, to allow UI to update.
        timeoutId = setTimeout(() => {
            allFetchDone.value = true;

        }, 1800);

    }

    watch(allFetchDone, (newVal) => {
        if (newVal && timeoutId) {
            clearTimeout(timeoutId);
        }
    });

    tryOnMounted(async () => {
        await fetchGameList();
    });

    return {
        gameListGHMirror,
        gameListFromDiscord,
        bundledGameList,
        fetchError,
        isReadyGH,
        isReadyDiscord,
        isReadyBundled,
        gameDB,
        fetchGameList,
        isLoadingGH,
        isLoadingDiscord,
        isLoadingBundled,
        allFetchDone
    }
}