<script setup lang="ts">
import { useSessionStore } from '@/stores/sessionStore'
import { computed } from 'vue'

const store = useSessionStore()

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
      <span>{{ store.model }}</span>
    </div>

    <div class="flex items-center gap-3">
      <span>in: {{ store.inputTokens.toLocaleString() }}</span>
      <span>out: {{ store.outputTokens.toLocaleString() }}</span>
      <span v-if="store.cacheHits > 0">cache: {{ store.cacheHits.toLocaleString() }}</span>
      <span class="text-(--text-muted)">|</span>
      <span>{{ formattedCost }}</span>
    </div>
  </footer>
</template>
