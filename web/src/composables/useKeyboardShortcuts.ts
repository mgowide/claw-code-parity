import { inject } from 'vue'
import { useRouter } from 'vue-router'
import { onKeyStroke } from '@vueuse/core'
import { useSessionStore } from '@/stores/sessionStore'
import { useUiStore } from '@/stores/uiStore'
import type { useWebSocket } from '@/composables/useWebSocket'

/**
 * Registers global keyboard shortcuts.
 * Must be called from a component that has access to the provide/inject context
 * (e.g. App.vue, which is a descendant of AppShell where `ws` is provided).
 *
 * Shortcut map:
 *   Ctrl+Enter   — send message (handled locally in InputComposer; here as fallback)
 *   Escape       — cancel streaming turn
 *   Ctrl+N       — new session
 *   Ctrl+K       — open model selector
 *   Ctrl+/       — focus InputComposer
 *   Ctrl+B       — toggle sidebar
 *   Ctrl+L       — clear chat display (no backend delete)
 */
export function useKeyboardShortcuts() {
  const store = useSessionStore()
  const uiStore = useUiStore()
  const router = useRouter()
  const ws = inject<ReturnType<typeof useWebSocket>>('ws')

  // Escape — cancel streaming turn
  onKeyStroke('Escape', () => {
    if (store.isStreaming && store.sessionId && ws) {
      ws.send({ type: 'cancel_turn', session_id: store.sessionId })
    }
  })

  // Ctrl+N — new session
  onKeyStroke('n', (e: KeyboardEvent) => {
    if (e.ctrlKey && !e.shiftKey && !e.altKey) {
      e.preventDefault()
      store.reset()
      router.push('/')
    }
  })

  // Ctrl+K — open model selector
  onKeyStroke('k', (e: KeyboardEvent) => {
    if (e.ctrlKey && !e.shiftKey && !e.altKey) {
      e.preventDefault()
      uiStore.modelSelectorOpen = !uiStore.modelSelectorOpen
    }
  })

  // Ctrl+/ — focus InputComposer
  onKeyStroke('/', (e: KeyboardEvent) => {
    if (e.ctrlKey) {
      e.preventDefault()
      uiStore.composerFocusTrigger++
    }
  })

  // Ctrl+B — toggle sidebar
  onKeyStroke('b', (e: KeyboardEvent) => {
    if (e.ctrlKey && !e.shiftKey && !e.altKey) {
      e.preventDefault()
      uiStore.sidebarOpen = !uiStore.sidebarOpen
    }
  })

  // Ctrl+L — clear chat display
  onKeyStroke('l', (e: KeyboardEvent) => {
    if (e.ctrlKey && !e.shiftKey && !e.altKey) {
      e.preventDefault()
      store.clearDisplay()
    }
  })
}
