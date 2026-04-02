import { defineStore } from 'pinia'
import { ref, reactive } from 'vue'
import type { Message, ToolCall, PermissionRequest, SessionSummary } from '@/types/events'
import { getSessions, deleteSession as apiDeleteSession } from '@/lib/api'

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
  const toolCalls = ref<ToolCall[]>([])
  const pendingPermissions = ref<PermissionRequest[]>([])
  const alwaysAllowedTools = reactive(new Set<string>())

  // ── Session list (Phase 3) ──────────────────────────────────────────────────
  const sessions = ref<SessionSummary[]>([])
  const sessionsLoading = ref(false)
  const sessionsError = ref<string | null>(null)

  async function loadSessions(): Promise<void> {
    sessionsLoading.value = true
    sessionsError.value = null
    try {
      sessions.value = await getSessions()
    } catch (err) {
      sessionsError.value = err instanceof Error ? err.message : 'Failed to load sessions'
    } finally {
      sessionsLoading.value = false
    }
  }

  async function removeSession(id: string): Promise<void> {
    await apiDeleteSession(id)
    sessions.value = sessions.value.filter((s) => s.id !== id)
    // If the active session was deleted, reset the chat view
    if (sessionId.value === id) {
      reset()
    }
  }

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

  function addToolCall(tc: Omit<ToolCall, 'output' | 'isError' | 'endTime' | 'diff'>) {
    toolCalls.value.push({ ...tc })
  }

  function resolveToolCall(id: string, result: { output: string; isError: boolean; status: 'success' | 'error'; endTime: number }) {
    const tc = toolCalls.value.find((t) => t.id === id)
    if (tc) {
      tc.output = result.output
      tc.isError = result.isError
      tc.status = result.status
      tc.endTime = result.endTime
    }
  }

  function attachDiff(toolId: string, diff: { path: string; oldContent: string; newContent: string }) {
    // Attach diff to the most recent tool call matching the path, or the last running one
    const tc = toolCalls.value.find((t) => t.id === toolId) ?? toolCalls.value.findLast((t) => t.status === 'running')
    if (tc) {
      tc.diff = diff
    }
  }

  function addPermissionRequest(req: PermissionRequest) {
    // If the tool is already always-allowed, don't show the modal
    if (alwaysAllowedTools.has(req.tool)) return
    pendingPermissions.value.push(req)
  }

  function resolvePermission(requestId: string) {
    pendingPermissions.value = pendingPermissions.value.filter((p) => p.id !== requestId)
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
    toolCalls.value = []
    pendingPermissions.value = []
    alwaysAllowedTools.clear()
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
    toolCalls,
    pendingPermissions,
    alwaysAllowedTools,
    // Phase 3
    sessions,
    sessionsLoading,
    sessionsError,
    addUserMessage,
    appendDelta,
    completeMessage,
    updateUsage,
    addToolCall,
    resolveToolCall,
    attachDiff,
    addPermissionRequest,
    resolvePermission,
    loadSessions,
    removeSession,
    reset,
  }
})
