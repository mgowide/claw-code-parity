<script setup lang="ts">
import { computed } from 'vue'
import { useSessionStore } from '@/stores/sessionStore'

const store = useSessionStore()

const contextLimit = 200_000

const pct = computed(() =>
  Math.min(100, Math.round(((store.inputTokens + store.outputTokens) / contextLimit) * 100)),
)

function fmt(n: number): string {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`
  if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`
  return String(n)
}
</script>

<template>
  <div class="flex flex-col gap-3 p-4 text-xs text-(--text-primary)">
    <h3 class="text-[10px] font-semibold uppercase tracking-widest text-(--text-muted)">
      Token Usage
    </h3>

    <!-- Bars -->
    <div class="flex flex-col gap-2">
      <div class="flex flex-col gap-0.5">
        <div class="flex justify-between text-(--text-secondary)">
          <span>Input</span>
          <span>{{ fmt(store.inputTokens) }}</span>
        </div>
        <div class="h-1.5 w-full overflow-hidden rounded-full bg-(--bg-primary)">
          <div
            class="h-full rounded-full bg-blue-500 transition-all"
            :style="{ width: Math.min(100, (store.inputTokens / contextLimit) * 100) + '%' }"
          />
        </div>
      </div>

      <div class="flex flex-col gap-0.5">
        <div class="flex justify-between text-(--text-secondary)">
          <span>Output</span>
          <span>{{ fmt(store.outputTokens) }}</span>
        </div>
        <div class="h-1.5 w-full overflow-hidden rounded-full bg-(--bg-primary)">
          <div
            class="h-full rounded-full bg-emerald-500 transition-all"
            :style="{ width: Math.min(100, (store.outputTokens / contextLimit) * 100) + '%' }"
          />
        </div>
      </div>

      <div class="flex flex-col gap-0.5">
        <div class="flex justify-between text-(--text-secondary)">
          <span>Cache hits</span>
          <span>{{ fmt(store.cacheHits) }}</span>
        </div>
        <div class="h-1.5 w-full overflow-hidden rounded-full bg-(--bg-primary)">
          <div
            class="h-full rounded-full bg-amber-500 transition-all"
            :style="{ width: Math.min(100, (store.cacheHits / contextLimit) * 100) + '%' }"
          />
        </div>
      </div>
    </div>

    <!-- Context gauge -->
    <div class="mt-1 flex flex-col gap-1">
      <div class="flex justify-between text-(--text-muted)">
        <span>Context used</span>
        <span>{{ pct }}%</span>
      </div>
      <div class="h-2.5 w-full overflow-hidden rounded-full bg-(--bg-primary)">
        <div
          class="h-full rounded-full transition-all"
          :class="pct > 80 ? 'bg-red-500' : pct > 50 ? 'bg-amber-400' : 'bg-blue-500'"
          :style="{ width: pct + '%' }"
        />
      </div>
      <div class="text-[10px] text-(--text-muted)">of {{ fmt(contextLimit) }} limit</div>
    </div>
  </div>
</template>
