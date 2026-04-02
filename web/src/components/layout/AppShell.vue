<script setup lang="ts">
import { provide } from 'vue'
import StatusBar from './StatusBar.vue'
import PermissionModal from '@/components/chat/PermissionModal.vue'
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
      <!-- Sidebar stub -->
      <aside class="flex w-56 flex-col border-r border-(--border) bg-(--bg-secondary)">
        <div class="flex items-center gap-2 border-b border-(--border) px-4 py-3">
          <div class="h-6 w-6 rounded-md bg-(--accent)" />
          <span class="text-sm font-semibold text-(--text-primary)">Claw</span>
        </div>
        <div class="p-3">
          <button
            class="w-full rounded-md border border-(--border) px-3 py-1.5 text-xs text-(--text-secondary) transition hover:border-(--accent) hover:text-(--text-primary)"
            @click="store.reset()"
          >
            + New Chat
          </button>
        </div>
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
