<script setup lang="ts">
import { ref, nextTick, watch } from 'vue'
import { useSessionStore } from '@/stores/sessionStore'
import MessageBubble from './MessageBubble.vue'

const store = useSessionStore()
const threadRef = ref<HTMLElement | null>(null)

// Auto-scroll to bottom when messages change
watch(
  () => store.messages.length + (store.messages.at(-1)?.content.length ?? 0),
  async () => {
    await nextTick()
    if (threadRef.value) {
      threadRef.value.scrollTop = threadRef.value.scrollHeight
    }
  },
)
</script>

<template>
  <div ref="threadRef" class="flex-1 overflow-y-auto px-4 py-6">
    <!-- Empty state -->
    <div
      v-if="store.messages.length === 0"
      class="flex h-full flex-col items-center justify-center gap-4 text-center"
    >
      <div class="text-4xl">🦀</div>
      <h2 class="text-xl font-medium text-(--text-primary)">Welcome to Claw</h2>
      <p class="max-w-md text-sm text-(--text-secondary)">
        Start a conversation by typing a message below. I can help you with code, answer questions,
        and use tools to work with files.
      </p>
    </div>

    <!-- Messages and Tool Calls -->
    <div class="mx-auto flex max-w-3xl flex-col gap-4">
      <MessageBubble
        v-for="msg in store.messages"
        :key="msg.id"
        :role="msg.role"
        :content="msg.content"
        :is-streaming="msg.isStreaming"
      />
    </div>
  </div>
</template>
