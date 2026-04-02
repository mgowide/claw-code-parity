<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getMcpServers, addMcpServer, removeMcpServer } from '@/lib/api'
import type { McpServer } from '@/lib/api'

const servers = ref<McpServer[]>([])
const loading = ref(false)
const error = ref('')
const showAdd = ref(false)
const addError = ref('')
const adding = ref(false)

const form = ref({
  name: '',
  transport: 'stdio' as 'stdio' | 'sse',
  command: '',
  url: '',
})

async function load() {
  loading.value = true
  error.value = ''
  try {
    servers.value = await getMcpServers()
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load MCP servers'
  } finally {
    loading.value = false
  }
}

async function handleAdd() {
  addError.value = ''
  if (!form.value.name.trim()) {
    addError.value = 'Name is required'
    return
  }
  if (form.value.transport === 'stdio' && !form.value.command.trim()) {
    addError.value = 'Command is required for stdio transport'
    return
  }
  if (form.value.transport === 'sse' && !form.value.url.trim()) {
    addError.value = 'URL is required for SSE transport'
    return
  }
  adding.value = true
  try {
    const body = {
      name: form.value.name.trim(),
      transport: form.value.transport,
      ...(form.value.transport === 'stdio' ? { command: form.value.command.trim() } : {}),
      ...(form.value.transport === 'sse' ? { url: form.value.url.trim() } : {}),
    }
    const server = await addMcpServer(body)
    servers.value.push(server)
    showAdd.value = false
    form.value = { name: '', transport: 'stdio', command: '', url: '' }
  } catch (e) {
    addError.value = e instanceof Error ? e.message : 'Failed to add server'
  } finally {
    adding.value = false
  }
}

async function handleRemove(id: string) {
  try {
    await removeMcpServer(id)
    servers.value = servers.value.filter((s) => s.id !== id)
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to remove server'
  }
}

const statusColor: Record<string, string> = {
  connected: 'bg-emerald-500',
  connecting: 'bg-amber-400',
  error: 'bg-red-500',
  disconnected: 'bg-gray-500',
}

onMounted(load)
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Header -->
    <div class="border-b border-(--border) bg-(--bg-secondary) px-6 py-4">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-sm font-semibold text-(--text-primary)">MCP Servers</h1>
          <p class="mt-0.5 text-xs text-(--text-muted)">Model Context Protocol connections</p>
        </div>
        <button
          class="rounded-md bg-(--accent) px-3 py-1.5 text-xs font-medium text-white transition hover:opacity-90"
          @click="showAdd = true"
        >
          + Add server
        </button>
      </div>
    </div>

    <!-- Add server dialog -->
    <div
      v-if="showAdd"
      class="border-b border-(--border) bg-(--bg-secondary) px-6 py-4"
    >
      <h2 class="mb-3 text-xs font-semibold text-(--text-primary)">Add MCP Server</h2>
      <div class="flex flex-col gap-3">
        <div class="flex gap-3">
          <!-- Name -->
          <div class="flex flex-1 flex-col gap-1">
            <label class="text-[10px] text-(--text-muted)">Name</label>
            <input
              v-model="form.name"
              placeholder="my-mcp-server"
              class="rounded-md border border-(--border) bg-(--bg-primary) px-2.5 py-1.5 text-xs text-(--text-primary) outline-none focus:border-(--accent)"
            />
          </div>
          <!-- Transport -->
          <div class="flex flex-col gap-1">
            <label class="text-[10px] text-(--text-muted)">Transport</label>
            <select
              v-model="form.transport"
              class="rounded-md border border-(--border) bg-(--bg-primary) px-2.5 py-1.5 text-xs text-(--text-primary) outline-none"
            >
              <option value="stdio">stdio</option>
              <option value="sse">SSE</option>
            </select>
          </div>
        </div>

        <!-- Command or URL -->
        <div v-if="form.transport === 'stdio'" class="flex flex-col gap-1">
          <label class="text-[10px] text-(--text-muted)">Command</label>
          <input
            v-model="form.command"
            placeholder="npx my-mcp-server"
            class="rounded-md border border-(--border) bg-(--bg-primary) px-2.5 py-1.5 font-mono text-xs text-(--text-primary) outline-none focus:border-(--accent)"
          />
        </div>
        <div v-else class="flex flex-col gap-1">
          <label class="text-[10px] text-(--text-muted)">URL</label>
          <input
            v-model="form.url"
            placeholder="http://localhost:8080/sse"
            class="rounded-md border border-(--border) bg-(--bg-primary) px-2.5 py-1.5 font-mono text-xs text-(--text-primary) outline-none focus:border-(--accent)"
          />
        </div>

        <p v-if="addError" class="text-xs text-red-400">{{ addError }}</p>

        <div class="flex gap-2">
          <button
            class="rounded-md bg-(--accent) px-4 py-1.5 text-xs font-medium text-white transition hover:opacity-90 disabled:opacity-50"
            :disabled="adding"
            @click="handleAdd"
          >
            {{ adding ? 'Adding…' : 'Add' }}
          </button>
          <button
            class="rounded-md border border-(--border) px-4 py-1.5 text-xs text-(--text-secondary) transition hover:text-(--text-primary)"
            @click="showAdd = false"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>

    <!-- Server list -->
    <div class="flex-1 overflow-y-auto p-6">
      <div v-if="loading" class="text-xs text-(--text-muted)">Loading…</div>
      <div v-else-if="error" class="text-xs text-red-400">{{ error }}</div>
      <div v-else-if="servers.length === 0" class="py-12 text-center text-sm text-(--text-muted)">
        No MCP servers configured
      </div>
      <div v-else class="flex flex-col gap-3">
        <div
          v-for="s in servers"
          :key="s.id"
          class="rounded-lg border border-(--border) bg-(--bg-secondary) p-4"
        >
          <div class="flex items-start justify-between gap-3">
            <div class="flex flex-col gap-1">
              <!-- Name + status -->
              <div class="flex items-center gap-2">
                <span
                  class="h-2 w-2 rounded-full"
                  :class="statusColor[s.status] ?? 'bg-gray-500'"
                />
                <span class="text-sm font-medium text-(--text-primary)">{{ s.name }}</span>
                <span class="rounded bg-(--bg-primary) px-1.5 py-0.5 text-[10px] font-mono text-(--text-muted)">
                  {{ s.transport }}
                </span>
              </div>

              <!-- Command / URL -->
              <span class="font-mono text-[11px] text-(--text-muted)">
                {{ s.command ?? s.url }}
              </span>

              <!-- Error -->
              <span v-if="s.error" class="text-[11px] text-red-400">{{ s.error }}</span>

              <!-- Tools -->
              <div v-if="s.tools?.length" class="mt-1 flex flex-wrap gap-1">
                <span
                  v-for="t in s.tools"
                  :key="t.name"
                  class="rounded bg-(--bg-primary) px-1.5 py-0.5 text-[10px] text-(--text-secondary)"
                >
                  {{ t.name }}
                </span>
              </div>
            </div>

            <!-- Remove -->
            <button
              class="shrink-0 rounded-md border border-(--border) px-2.5 py-1 text-[11px] text-red-400 transition hover:border-red-500 hover:bg-red-500/10"
              @click="handleRemove(s.id)"
            >
              Remove
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
