<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getTools } from '@/lib/api'
import type { ToolEntry } from '@/lib/api'

const tools = ref<ToolEntry[]>([])
const loading = ref(false)
const error = ref('')
const search = ref('')
const permFilter = ref('')

const permissions = computed(() => ['', ...new Set(tools.value.map((t) => t.permission))])

const filtered = computed(() => {
  const q = search.value.toLowerCase()
  return tools.value.filter(
    (t) =>
      (!q || t.name.includes(q) || t.description.toLowerCase().includes(q)) &&
      (!permFilter.value || t.permission === permFilter.value),
  )
})

const maxCalls = computed(() => Math.max(1, ...tools.value.map((t) => t.call_count)))

const permColor: Record<string, string> = {
  read: 'bg-blue-500/20 text-blue-400',
  write: 'bg-amber-500/20 text-amber-400',
  execute: 'bg-red-500/20 text-red-400',
  network: 'bg-purple-500/20 text-purple-400',
  builtin: 'bg-emerald-500/20 text-emerald-400',
}

async function load() {
  loading.value = true
  error.value = ''
  try {
    tools.value = await getTools()
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load'
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Header -->
    <div class="border-b border-(--border) bg-(--bg-secondary) px-6 py-4">
      <h1 class="text-sm font-semibold text-(--text-primary)">Tool Registry</h1>
      <p class="mt-0.5 text-xs text-(--text-muted)">{{ tools.length }} built-in tools</p>
    </div>

    <!-- Filters -->
    <div class="flex items-center gap-3 border-b border-(--border) bg-(--bg-secondary) px-6 py-2">
      <input
        v-model="search"
        placeholder="Search tools…"
        class="w-48 rounded-md border border-(--border) bg-(--bg-primary) px-2.5 py-1.5 text-xs text-(--text-primary) outline-none placeholder:text-(--text-muted) focus:border-(--accent)"
      />
      <select
        v-model="permFilter"
        class="rounded-md border border-(--border) bg-(--bg-primary) px-2 py-1.5 text-xs text-(--text-primary) outline-none"
      >
        <option value="">All permissions</option>
        <option v-for="p in permissions.slice(1)" :key="p" :value="p">{{ p }}</option>
      </select>
    </div>

    <!-- Tool list -->
    <div class="flex-1 overflow-y-auto p-6">
      <div v-if="loading" class="text-xs text-(--text-muted)">Loading…</div>
      <div v-else-if="error" class="text-xs text-red-400">{{ error }}</div>
      <div v-else class="grid grid-cols-1 gap-3 md:grid-cols-2 lg:grid-cols-3">
        <div
          v-for="tool in filtered"
          :key="tool.name"
          class="flex flex-col gap-2 rounded-lg border border-(--border) bg-(--bg-secondary) p-4"
        >
          <!-- Name + permission badge -->
          <div class="flex items-start justify-between gap-2">
            <span class="font-mono text-sm font-medium text-(--text-primary)">{{ tool.name }}</span>
            <span
              class="shrink-0 rounded px-1.5 py-0.5 text-[10px] font-semibold"
              :class="permColor[tool.permission] ?? 'bg-(--bg-primary) text-(--text-muted)'"
            >
              {{ tool.permission }}
            </span>
          </div>

          <!-- Description -->
          <p class="text-[11px] leading-relaxed text-(--text-secondary)">{{ tool.description }}</p>

          <!-- Usage bar -->
          <div class="mt-auto flex items-center gap-2">
            <div class="h-1 flex-1 overflow-hidden rounded-full bg-(--bg-primary)">
              <div
                class="h-full rounded-full bg-(--accent)/60 transition-all"
                :style="{ width: (tool.call_count / maxCalls) * 100 + '%' }"
              />
            </div>
            <span class="text-[10px] text-(--text-muted)">{{ tool.call_count }} calls</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
