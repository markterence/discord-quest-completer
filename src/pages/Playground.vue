<template>
    <div>
        <button 
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            @click="discordTest">Discord Test

            connected: {{ isConnected }}
        </button>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';    
import { invoke } from '@tauri-apps/api/core';

const ActivityKind = {
    Playing: 0,
    Listening: 2,
    Watching: 3,
    Competing: 5
} as const;

const isConnected = ref(false);

function discordTest() { 

    const appIdCode = '1361728268088381706';
 
    if(isConnected.value) {
        console.log('Disconnecting from Discord');
        invoke('set_activity', {
            activity_json:  JSON.stringify({
                app_id: appIdCode,
                details: 'Disconnecting from Discord'
            }),
            state: 'disconnect'
        })
        isConnected.value = false;
        return;
    }
    

    invoke('set_activity', {
        activity_json:  JSON.stringify({
            app_id: appIdCode,
            details: 'Watching Japanese media',
            state: "state", 
            activity_kind: ActivityKind.Listening,
            timestamp: createAgoTimestamp('1h 30m')
        }),
        state: 'connect'
    })
    
    isConnected.value = true;
}

// function to create timestamp behind current time.
// example: input is `4h 30m` means timestamp should start from 4 hours and 30 minutes behind current time.
function createAgoTimestamp(input: string) {
    const time = input.split(' ');
    let hours = 0;
    let minutes = 0;

    for(let i = 0; i < time.length; i++) {
        if(time[i].includes('h')) {
            hours = parseInt(time[i]);
        } else if(time[i].includes('m')) {
            minutes = parseInt(time[i]);
        }
    }

    const date = new Date();
    date.setHours(date.getHours() - hours);
    date.setMinutes(date.getMinutes() - minutes);

    return Math.floor(date.getTime() / 1000);
}


onMounted(() => {
   
})
</script>

<style scoped>

</style>