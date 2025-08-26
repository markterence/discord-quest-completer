// Do not include this on auto module install

import { createRouter, createWebHistory } from 'vue-router'
import { routes, handleHotUpdate  } from 'vue-router/auto-routes';

export const router = createRouter({
    history: createWebHistory(),
    routes
});

if (import.meta.hot) { 
  handleHotUpdate(router) 
} 