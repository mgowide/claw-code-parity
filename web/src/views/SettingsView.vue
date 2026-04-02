<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getConfig, putConfig } from '@/lib/api'

const config = ref<Record<string, unknown>>({})
const rawJson = ref('')
const rawMode = ref(false)
const loading = ref(false)
const saving = ref(false)
const error = ref('')
const saveMsg = ref('')
const jsonError = ref('')

async function load() {
  loading.value = true
  error.value = ''
  try {
    config.value = await getConfig()
    rawJson.value = JSON.stringify(config.value, null, 2)
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load config'
  } finally {
    loading.value = false
  }
}

function toggleMode() {
  if (!rawMode.value) {
    // switching to raw — sync from form
    rawJson.value = JSON.stringify(config.value, null, 2)
  } else {
    // switching to form — parse raw
    try {
      config.value = JSON.parse(rawJson.value)
      jsonError.value = ''
    } catch {
      jsonError.value = 'Invalid JSON — fix before switching'
      return
    }
  }
  rawMode.value = !rawMode.value
}

async function save() {
  saveMsg.value = ''
  jsonError.value = ''
  let payload = config.value
  if (rawMode.value) {
    try {
      payload = JSON.parse(rawJson.value)
    } catch {
      jsonError.value = 'Invalid JSON'
      return
    }
  }
  saving.value = true
  try {
    await putConfig(payload)
    saveMsg.value = 'Saved successfully'
    config.value = payload
    setTimeout(() => (saveMsg.value = ''), 2000)
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to save'
  } finally {
    saving.value = false
  }
}

function getStr(key: string): string {
  return typeof config.value[key] === 'string' ? (config.value[key] as string) : ''
}
function setStr(key: string, val: string) {
  config.value = { ...config.value, [key]: val }
}

onMounted(load)
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Header -->
    <div class="border-b border-(--border) bg-(--bg-secondary) px-6 py-4">
      <h1 class="text-sm font-semibold text-(--text-primary)">Settings</h1>
      <p class="mt-0.5 text-xs text-(--text-muted)">.claude.json workspace configuration</p>
    </div>

    <div class="flex-1 overflow-y-auto p-6">
      <div v-if="loading" class="text-xs text-(--text-muted)">Loading…</div>
      <div v-else-if="error" class="mb-4 rounded-md bg-red-500/10 px-3 py-2 text-xs text-red-400">
        {{ error }}
      </div>

      <div v-else class="mx-auto max-w-2xl flex flex-col gap-6">
        <!-- Mode toggle -->
        <div class="flex items-center justify-between">
          <h2 class="text-xs font-semibold text-(--text-secondary)">
            {{ rawMode ? 'Raw JSON' : 'Form editor' }}
          </h2>
          <button
            class="rounded-md border border-(--border) px-3 py-1 text-xs text-(--text-secondary) transition hover:border-(--accent) hover:text-(--text-primary)"
            @click="toggleMode"
          >
            {{ rawMode ? 'Switch to form' : 'Edit raw JSON' }}
          </button>
        </div>

        <!-- Raw JSON editor -->
        <div v-if="rawMode">
          <textarea
            v-model="rawJson"
            rows="20"
            spellcheck="false"
            class="w-full rounded-md border border-(--border) bg-(--bg-primary) p-3 font-mono text-xs text-(--text-primary) outline-none focus:border-(--accent)"
          />
          <p v-if="jsonError" class="mt-1 text-xs text-red-400">{{ jsonError }}</p>
        </div>

        <!-- Form fields -->
        <div v-else class="flex flex-col gap-4">
          <!-- model -->
          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-(--text-secondary)">model</label>
            <input
              :value="getStr('model')"
              placeholder="claude-sonnet-4-20250514"
              class="rounded-md border border-(--border) bg-(--bg-primary) px-3 py-2 text-xs text-(--text-primary) outline-none focus:border-(--accent)"
              @input="(e) => setStr('model', (e.target as HTMLInputElement).value)"
            />
          </div>

          <!-- permission_mode -->
          <div class="flex flex-col gap-1.5">
            <label class="text-xs font-medium text-(--text-secondary)">permissionMode</label>
            <select
              :value="getStr('permissionMode') || 'default'"
              class="rounded-md border border-(--border) bg-(--bg-primary) px-3 py-2 text-xs text-(--text-primary) outline-none"
              @change="(e) => setStr('permissionMode', (e.target as HTMLSelectElement).value)"
            >
              <option value="default">default</option>
              <option value="acceptEdits">acceptEdits</option>
              <option value="bypassPermissions">bypassPermissions</option>
            </select>
          </div>

          <!-- Raw JSON fallback for unrecognized keys -->
          <div class="rounded-md border border-(--border) bg-(--bg-primary) p-3">
            <p class="mb-2 text-xs text-(--text-muted)">Other settings (raw JSON preview):</p>
            <pre class="text-[11px] text-(--text-secondary)">{{ JSON.stringify(config, null, 2) }}</pre>
          </div>
        </div>

        <!-- Save bar -->
        <div class="flex items-center gap-3">
          <button
            class="rounded-md bg-(--accent) px-4 py-1.5 text-xs font-medium text-white transition hover:opacity-90 disabled:opacity-50"
            :disabled="saving"
            @click="save"
          >
            {{ saving ? 'Saving…' : 'Save' }}
          </button>
          <span v-if="saveMsg" class="text-xs text-emerald-400">{{ saveMsg }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
