<script setup lang="ts">
import { computed } from 'vue'
import { useSessionStore } from '@/stores/sessionStore'

const store = useSessionStore()

const inputCost = computed(() => store.inputTokens * 0.000003)
const outputCost = computed(() => store.outputTokens * 0.000015)
const cacheCost = computed(() => store.cacheHits * 0.0000003)
const total = computed(() => store.cost || inputCost.value + outputCost.value + cacheCost.value)

function usd(n: number): string {
  if (n < 0.001) return `$${(n * 1000).toFixed(3)}m`
  return `$${n.toFixed(4)}`
}
</script>

<template>
  <div class="flex flex-col gap-3 p-4 text-xs text-(--text-primary)">
    <h3 class="text-[10px] font-semibold uppercase tracking-widest text-(--text-muted)">
      Cost Tracker
    </h3>

    <!-- Total -->
    <div class="rounded-md bg-(--bg-primary) px-3 py-2">
      <div class="text-[10px] text-(--text-muted)">Session Total</div>
      <div class="mt-0.5 text-lg font-semibold text-(--accent)">{{ usd(total) }}</div>
    </div>

    <!-- Breakdown -->
    <div class="flex flex-col gap-1.5">
      <div class="flex items-center justify-between">
        <span class="flex items-center gap-1.5">
          <span class="inline-block h-2 w-2 rounded-full bg-blue-500" />
          Input tokens
        </span>
        <span class="text-(--text-secondary)">{{ usd(inputCost) }}</span>
      </div>
      <div class="flex items-center justify-between">
        <span class="flex items-center gap-1.5">
          <span class="inline-block h-2 w-2 rounded-full bg-emerald-500" />
          Output tokens
        </span>
        <span class="text-(--text-secondary)">{{ usd(outputCost) }}</span>
      </div>
      <div class="flex items-center justify-between">
        <span class="flex items-center gap-1.5">
          <span class="inline-block h-2 w-2 rounded-full bg-amber-500" />
          Cache reads
        </span>
        <span class="text-(--text-secondary)">{{ usd(cacheCost) }}</span>
      </div>
    </div>

    <!-- Per-model rate note -->
    <p class="text-[10px] text-(--text-muted)">
      Estimated for claude-sonnet-4. Actual billing may differ.
    </p>
  </div>
</template>
