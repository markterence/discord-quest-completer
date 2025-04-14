<script setup lang="ts">
// Layout component for consistent page structure
</script>

<template>
  <div class="min-h-screen bg-gray-100 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow-md">
      <div class="container mx-auto px-4 py-4 flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <img src="/tauri.svg" alt="Logo" class="h-8 w-8" />
          <h2 class="text-xl font-bold text-gray-900 dark:text-white">Game Launcher</h2>
        </div>
        <nav>
          <ul class="flex space-x-6">
            <li>
              <a href="#" class="text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400">Home</a>
            </li>
            <li>
              <a href="#" class="text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400">Library</a>
            </li>
            <li>
              <a href="#" class="text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400">Settings</a>
            </li>
          </ul>
        </nav>
      </div>
    </header>
    
    <main>
      <slot></slot>
    </main>
    
    <footer class="bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 mt-auto">
      <div class="container mx-auto px-4 py-4 text-center text-sm text-gray-600 dark:text-gray-400">
        &copy; 2025 Game Launcher - Built with Tauri + Vue
      </div>
    </footer>
  </div>
</template>