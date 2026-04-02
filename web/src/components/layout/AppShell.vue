<script setup lang="ts">
import { provide } from 'vue'
import StatusBar from './StatusBar.vue'
import PermissionModal from '@/components/chat/PermissionModal.vue'
import SessionList from '@/components/sidebar/SessionList.vue'
import { useWebSocket } from '@/composables/useWebSocket'
import { useStream } from '@/composables/useStream'
import { useSessionStore } from '@/stores/sessionStore'

const ws = useWebSocket()
const { handleEvent } = useStream()
const store = useSessionStore()

// Sync connection status
ws.onEvent((event) => {
  handleEvent(event)
})

// Keep store status in sync with WebSocket status
store.connectionStatus = ws.status.value

// Provide WebSocket to child components
provide('ws', ws)
</script>

<template>
  <div class="flex h-full w-full flex-col bg-(--bg-primary)">
    <!-- Main area -->
    <div class="flex min-h-0 flex-1">
      <!-- Sidebar -->
      <aside class="flex w-56 flex-col border-r border-(--border) bg-(--bg-secondary)">
        <SessionList />
      </aside>

      <!-- Router view -->
      <main class="flex min-w-0 flex-1 flex-col">
        <router-view />
      </main>
    </div>

    <!-- Status bar -->
    <StatusBar />

    <!-- Permission modals -->
    <PermissionModal
      v-for="req in store.pendingPermissions"
      :key="req.id"
      :request="req"
    />
  </div>
</template>
