import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Message } from '@/types/events'

export const useSessionStore = defineStore('session', () => {
  const sessionId = ref<string | null>(null)
  const model = ref('claude-sonnet-4-20250514')
  const messages = ref<Message[]>([])
  const isStreaming = ref(false)
  const isThinking = ref(false)
  const inputTokens = ref(0)
  const outputTokens = ref(0)
  const cacheHits = ref(0)
  const cost = ref(0)
  const connectionStatus = ref<'connected' | 'connecting' | 'disconnected'>('disconnected')

  function addUserMessage(text: string) {
    messages.value.push({
      id: crypto.randomUUID(),
      role: 'user',
      content: text,
      isStreaming: false,
      timestamp: Date.now(),
    })
    // Create a placeholder assistant message for streaming into
    messages.value.push({
      id: crypto.randomUUID(),
      role: 'assistant',
      content: '',
      isStreaming: true,
      timestamp: Date.now(),
    })
    isStreaming.value = true
  }

  function appendDelta(text: string) {
    const last = messages.value.at(-1)
    if (last && last.role === 'assistant' && last.isStreaming) {
      last.content += text
    }
  }

  function completeMessage() {
    const last = messages.value.at(-1)
    if (last && last.role === 'assistant') {
      last.isStreaming = false
      // Remove empty assistant messages (e.g. cancelled turns)
      if (!last.content) {
        messages.value.pop()
      }
    }
    isStreaming.value = false
    isThinking.value = false
  }

  function updateUsage(tokens: { input_tokens: number; output_tokens: number; cache_hits: number; cost: number }) {
    inputTokens.value += tokens.input_tokens
    outputTokens.value += tokens.output_tokens
    cacheHits.value += tokens.cache_hits
    cost.value += tokens.cost
  }

  function reset() {
    sessionId.value = null
    messages.value = []
    isStreaming.value = false
    isThinking.value = false
    inputTokens.value = 0
    outputTokens.value = 0
    cacheHits.value = 0
    cost.value = 0
  }

  return {
    sessionId,
    model,
    messages,
    isStreaming,
    isThinking,
    inputTokens,
    outputTokens,
    cacheHits,
    cost,
    connectionStatus,
    addUserMessage,
    appendDelta,
    completeMessage,
    updateUsage,
    reset,
  }
})
