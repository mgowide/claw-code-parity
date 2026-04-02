<script setup lang="ts">
import { ref, inject } from 'vue'
import { useSessionStore } from '@/stores/sessionStore'
import type { PermissionRequest } from '@/types/events'
import type { useWebSocket } from '@/composables/useWebSocket'

const props = defineProps<{
  request: PermissionRequest
}>()

const emit = defineEmits<{
  resolved: [requestId: string]
}>()

const store = useSessionStore()
const ws = inject<ReturnType<typeof useWebSocket>>('ws')!
const alwaysAllow = ref(false)

const riskLevel = getRiskLevel(props.request.tool)

function getRiskLevel(tool: string): 'low' | 'medium' | 'high' {
  const highRisk = ['bash', 'write_file', 'edit_file']
  const mediumRisk = ['read_file', 'glob_search', 'grep_search']
  if (highRisk.includes(tool)) return 'high'
  if (mediumRisk.includes(tool)) return 'medium'
  return 'low'
}

function approve() {
  if (alwaysAllow.value) {
    store.alwaysAllowedTools.add(props.request.tool)
  }
  ws.send({ type: 'approve_permission', request_id: props.request.id })
  store.resolvePermission(props.request.id)
  emit('resolved', props.request.id)
}

function deny() {
  ws.send({ type: 'deny_permission', request_id: props.request.id })
  store.resolvePermission(props.request.id)
  emit('resolved', props.request.id)
}
</script>

<template>
  <Teleport to="body">
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60">
      <div class="w-full max-w-md rounded-xl border border-(--border) bg-(--bg-secondary) p-6 shadow-2xl">
        <!-- Header -->
        <div class="mb-4 flex items-center gap-2 text-lg font-semibold text-(--text-primary)">
          <span>⚠️</span>
          <span>Permission Request</span>
        </div>

        <!-- Tool info -->
        <div class="mb-3 flex items-center gap-2 text-sm">
          <span class="text-(--text-secondary)">Tool:</span>
          <span class="font-mono font-medium text-(--text-primary)">{{ request.tool }}</span>
          <span
            :class="[
              'rounded px-1.5 py-0.5 text-[10px] font-bold uppercase',
              riskLevel === 'high' ? 'bg-red-500/20 text-red-400'
                : riskLevel === 'medium' ? 'bg-yellow-500/20 text-yellow-400'
                : 'bg-green-500/20 text-green-400',
            ]"
          >
            {{ riskLevel }}
          </span>
        </div>

        <!-- Description -->
        <div class="mb-4 rounded-lg bg-(--bg-primary) p-3 text-xs text-(--text-secondary)">
          <p class="mb-1 text-(--text-muted)">The assistant wants to:</p>
          <p class="font-mono text-(--text-primary)">{{ request.description }}</p>
        </div>

        <!-- Always allow -->
        <label class="mb-4 flex cursor-pointer items-center gap-2 text-sm text-(--text-secondary)">
          <input v-model="alwaysAllow" type="checkbox" class="accent-(--accent)" />
          Always allow <span class="font-mono">{{ request.tool }}</span> this session
        </label>

        <!-- Actions -->
        <div class="flex justify-end gap-3">
          <button
            class="rounded-lg border border-(--border) px-4 py-2 text-sm text-(--text-secondary) transition hover:border-(--error) hover:text-(--error)"
            @click="deny"
          >
            Deny
          </button>
          <button
            class="rounded-lg bg-(--accent) px-4 py-2 text-sm font-medium text-white transition hover:bg-(--accent-hover)"
            @click="approve"
          >
            Allow Once
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
