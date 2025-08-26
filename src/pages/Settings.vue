<template>
    <div>
        <!-- development banner / warning fullwidth that notes this page is not yet working -->
        <div
            class="m-0 top-0 left-0 w-full bg-yellow-100 border-t border-b border-yellow-500 text-yellow-700 py-2 px-4 text-sm">
            <strong class="font-bold">Warning:</strong> 
            Settings page is still under development and will not work as intended.
        </div>
        <!-- end banner -->
        <div class="container mx-auto px-4 py-10 relative">

            <h1 class=" max-w-2xl mx-auto text-3xl font-bold mb-8 text-gray-900 dark:text-white">Settings</h1>
            <div class="grid gap-1 max-w-2xl mx-auto">
                <section
                    class="bg-white dark:bg-gray-800 shadow rounded-xl p-8 border border-gray-100 dark:border-gray-700">
                    <h2 class="text-xl font-semibold mb-4 text-gray-900 dark:text-white">Game List Source</h2>
                    <p class="text-gray-600 dark:text-gray-400 mb-6 text-sm">
                        Configure how the game list is fetched:
                    </p>
                    <ul class="space-y-6 mb-2">
                        <li>
                            <div class="flex items-center gap-3">
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="radio" name="gameListSource" value="discord" v-model="selectedSource"
                                        class="accent-blue-500" />
                                    <span class="font-semibold text-gray-900 dark:text-white">Fetch Directly from
                                        Discord</span>
                                </label>
                            </div>
                            <div class="text-gray-600 dark:text-gray-400 text-xs mt-1 ml-7">
                                This option fetches the game list directly from Discord's servers. It may provide the
                                most
                                up-to-date information, but Discord may notice unusual traffic that you are requesting
                                to
                                the game list endpoint outside of the Discord webapp/client. This is only a precaution,
                                as
                                we are not entirely sure how Discord monitors such requests.
                            </div>
                            <div v-if="selectedSource === 'discord'" class="mt-4 ml-7">
                                <label for="discordApiVersion"
                                    class="block text-gray-700 dark:text-gray-300 mb-2 font-semibold">Discord API
                                    Version</label>
                                <select id="discordApiVersion" v-model="selectedApiVersion"
                                    class="w-full border border-gray-300 dark:border-gray-600 rounded-md p-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white">
                                    <option value="v9">v9</option>
                                    <option value="v10">v10</option>
                                    <option value="v11">v11</option>
                                </select>
                            </div>
                        </li>
                        <li>
                            <div class="flex items-center gap-3">
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="radio" name="gameListSource" value="proxy" v-model="selectedSource"
                                        class="accent-blue-500" />
                                    <span class="font-semibold text-gray-900 dark:text-white">Fetch from mirror</span>
                                </label>
                            </div>
                            <div class="text-gray-600 dark:text-gray-400 text-xs mt-1 ml-7">
                                This option fetches the gamelist JSON served on this repository's GitHub Pages. The
                                gamelist
                                is updated periodically to reflect changes from Discord. This avoids direct HTTP
                                requests to
                                Discord's servers.
                            </div>
                            <div v-if="selectedSource === 'proxy'" class="mt-4 ml-7">
                                <label for="proxyList"
                                    class="block text-gray-700 dark:text-gray-300 mb-2 font-semibold">Proxy List</label>
                                <select id="proxyList" v-model="selectedProxy"
                                    class="w-full border border-gray-300 dark:border-gray-600 rounded-md p-2 bg-white dark:bg-gray-700 text-gray-900 dark:text-white">
                                    <option value="proxy1">Proxy 1</option>
                                    <option value="proxy2">Proxy 2</option>
                                    <option value="proxy3">Proxy 3</option>
                                </select>
                            </div>
                        </li>
                    </ul>
                </section>
                <!-- Add more settings sections here as needed -->

                <section
                    class="bg-white dark:bg-gray-800 shadow rounded-xl p-8 border border-gray-100 dark:border-gray-700 mt-6">
                    <h2 class="text-xl font-semibold mb-4 text-gray-900 dark:text-white">Dummy Executable</h2>
                    <p class="text-gray-600 dark:text-gray-400 mb-6 text-sm">
                        Select which dummy executable to use for launching. Choose based on your system compatibility
                        and
                        file size preference.
                    </p>
                    <ul class="space-y-6 mb-2">
                        <li>
                            <div class="flex items-center gap-3">
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="radio" name="dummyExe" value="csharp" v-model="selectedDummyExe"
                                        class="accent-blue-500" />
                                    <span class="font-semibold text-gray-900 dark:text-white">C# (CSharp / .NET
                                        WinForms)</span>
                                </label>
                            </div>
                            <div class="text-gray-600 dark:text-gray-400 text-xs mt-1 ml-7">
                                Small file size (&lt; 13KB exe file), but requires .NET Framework 4.6 or later. May fail
                                to run if .NET
                                is not installed on your system.
                            </div>
                        </li>
                        <li>
                            <div class="flex items-center gap-3">
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="radio" name="dummyExe" value="rust" v-model="selectedDummyExe"
                                        class="accent-blue-500" />
                                    <span class="font-semibold text-gray-900 dark:text-white">Rust GUI (GUI )</span>
                                </label>
                            </div>
                            <div class="text-gray-600 dark:text-gray-400 text-xs mt-1 ml-7">
                                Written in Rust and calls native Windows APIs to create a simple GUI window. <br>
                                Executable is slightly larger file size (~160KB+ exe file).
                                If the C# option fails to run due to missing .NET Framework, try this one.
                            </div>
                        </li>

                    </ul>
                </section>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const selectedSource = ref('discord');
const selectedApiVersion = ref('v10');
const selectedProxy = ref('proxy1');
const selectedDummyExe = ref('rust');
</script>

<style scoped></style>