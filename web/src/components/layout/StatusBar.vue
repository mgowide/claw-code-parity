<script setup lang="ts">
import { useSessionStore } from '@/stores/sessionStore'
import { useUiStore } from '@/stores/uiStore'
import { computed } from 'vue'
import ThemePicker from './ThemePicker.vue'

const store = useSessionStore()
const uiStore = useUiStore()

const statusColor = computed(() => {
  switch (store.connectionStatus) {
    case 'connected':
      return 'bg-[var(--success)]'
    case 'connecting':
      return 'bg-[var(--warning)]'
    default:
      return 'bg-[var(--error)]'
  }
})

const formattedCost = computed(() => `$${store.cost.toFixed(4)}`)
</script>

<template>
  <footer class="flex items-center justify-between border-t border-(--border) bg-(--bg-secondary) px-4 py-1 text-xs text-(--text-secondary)">
    <div class="flex items-center gap-3">
      <span class="flex items-center gap-1.5">
        <span :class="['inline-block h-2 w-2 rounded-full', statusColor]" />
        {{ store.connectionStatus }}
      </span>
      <span class="text-(--text-muted)">|</span>
      <!-- Clickable model name opens model selector (Ctrl+K) -->
      <button
        class="rounded px-1 transition hover:bg-(--bg-primary) hover:text-(--text-primary)"
        :title="'Switch model (Ctrl+K)'"
        @click="uiStore.modelSelectorOpen = true"
      >
        {{ store.model }}
      </button>
    </div>

    <div class="flex items-center gap-3">
      <ThemePicker />
      <span class="text-(--text-muted)">|</span>
      <span>in: {{ store.inputTokens.toLocaleString() }}</span>
      <span>out: {{ store.outputTokens.toLocaleString() }}</span>
      <span v-if="store.cacheHits > 0">cache: {{ store.cacheHits.toLocaleString() }}</span>
      <span class="text-(--text-muted)">|</span>
      <span>{{ formattedCost }}</span>
    </div>
  </footer>
</template>
