<script setup lang="ts">
import { defineAsyncComponent } from 'vue'

// Self-referencing component loaded lazily to allow recursion
const TreeItem = defineAsyncComponent(() => import('./TreeItem.vue'))

export interface FileNode {
  name: string
  path: string
  kind: 'file' | 'dir'
  size?: number
  lang?: string
  children?: FileNode[]
  expanded?: boolean
  loading?: boolean
}

const props = defineProps<{
  node: FileNode
  depth?: number
}>()

const emit = defineEmits<{
  toggle: [node: FileNode]
  open: [node: FileNode]
}>()

const indent = `${(props.depth ?? 0) * 12 + 8}px`

const icon = props.node.kind === 'dir'
  ? props.node.expanded ? '▾' : '▸'
  : '·'
</script>

<template>
  <div>
    <button
      class="flex w-full items-center gap-1.5 py-0.5 text-xs transition hover:bg-(--bg-primary) hover:text-(--text-primary)"
      :class="node.kind === 'dir' ? 'font-medium text-(--text-secondary)' : 'text-(--text-muted)'"
      :style="{ paddingLeft: indent }"
      @click="node.kind === 'dir' ? emit('toggle', node) : emit('open', node)"
    >
      <span class="w-3 text-center">{{ node.loading ? '…' : icon }}</span>
      <span class="truncate">{{ node.name }}</span>
    </button>

    <template v-if="node.kind === 'dir' && node.expanded && node.children">
      <TreeItem
        v-for="child in node.children"
        :key="child.path"
        :node="child"
        :depth="(depth ?? 0) + 1"
        @toggle="emit('toggle', $event)"
        @open="emit('open', $event)"
      />
    </template>
  </div>
</template>
