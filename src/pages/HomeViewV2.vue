<script setup lang="ts">
import { provide, useTemplateRef, ref } from 'vue';
import { type GameActionsProvider, type GameExecutable, type Game } from '@/types/types';
import { GameActionsKey } from '@/constants/constants';
import { useFetchGameList } from '@/composables/fetch-gamelist';
import { useGameList } from '@/composables/useGameList';
import { useGameActions } from '@/composables/useGameActions';
import Dialog from '@/components/Dialog.vue';
import TestRPCDialog, { TestRPCDialogKey } from '@/components/TestRPCDialog.vue';
import GameSearch from '@/components/GameSearch.vue';
import GameList from '@/components/GameList.vue';
import GameActionsPanel from '@/components/GameActionsPanel.vue';
import GameListFetchStatus from '@/components/GameListFetchStatus.vue';
import { isEmpty } from 'lodash-es';

// Fetch game list
const {
    gameDB,
    isLoadingBundled,
    isLoadingDiscord,
    isLoadingGH,
    fetchGameList,
    isReadyGH,
    isReadyBundled,
    isReadyDiscord,
    allFetchDone,
} = useFetchGameList()

// Game list management
const { gameList, selectedGameId, selectedGame, addGame, removeGame, selectGame, initializeWithGameDB } = useGameList()

// Initialize game list with gameDB
initializeWithGameDB(gameDB, allFetchDone)

// Game actions
const {
    currentlyPlaying,
    isConnectedToRPC,
    isConnecting,
    playGame,
    stopPlaying,
    installAndPlay,
    handleTestRPC,
    continueRPCRisk,
    canPlayGame,
    isGameInstalled,
    isExecutableRunning,
    isGameExecutableInstalled,
} = useGameActions()

// Dialog management
const dialogRef = useTemplateRef<InstanceType<typeof TestRPCDialog>>('dialogRef')
const dialogMessage = ref('')
const isDialogOpen = ref(false)

function showDialog(message: TestRPCDialogKey) {
    isDialogOpen.value = true
    dialogMessage.value = message
    if(!isEmpty(message)) {
        dialogRef.value?.showModal(message)
    }
}

function hideDialog() {
    dialogRef.value?.close()
    dialogMessage.value = ''
    isDialogOpen.value = false
}

// Handle test RPC with dialog
async function handleTestRPCClick() {
    try {
        await handleTestRPC(selectedGame.value)
    } catch (error) {
        const errorMessage = (error as Error).message
        if (errorMessage === 'no_game_selected' || errorMessage === 'rpc_message_1') {
            showDialog(errorMessage as TestRPCDialogKey)
        }
    }
}

// Continue RPC risk
async function handleContinueRPCRisk() {
    await continueRPCRisk(selectedGame.value)
    hideDialog()
}

// Provide game actions for child components
provide<GameActionsProvider>(GameActionsKey, {
    canPlayGame,
    isGameInstalled,
    isExecutableRunning,
    isGameExecutableInstalled,
})
</script>

<template>
    <div class="container mx-auto px-4 py-8">
        <!-- Error Dialog -->
        <Dialog /> 
        
        <!-- RPC dialog -->
        <TestRPCDialog ref="dialogRef" @continueRPCRisk="handleContinueRPCRisk" />

        <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-6 text-center">
            Discord Quest Completer
        </h1>

        <!-- Fetch status notifications -->
        <GameListFetchStatus
            :is-loading-g-h="isLoadingGH"
            :is-ready-g-h="isReadyGH"
            :is-loading-discord="isLoadingDiscord"
            :is-ready-discord="isReadyDiscord"
            :is-loading-bundled="isLoadingBundled"
            :is-ready-bundled="isReadyBundled"
            :all-fetch-done="allFetchDone"
        />

        <!-- Search Bar -->
        <GameSearch
            :game-db="gameDB"
            @add-game="addGame"
            @refetch="fetchGameList"
        />

        <!-- Two-Column Layout -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 relative">
            <!-- Left Column: Selected Games -->
            <GameList
                :games="gameList"
                :selected-game-id="selectedGameId"
                @select="selectGame"
                @remove="removeGame"
            />

            <!-- Right Column: Game Actions -->
            <GameActionsPanel
                :selected-game="selectedGame"
                :is-connected-to-r-p-c="isConnectedToRPC"
                :is-connecting="isConnecting"
                :currently-playing="currentlyPlaying"
                :games="gameList"
                @test-r-p-c="handleTestRPCClick"
                @play="playGame"
                @stop="stopPlaying"
                @install-and-play="installAndPlay"
            />
        </div>
    </div>
</template>