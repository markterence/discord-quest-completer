<script setup lang="ts">
import MainLayout from './components/MainLayout.vue';
import { Pages, useGlobalState } from './composables/app-state';
import HomeViewV2 from './pages/HomeViewV2.vue';
import Playground from './pages/Playground.vue';

const appState = useGlobalState();
const { page } = appState;

</script>

<template>
  <MainLayout>
    <HomeViewV2 v-show="page === Pages.HOME"/>
    <Playground v-show="page === Pages.PLAYGROUND"/>
  </MainLayout>
</template>

<style>
/* Global styles are managed by Tailwind CSS */
</style>