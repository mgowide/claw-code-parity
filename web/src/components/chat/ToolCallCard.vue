<script setup lang="ts">
import { ref, computed } from 'vue'
import type { ToolCall } from '@/types/events'
import DiffViewer from './DiffViewer.vue'
import CodeBlock from './CodeBlock.vue'
import TerminalPanel from './TerminalPanel.vue'

const props = defineProps<{
  toolCall: ToolCall
}>()

const expanded = ref(
  // Auto-expand for edit/write (they show diffs)
  props.toolCall.name === 'edit_file' || props.toolCall.name === 'write_file',
)

const borderColor = computed(() => {
  switch (props.toolCall.status) {
    case 'running':
      return 'border-(--accent)'
    case 'success':
      return 'border-(--success)'
    case 'error':
      return 'border-(--error)'
  }
})

const statusIcon = computed(() => {
  switch (props.toolCall.status) {
    case 'running':
      return '⏳'
    case 'success':
      return '✅'
    case 'error':
      return '❌'
  }
})

const duration = computed(() => {
  if (!props.toolCall.endTime) return null
  const ms = props.toolCall.endTime - props.toolCall.startTime
  return ms < 1000 ? `${ms}ms` : `${(ms / 1000).toFixed(1)}s`
})

const isBashTool = computed(() => ['bash', 'execute_bash'].includes(props.toolCall.name))
const isFileTool = computed(() => ['read_file', 'grep_search', 'glob_search'].includes(props.toolCall.name))

const inputSummary = computed(() => {
  const input = props.toolCall.input
  if (input.path) return String(input.path)
  if (input.command) return String(input.command)
  if (input.pattern) return String(input.pattern)
  return JSON.stringify(input).slice(0, 80)
})
</script>

<template>
  <div
    :class="['my-2 overflow-hidden rounded-lg border-l-2 bg-(--bg-secondary)', borderColor]"
  >
    <!-- Header — always visible -->
    <button
      class="flex w-full items-center justify-between px-3 py-2 text-left text-xs transition hover:bg-(--bg-tertiary)"
      @click="expanded = !expanded"
    >
      <span class="flex items-center gap-2">
        <span>{{ statusIcon }}</span>
        <span class="font-mono font-medium text-(--text-primary)">{{ toolCall.name }}</span>
        <span class="truncate text-(--text-muted)" style="max-inline-size: 300px">{{ inputSummary }}</span>
      </span>
      <span class="flex items-center gap-2 text-(--text-muted)">
        <span v-if="duration">{{ duration }}</span>
        <span>{{ expanded ? '▲' : '▼' }}</span>
      </span>
    </button>

    <!-- Body — collapsible with expand animation -->
    <Transition name="expand">
    <div v-if="expanded" class="border-t border-(--border) p-3">
      <!-- Input params -->
      <details class="mb-2">
        <summary class="cursor-pointer text-xs text-(--text-muted)">Input</summary>
        <pre class="mt-1 overflow-auto rounded bg-(--bg-primary) p-2 text-xs text-(--text-secondary)">{{ JSON.stringify(toolCall.input, null, 2) }}</pre>
      </details>

      <!-- Diff viewer for edit/write -->
      <DiffViewer
        v-if="toolCall.diff"
        :file-path="toolCall.diff.path"
        :old-content="toolCall.diff.oldContent"
        :new-content="toolCall.diff.newContent"
        class="mb-2"
      />

      <!-- Terminal for bash output -->
      <TerminalPanel
        v-if="isBashTool && toolCall.output"
        :output="toolCall.output"
        class="mb-2"
      />

      <!-- Code block for file reads -->
      <CodeBlock
        v-if="isFileTool && toolCall.output && !toolCall.isError"
        :code="toolCall.output"
        :file-name="toolCall.input.path ? String(toolCall.input.path) : undefined"
        class="mb-2"
      />

      <!-- Generic output fallback -->
      <div
        v-if="toolCall.output && !isBashTool && !isFileTool && !toolCall.diff"
        class="overflow-auto rounded bg-(--bg-primary) p-2 text-xs"
        :class="toolCall.isError ? 'text-(--error)' : 'text-(--text-secondary)'"
      >
        <pre class="whitespace-pre-wrap">{{ toolCall.output }}</pre>
      </div>

      <!-- Error output -->
      <div
        v-if="toolCall.isError && (isBashTool || isFileTool)"
        class="overflow-auto rounded bg-(--bg-primary) p-2 text-xs text-(--error)"
      >
        <pre class="whitespace-pre-wrap">{{ toolCall.output }}</pre>
      </div>
    </div>
    </Transition>
  </div>
</template>
