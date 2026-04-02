<script setup lang="ts">
import StreamingText from './StreamingText.vue'

defineProps<{
  role: 'user' | 'assistant'
  content: string
  isStreaming: boolean
}>()
</script>

<template>
  <div
    :class="[
      'rounded-xl px-4 py-3 text-sm leading-relaxed',
      role === 'user'
        ? 'ml-auto max-w-[75%] bg-(--user-bubble) text-white'
        : 'mr-auto max-w-[85%] bg-(--assistant-bubble) text-(--text-primary)',
    ]"
  >
    <!-- User messages: plain text -->
    <template v-if="role === 'user'">
      <p class="whitespace-pre-wrap">{{ content }}</p>
    </template>

    <!-- Assistant messages -->
    <template v-else>
      <StreamingText v-if="isStreaming" :text="content" />
      <p v-else class="whitespace-pre-wrap">{{ content }}</p>
    </template>
  </div>
</template>
