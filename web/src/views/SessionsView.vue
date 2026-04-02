<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useSession } from '@/composables/useSession'

type SortKey = 'updated_at' | 'cost' | 'message_count'

const { sessions, sessionsLoading, sessionsError, loadSessions, resumeSession, deleteSession, exportSession } =
  useSession()

const query = ref('')
const sortKey = ref<SortKey>('updated_at')
const sortAsc = ref(false)
const deleteConfirm = ref<string | null>(null) // session id pending delete
const exportingId = ref<string | null>(null)

const filtered = computed(() => {
  const q = query.value.trim().toLowerCase()
  const list = sessions.value.filter(
    (s) => !q || s.name.toLowerCase().includes(q) || s.model.toLowerCase().includes(q),
  )
  return [...list].sort((a, b) => {
    let diff = 0
    if (sortKey.value === 'updated_at') {
      diff = a.updated_at.localeCompare(b.updated_at)
    } else if (sortKey.value === 'cost') {
      diff = a.cost - b.cost
    } else {
      diff = a.message_count - b.message_count
    }
    return sortAsc.value ? diff : -diff
  })
})

function toggleSort(key: SortKey) {
  if (sortKey.value === key) {
    sortAsc.value = !sortAsc.value
  } else {
    sortKey.value = key
    sortAsc.value = false
  }
}

function sortIcon(key: SortKey) {
  if (sortKey.value !== key) return '↕'
  return sortAsc.value ? '↑' : '↓'
}

async function handleExport(id: string, format: 'md' | 'json') {
  exportingId.value = id
  try {
    await exportSession(id, format)
  } finally {
    exportingId.value = null
  }
}

async function handleDelete(id: string) {
  await deleteSession(id)
  deleteConfirm.value = null
}

function formatDate(ts: string): string {
  const n = Number(ts)
  const d = Number.isNaN(n) ? new Date(ts) : new Date(n * 1000)
  return d.toLocaleString(undefined, { dateStyle: 'medium', timeStyle: 'short' })
}

function formatCost(c: number): string {
  return c < 0.001 ? '<$0.001' : `$${c.toFixed(4)}`
}

