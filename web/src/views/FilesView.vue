<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getFiles, readFile } from '@/lib/api'
import type { FileEntry, FileReadResponse } from '@/lib/api'
import TreeItem from '@/components/files/TreeItem.vue'
import type { FileNode } from '@/components/files/TreeItem.vue'
import hljs from 'highlight.js'

const tree = ref<FileNode[]>([])
const fileContent = ref<FileReadResponse | null>(null)
const viewerError = ref('')
const treeLoading = ref(false)
const treeError = ref('')

const highlighted = computed(() => {
  if (!fileContent.value) return ''
  try {
    const lang = fileContent.value.lang
    if (lang && hljs.getLanguage(lang)) {
      return hljs.highlight(fileContent.value.content, { language: lang }).value
    }
    return hljs.highlightAuto(fileContent.value.content).value
  } catch {
    return fileContent.value.content
  }
})

function toNodes(entries: FileEntry[]): FileNode[] {
  return entries.map((e) => ({ ...e, expanded: false, loading: false }))
}

async function loadDir(path = '.'): Promise<FileNode[]> {
  try {
    const { entries } = await getFiles(path)
    return toNodes(entries)
  } catch {
    return []
  }
}

async function handleToggle(node: FileNode) {
  if (node.kind !== 'dir') return
  if (!node.expanded) {
    node.loading = true
    node.children = await loadDir(node.path)
    node.loading = false
  }
  node.expanded = !node.expanded
}

async function handleOpen(node: FileNode) {
  if (node.kind !== 'file') return
  viewerError.value = ''
  fileContent.value = null
  try {
    fileContent.value = await readFile(node.path)
  } catch (e) {
    viewerError.value = e instanceof Error ? e.message : 'Failed to read file'
  }
}

async function init() {
  treeLoading.value = true
  treeError.value = ''
  try {
    tree.value = await loadDir('.')
  } catch (e) {
    treeError.value = e instanceof Error ? e.message : 'Failed to load'
  } finally {
    treeLoading.value = false
  }
}

onMounted(init)
</script>

<template>
  <div class="flex h-full min-h-0">
    <!-- File tree -->
    <div class="flex w-60 flex-shrink-0 flex-col border-r border-(--border) bg-(--bg-secondary)">
      <div class="border-b border-(--border) px-4 py-3">
        <h1 class="text-sm font-semibold text-(--text-primary)">Files</h1>
        <p class="text-[11px] text-(--text-muted)">Workspace browser</p>
      </div>

      <div class="flex-1 overflow-y-auto py-1">
        <div v-if="treeLoading" class="px-4 py-4 text-xs text-(--text-muted)">Loading...</div>
        <div v-else-if="treeError" class="px-4 py-2 text-xs text-red-400">{{ treeError }}</div>
        <template v-else>
          <TreeItem
            v-for="node in tree"
            :key="node.path"
            :node="node"
            :depth="0"
            @toggle="handleToggle"
            @open="handleOpen"
          />
        </template>
      </div>
    </div>

    <!-- Code viewer -->
    <div class="flex min-w-0 flex-1 flex-col bg-(--bg-primary)">
      <div
        v-if="!fileContent && !viewerError"
        class="flex flex-1 items-center justify-center text-sm text-(--text-muted)"
      >
        Select a file to view
      </div>
      <div v-else-if="viewerError" class="p-4 text-sm text-red-400">{{ viewerError }}</div>
      <template v-else-if="fileContent">
        <div class="flex items-center justify-between border-b border-(--border) px-4 py-2">
          <span class="font-mono text-xs text-(--text-secondary)">{{ fileContent.path }}</span>
          <span class="text-[10px] text-(--text-muted)">
            {{ fileContent.total_lines }} lines
            <span class="rounded bg-(--bg-secondary) px-1 py-0.5 font-mono">{{ fileContent.lang }}</span>
          </span>
        </div>
        <div class="flex flex-1 overflow-auto">
          <div class="select-none border-r border-(--border) bg-(--bg-secondary) px-2 py-4 text-right">
            <div
              v-for="n in fileContent.end_line - fileContent.start_line + 1"
              :key="n"
              class="font-mono text-[11px] leading-5 text-(--text-muted)"
            >
              {{ fileContent.start_line + n - 1 }}
            </div>
          </div>
          <pre
            class="flex-1 overflow-auto p-4 font-mono text-[12px] leading-5 text-(--text-primary)"
            v-html="highlighted"
          />
        </div>
      </template>
    </div>
  </div>
</template>
