<script setup lang="ts">
import { ref, computed, watch } from 'vue'

export interface SlashCommand {
  name: string
  description: string
  group: string
  hasArg?: boolean
}

const COMMANDS: SlashCommand[] = [
  // Session
  { name: '/clear', description: 'Clear conversation display', group: 'Session' },
  { name: '/compact', description: 'Compact conversation to reduce tokens', group: 'Session' },
  { name: '/cost', description: 'Show token usage and cost', group: 'Session' },
  // Model
  { name: '/model', description: 'Switch AI model', group: 'Model', hasArg: true },
  // Tools
  { name: '/allowed-tools', description: 'Show currently allowed tools', group: 'Tools' },
  { name: '/permissions', description: 'Show permission settings', group: 'Tools' },
  // Files
  { name: '/add-dir', description: 'Add directory to context', group: 'Files', hasArg: true },
  // Debug
  { name: '/doctor', description: 'Check system health', group: 'Debug' },
  { name: '/config', description: 'Show current configuration', group: 'Debug' },
  { name: '/status', description: 'Show session status', group: 'Debug' },
  { name: '/help', description: 'Show all available commands', group: 'Debug' },
  { name: '/version', description: 'Show version information', group: 'Debug' },
]

interface Props {
  /** Text after the '/' — used for fuzzy filtering */
  query: string
}

const props = defineProps<Props>()
const emit = defineEmits<{
  select: [name: string]
  close: []
}>()

const activeIndex = ref(0)

function score(cmd: SlashCommand, q: string): number {
  if (!q) return 1
  const name = cmd.name.slice(1).toLowerCase()
  const desc = cmd.description.toLowerCase()
  const lower = q.toLowerCase()
  if (name === lower) return 100
  if (name.startsWith(lower)) return 80
  if (name.includes(lower)) return 60
  if (desc.includes(lower)) return 40
  return 0
}

const filtered = computed(() => {
  const q = props.query.trim()
  return COMMANDS.filter((c) => score(c, q) > 0).sort((a, b) => score(b, props.query) - score(a, props.query))
})

const groupedResults = computed(() => {
  const groups: Record<string, SlashCommand[]> = {}
  for (const cmd of filtered.value) {
    if (!groups[cmd.group]) groups[cmd.group] = []
    groups[cmd.group].push(cmd)
  }
  return groups
})

// Flat list for keyboard navigation
const flatList = computed(() => filtered.value)

watch(
  () => props.query,
  () => { activeIndex.value = 0 },
)

function moveDown() {
  activeIndex.value = (activeIndex.value + 1) % flatList.value.length
}
function moveUp() {
  activeIndex.value = (activeIndex.value - 1 + flatList.value.length) % flatList.value.length
}
function confirmActive() {
  const cmd = flatList.value[activeIndex.value]
  if (cmd) emit('select', cmd.name)
}

defineExpose({ moveDown, moveUp, confirmActive })
</script>

<template>
  <div
    v-if="filtered.length > 0"
    class="absolute bottom-full left-0 mb-1 w-72 rounded-xl border border-(--border) bg-(--bg-secondary) py-1 shadow-2xl"
    role="listbox"
  >
    <template v-for="(cmds, group) in groupedResults" :key="group">
      <div class="px-3 pt-2 pb-0.5 text-[10px] font-semibold uppercase tracking-wider text-(--text-muted)">
        {{ group }}
      </div>
      <button
        v-for="cmd in cmds"
        :key="cmd.name"
        role="option"
        :aria-selected="flatList.indexOf(cmd) === activeIndex"
        class="flex w-full items-center gap-3 px-3 py-1.5 text-left transition"
        :class="[
          flatList.indexOf(cmd) === activeIndex
            ? 'bg-(--accent)/15 text-(--accent)'
            : 'text-(--text-secondary) hover:bg-(--bg-primary)',
        ]"
        @mouseenter="activeIndex = flatList.indexOf(cmd)"
        @click="emit('select', cmd.name)"
      >
        <span class="w-32 shrink-0 font-mono text-xs">{{ cmd.name }}</span>
        <span class="truncate text-xs text-(--text-muted)">{{ cmd.description }}</span>
      </button>
    </template>
  </div>
</template>