onMounted(() => loadSessions())
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Header bar -->
    <div class="flex items-center justify-between border-b border-(--border) px-6 py-4">
      <h1 class="text-lg font-semibold text-(--text-primary)">Sessions</h1>
      <div class="flex items-center gap-3">
        <!-- Search -->
        <input
          v-model="query"
          placeholder="Search sessions…"
          class="w-56 rounded-md border border-(--border) bg-(--bg-secondary) px-3 py-1.5 text-sm text-(--text-primary) outline-none placeholder:text-(--text-muted) focus:border-(--accent)"
        />

        <!-- Sort buttons -->
        <div class="flex gap-1">
          <button
            v-for="(label, key) in ({ updated_at: 'Date', cost: 'Cost', message_count: 'Messages' } as Record<SortKey, string>)"
            :key="key"
            class="rounded px-2 py-1 text-xs transition"
            :class="[
              sortKey === key
                ? 'bg-(--accent)/15 text-(--accent)'
                : 'text-(--text-muted) hover:text-(--text-secondary)',
            ]"
            @click="toggleSort(key as SortKey)"
          >
            {{ label }} {{ sortIcon(key as SortKey) }}
          </button>
        </div>
      </div>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto px-6 py-4">
      <!-- Loading -->
      <div v-if="sessionsLoading" class="flex h-40 items-center justify-center">
        <span class="text-sm text-(--text-muted)">Loading sessions…</span>
      </div>

      <!-- Error -->
      <div v-else-if="sessionsError" class="rounded-md border border-red-500/30 bg-red-500/10 p-4 text-sm text-red-400">
        {{ sessionsError }}
        <button class="ml-2 underline hover:no-underline" @click="loadSessions()">Retry</button>
      </div>

      <!-- Empty -->
      <div v-else-if="filtered.length === 0" class="flex h-40 items-center justify-center">
        <span class="text-sm text-(--text-muted)">{{ query ? 'No matching sessions' : 'No sessions yet — start a chat!' }}</span>
      </div>

      <!-- Session grid -->
      <div v-else class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
        <div
          v-for="s in filtered"
          :key="s.id"
          class="group flex flex-col gap-3 rounded-xl border border-(--border) bg-(--bg-secondary) p-4 transition hover:border-(--accent)/50"
        >
          <!-- Name + model chip -->
          <div class="flex items-start justify-between gap-2">
            <h2 class="min-w-0 truncate text-sm font-medium text-(--text-primary)">{{ s.name }}</h2>
            <span class="shrink-0 rounded-full border border-(--border) px-2 py-0.5 text-[10px] text-(--text-muted)">
              {{ s.model.replace('claude-', '').replace(/-\d{8}$/, '') }}
            </span>
          </div>

          <!-- Stats row -->
          <dl class="grid grid-cols-3 gap-2 text-center text-xs">
            <div>
              <dt class="text-(--text-muted)">Messages</dt>
              <dd class="font-medium text-(--text-primary)">{{ s.message_count }}</dd>
            </div>
            <div>
              <dt class="text-(--text-muted)">Tokens</dt>
              <dd class="font-medium text-(--text-primary)">{{ (s.input_tokens + s.output_tokens).toLocaleString() }}</dd>
            </div>
            <div>
              <dt class="text-(--text-muted)">Cost</dt>
              <dd class="font-medium text-(--text-primary)">{{ formatCost(s.cost) }}</dd>
            </div>
          </dl>

          <!-- Date -->
          <p class="text-[10px] text-(--text-muted)">Updated {{ formatDate(s.updated_at) }}</p>

          <!-- Actions -->
          <div class="flex gap-2">
            <button
              class="flex-1 rounded-md bg-(--accent) px-2 py-1.5 text-xs font-medium text-white transition hover:opacity-80"
              @click="resumeSession(s.id)"
            >
              Resume
            </button>

            <!-- Export dropdown (Markdown / JSON) -->
            <div class="relative">
              <select
                class="h-full cursor-pointer rounded-md border border-(--border) bg-(--bg-primary) px-1.5 py-1 text-xs text-(--text-secondary) outline-none transition hover:border-(--accent)"
                :disabled="exportingId === s.id"
                @change="(e) => { const v = (e.target as HTMLSelectElement).value as 'md'|'json'; if(v) handleExport(s.id, v); (e.target as HTMLSelectElement).value = '' }"
              >
                <option value="" disabled selected>Export</option>
                <option value="md">Markdown</option>
                <option value="json">JSON</option>
              </select>
            </div>

            <!-- Delete -->
            <button
              class="rounded-md border border-(--border) px-2 py-1 text-xs text-red-400 transition hover:border-red-400"
              @click="deleteConfirm = s.id"
            >
              Delete
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Delete confirmation modal -->
    <Teleport to="body">
      <div
        v-if="deleteConfirm"
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
        @click.self="deleteConfirm = null"
      >
        <div class="w-80 rounded-xl border border-(--border) bg-(--bg-secondary) p-6 shadow-2xl">
          <h3 class="mb-2 text-base font-semibold text-(--text-primary)">Delete session?</h3>
          <p class="mb-4 text-sm text-(--text-muted)">This action cannot be undone.</p>
          <div class="flex justify-end gap-2">
            <button
              class="rounded-md border border-(--border) px-3 py-1.5 text-sm text-(--text-secondary) transition hover:border-(--accent)"
              @click="deleteConfirm = null"
            >
              Cancel
            </button>
            <button
              class="rounded-md bg-red-500 px-3 py-1.5 text-sm font-medium text-white transition hover:bg-red-600"
              @click="deleteConfirm && handleDelete(deleteConfirm)"
            >
              Delete
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>
