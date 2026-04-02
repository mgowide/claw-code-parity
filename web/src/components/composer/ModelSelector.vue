<script setup lang="ts">
import { ref, onMounted, inject } from 'vue'
import { getModels } from '@/lib/api'
import type { ModelInfo } from '@/lib/api'
import { useSessionStore } from '@/stores/sessionStore'
import { useUiStore } from '@/stores/uiStore'
import type { useWebSocket } from '@/composables/useWebSocket'

const uiStore = useUiStore()
const store = useSessionStore()
const ws = inject<ReturnType<typeof useWebSocket>>('ws')!

const models = ref<ModelInfo[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

const PROVIDER_LABELS: Record<string, string> = {
  anthropic: 'Anthropic',
  xai: 'xAI',
  'openai-compat': 'OpenAI Compatible',
}

// Group models by provider
function grouped(list: ModelInfo[]): Record<string, ModelInfo[]> {
  const g: Record<string, ModelInfo[]> = {}
  for (const m of list) {
    const p = m.provider
    if (!g[p]) g[p] = []
    g[p].push(m)
  }
  return g
}

function selectModel(id: string) {
  store.model = id
  ws.send({ type: 'switch_model', model: id })
  uiStore.modelSelectorOpen = false
}

onMounted(async () => {
  try {
    models.value = await getModels()
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load models'
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <Teleport to="body">
    <div
      v-if="uiStore.modelSelectorOpen"
      class="fixed inset-0 z-50 flex items-start justify-center bg-black/60 pt-24 backdrop-blur-sm"
      @click.self="uiStore.modelSelectorOpen = false"
    >
      <div class="w-96 rounded-xl border border-(--border) bg-(--bg-secondary) shadow-2xl">
        <!-- Header -->
        <div class="flex items-center justify-between border-b border-(--border) px-5 py-4">
          <h2 class="text-sm font-semibold text-(--text-primary)">Switch Model</h2>
          <kbd class="rounded border border-(--border) px-1.5 py-0.5 text-[10px] text-(--text-muted)">
            Ctrl K
          </kbd>
        </div>

        <!-- Content -->
        <div class="p-2">
          <div v-if="loading" class="py-8 text-center text-sm text-(--text-muted)">Loading…</div>
          <div v-else-if="error" class="py-4 text-center text-sm text-red-400">{{ error }}</div>
          <template v-else>
            <template v-for="(list, provider) in grouped(models)" :key="provider">
              <div class="px-3 pt-2 pb-1 text-[10px] font-semibold uppercase tracking-wider text-(--text-muted)">
                {{ PROVIDER_LABELS[provider] ?? provider }}
              </div>
              <button
                v-for="m in list"
                :key="m.id"
                class="flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-left transition hover:bg-(--bg-primary)"
                @click="selectModel(m.id)"
              >
                <span
                  class="inline-block h-2 w-2 shrink-0 rounded-full"
                  :class="m.active ? 'bg-(--accent)' : 'bg-transparent border border-(--border)'"
                />
                <span class="flex-1 text-sm text-(--text-primary)">{{ m.id }}</span>
                <span v-if="m.active" class="text-[10px] text-(--accent)">active</span>
              </button>
            </template>
          </template>
        </div>
      </div>
    </div>
  </Teleport>
</template>
