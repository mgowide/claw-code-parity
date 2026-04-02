<script setup lang="ts">
import { ref, inject, watch, nextTick } from 'vue'
import { useSessionStore } from '@/stores/sessionStore'
import { useUiStore } from '@/stores/uiStore'
import SlashCommandMenu from './SlashCommandMenu.vue'
import FileAttachBar from './FileAttachBar.vue'
import type { AttachedFile } from './FileAttachBar.vue'
import type { useWebSocket } from '@/composables/useWebSocket'

const store = useSessionStore()
const uiStore = useUiStore()
const ws = inject<ReturnType<typeof useWebSocket>>('ws')!

const inputText = ref('')
const textareaRef = ref<HTMLTextAreaElement | null>(null)
const menuRef = ref<InstanceType<typeof SlashCommandMenu> | null>(null)
const isDragOver = ref(false)
const attachedFiles = ref<AttachedFile[]>([])

// ── Slash command state ───────────────────────────────────────────────────────
const slashQuery = ref<string | null>(null) // null = menu closed

const slashVisible = computed(() => slashQuery.value !== null)

function updateSlashState(text: string) {
  if (text.startsWith('/')) {
    slashQuery.value = text.slice(1)
  } else {
    slashQuery.value = null
  }
}

function onSlashSelect(name: string) {
  inputText.value = name + ' '
  slashQuery.value = null
  nextTick(() => textareaRef.value?.focus())
}

// ── Send ──────────────────────────────────────────────────────────────────────
function send() {
  const text = inputText.value.trim()
  if (!text || store.isStreaming) return

  store.addUserMessage(text)
  ws.send({
    type: 'send_message',
    session_id: store.sessionId ?? '',
    text,
    attachments: attachedFiles.value.map((f) => ({ name: f.name, content: f.content })),
  })
  inputText.value = ''
  attachedFiles.value = []
  slashQuery.value = null

  if (textareaRef.value) {
    textareaRef.value.style.height = 'auto'
  }
}

// ── Input handlers ────────────────────────────────────────────────────────────
function onKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
    e.preventDefault()
    send()
    return
  }
  if (slashVisible.value && menuRef.value) {
    if (e.key === 'ArrowDown') { e.preventDefault(); menuRef.value.moveDown(); return }
    if (e.key === 'ArrowUp')   { e.preventDefault(); menuRef.value.moveUp();   return }
    if (e.key === 'Enter')     { e.preventDefault(); menuRef.value.confirmActive(); return }
    if (e.key === 'Escape')    { slashQuery.value = null; return }
  }
}

function onInput(e: Event) {
  const el = e.target as HTMLTextAreaElement
  el.style.height = 'auto'
  el.style.height = `${Math.min(el.scrollHeight, 200)}px`
  updateSlashState(inputText.value)
}

// ── Drag-drop ─────────────────────────────────────────────────────────────────
function onDragOver(e: DragEvent) {
  e.preventDefault()
  isDragOver.value = true
}
function onDragLeave() {
  isDragOver.value = false
}
function onDrop(e: DragEvent) {
  e.preventDefault()
  isDragOver.value = false
  const files = Array.from(e.dataTransfer?.files ?? [])
  readFiles(files)
}

function readFiles(files: File[]) {
  for (const file of files) {
    const reader = new FileReader()
    reader.onload = (ev) => {
      const content = ev.target?.result as string
      // Avoid duplicates
      if (!attachedFiles.value.find((f) => f.name === file.name)) {
        attachedFiles.value.push({ name: file.name, content, size: file.size })
      }
    }
    reader.readAsText(file)
  }
}

// ── Focus trigger (Ctrl+/) ────────────────────────────────────────────────────
watch(
  () => uiStore.composerFocusTrigger,
  () => { nextTick(() => textareaRef.value?.focus()) },
)

import { computed } from 'vue'
</script>

<template>
  <div
    class="border-t border-(--border) bg-(--bg-secondary)"
    @dragover="onDragOver"
    @dragleave="onDragLeave"
    @drop="onDrop"
  >
    <!-- Drop zone overlay -->
    <div
      v-if="isDragOver"
      class="pointer-events-none absolute inset-0 z-10 flex items-center justify-center rounded-lg border-2 border-dashed border-(--accent) bg-(--accent)/5"
    >
      <span class="text-sm font-medium text-(--accent)">Drop files to attach</span>
    </div>

    <!-- Attached files bar -->
    <FileAttachBar
      :files="attachedFiles"
      @remove="(i) => attachedFiles.splice(i, 1)"
    />

    <!-- Composer row -->
    <div class="relative px-4 py-3">
      <!-- Slash command menu -->
      <SlashCommandMenu
        v-if="slashVisible"
        ref="menuRef"
        :query="slashQuery ?? ''"
        @select="onSlashSelect"
        @close="slashQuery = null"
      />

      <div class="mx-auto flex max-w-3xl items-end gap-2">
        <textarea
          ref="textareaRef"
          v-model="inputText"
          rows="1"
          placeholder="Message Claw… (/ for commands, Ctrl+Enter to send)"
          :disabled="store.isStreaming"
          class="min-h-10 flex-1 resize-none rounded-lg border border-(--border) bg-(--bg-primary) px-3 py-2 text-sm text-(--text-primary) placeholder-(--text-muted) outline-none transition focus:border-(--accent) disabled:opacity-50"
          @keydown="onKeydown"
          @input="onInput"
        />
        <button
          :disabled="!inputText.trim() || store.isStreaming"
          class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-(--accent) text-white transition hover:bg-(--accent-hover) disabled:opacity-30 disabled:hover:bg-(--accent)"
          @click="send"
        >
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <path d="M3 13L13 8L3 3V7L9 8L3 9V13Z" fill="currentColor" />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

