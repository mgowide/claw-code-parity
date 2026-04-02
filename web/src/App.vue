<script setup lang="ts">
import { ref } from 'vue'
import AppShell from './components/layout/AppShell.vue'
import SplashScreen from './components/visuals/SplashScreen.vue'
import { useKeyboardShortcuts } from '@/composables/useKeyboardShortcuts'
import { useTheme } from '@/composables/useTheme'
import { useServiceWorker } from '@/composables/useServiceWorker'

useKeyboardShortcuts()
useTheme() // applies persisted theme on mount
useServiceWorker() // registers SW

const splashDone = ref(sessionStorage.getItem('claw-splashed') === '1')

function onSplashDone() {
  splashDone.value = true
  sessionStorage.setItem('claw-splashed', '1')
}
</script>

<template>
  <SplashScreen v-if="!splashDone" @done="onSplashDone" />
  <AppShell v-else />
</template>
