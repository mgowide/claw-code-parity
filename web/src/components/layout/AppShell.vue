<script setup lang="ts">
import { provide, watch } from 'vue'
import StatusBar from './StatusBar.vue'
import ToastContainer from './ToastContainer.vue'
import PermissionModal from '@/components/chat/PermissionModal.vue'
import SessionList from '@/components/sidebar/SessionList.vue'
import ModelSelector from '@/components/composer/ModelSelector.vue'
import RightPanel from '@/components/panels/RightPanel.vue'
import { useWebSocket } from '@/composables/useWebSocket'
import { useStream } from '@/composables/useStream'
import { useSessionStore } from '@/stores/sessionStore'
import { useUiStore } from '@/stores/uiStore'

const ws = useWebSocket()
const { handleEvent } = useStream()
const store = useSessionStore()
const uiStore = useUiStore()

// Sync connection status reactively
watch(ws.status, (s) => { store.connectionStatus = s }, { immediate: true })

// Sync connection status
ws.onEvent((event) => {
  handleEvent(event)
})

// Provide WebSocket to child components
provide('ws', ws)
</script>

<template>
  <div class="flex h-full w-full flex-col bg-(--bg-primary)">
    <!-- Main area -->
    <div class="flex min-h-0 flex-1">
      <!-- Sidebar — hidden on mobile unless open -->
      <aside
        v-if="uiStore.sidebarOpen"
        class="flex w-56 flex-shrink-0 flex-col border-r border-(--border) bg-(--bg-secondary) max-md:fixed max-md:inset-y-0 max-md:left-0 max-md:z-40 max-md:shadow-lg"
      >
        <SessionList />
      </aside>

      <!-- Mobile backdrop -->
      <div
        v-if="uiStore.sidebarOpen"
        class="fixed inset-0 z-30 hidden bg-black/40 md:hidden"
        @click="uiStore.sidebarOpen = false"
      />

      <!-- Router view with page transitions -->
      <main class="flex min-w-0 flex-1 flex-col">
        <router-view v-slot="{ Component }">
          <Transition name="page" mode="out-in">
            <component :is="Component" />
          </Transition>
        </router-view>
      </main>

      <!-- Right panel (Ctrl+Shift+R) — slide over on mobile -->
      <RightPanel v-if="uiStore.rightPanelOpen" />
    </div>

    <!-- Status bar -->
    <StatusBar />

    <!-- Permission modals -->
    <PermissionModal
      v-for="req in store.pendingPermissions"
      :key="req.id"
      :request="req"
    />

    <!-- Model selector overlay (Ctrl+K) -->
    <ModelSelector />

    <!-- Toast notifications -->
    <ToastContainer />
  </div>
</template>
