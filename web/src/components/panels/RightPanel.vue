<script setup lang="ts">
import TokenCounter from './TokenCounter.vue'
import CostTracker from './CostTracker.vue'
import TodoBoard from './TodoBoard.vue'
import { useUiStore } from '@/stores/uiStore'

const uiStore = useUiStore()

const tabs = [
  { key: 'tokens', label: 'Tokens' },
  { key: 'cost', label: 'Cost' },
  { key: 'todos', label: 'Todos' },
] as const
</script>

<template>
  <aside
    class="flex flex-col border-l border-(--border) bg-(--bg-secondary)"
    :style="{ width: uiStore.rightPanelWidth + 'px' }"
  >
    <!-- Header: tabs + collapse -->
    <div class="flex items-center justify-between border-b border-(--border) px-2">
      <div class="flex">
        <button
          v-for="tab in tabs"
          :key="tab.key"
          class="px-3 py-2.5 text-xs font-medium transition"
          :class="
            uiStore.rightPanelTab === tab.key
              ? 'border-b-2 border-(--accent) text-(--accent)'
              : 'text-(--text-muted) hover:text-(--text-primary)'
          "
          @click="uiStore.rightPanelTab = tab.key"
        >
          {{ tab.label }}
        </button>
      </div>

      <!-- Collapse -->
      <button
        class="rounded-md p-1 text-(--text-muted) hover:bg-(--bg-primary) hover:text-(--text-primary)"
        title="Close panel (Ctrl+Shift+R)"
        @click="uiStore.rightPanelOpen = false"
      >
        <svg class="h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
          <path d="M9 2.5l-5 5.5 5 5.5" stroke="currentColor" stroke-width="1.5" fill="none" stroke-linecap="round" />
        </svg>
      </button>
    </div>

    <!-- Panel content -->
    <div class="flex-1 overflow-y-auto">
      <TokenCounter v-if="uiStore.rightPanelTab === 'tokens'" />
      <CostTracker v-else-if="uiStore.rightPanelTab === 'cost'" />
      <TodoBoard v-else-if="uiStore.rightPanelTab === 'todos'" />
    </div>
  </aside>
</template>
