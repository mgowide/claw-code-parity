<script setup lang="ts">
import { ref, inject } from 'vue'
import { useSessionStore } from '@/stores/sessionStore'
import type { useWebSocket } from '@/composables/useWebSocket'

const store = useSessionStore()
const ws = inject<ReturnType<typeof useWebSocket>>('ws')!

const inputText = ref('')
const textareaRef = ref<HTMLTextAreaElement | null>(null)

function send() {
  const text = inputText.value.trim()
  if (!text || store.isStreaming) return

  store.addUserMessage(text)
  ws.send({
    type: 'send_message',
    session_id: store.sessionId ?? '',
    text,
  })
  inputText.value = ''

  // Reset textarea height
  if (textareaRef.value) {
    textareaRef.value.style.height = 'auto'
  }
}

function onKeydown(e: KeyboardEvent) {
  // Ctrl+Enter or Cmd+Enter to send
  if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
    e.preventDefault()
    send()
  }
}

function autoResize(e: Event) {
  const el = e.target as HTMLTextAreaElement
  el.style.height = 'auto'
  el.style.height = `${Math.min(el.scrollHeight, 200)}px`
}
</script>

<template>
  <div class="border-t border-(--border) bg-(--bg-secondary) px-4 py-3">
    <div class="mx-auto flex max-w-3xl items-end gap-2">
      <textarea
        ref="textareaRef"
        v-model="inputText"
        rows="1"
        placeholder="Message Claw... (Ctrl+Enter to send)"
        :disabled="store.isStreaming"
        class="min-h-10 flex-1 resize-none rounded-lg border border-(--border) bg-(--bg-primary) px-3 py-2 text-sm text-(--text-primary) placeholder-(--text-muted) outline-none transition focus:border-(--accent) disabled:opacity-50"
        @keydown="onKeydown"
        @input="autoResize"
      />
      <button
        :disabled="!inputText.trim() || store.isStreaming"
        class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-(--accent) text-white transition hover:bg-(--accent-hover) disabled:opacity-30 disabled:hover:bg-(--accent)"
        @click="send"
      >
        <!-- Send arrow icon -->
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M3 13L13 8L3 3V7L9 8L3 9V13Z" fill="currentColor" />
        </svg>
      </button>
    </div>
  </div>
</template>
