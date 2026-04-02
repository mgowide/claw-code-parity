/**
 * Composable for session management — CRUD + resume from outside the main
 * WebSocket lifecycle.
 */

import { inject } from 'vue'
import { useRouter } from 'vue-router'
import { useSessionStore } from '@/stores/sessionStore'
import { exportSession as apiExport } from '@/lib/api'
import type { useWebSocket } from '@/composables/useWebSocket'

export function useSession() {
  const store = useSessionStore()
  const router = useRouter()
  // The WebSocket instance is provided by AppShell via provide('ws', ws)
  const ws = inject<ReturnType<typeof useWebSocket>>('ws')!

  /**
   * Load (or refresh) the session list from the REST API.
   */
  async function loadSessions(): Promise<void> {
    await store.loadSessions()
  }

  /**
   * Resume an existing session by its ID.
   * Navigates to the chat view and sends a `resume_session` command over WS.
   */
  async function resumeSession(id: string): Promise<void> {
    ws.send({ type: 'resume_session', session_id: id })
    await router.push('/')
  }

  /**
   * Delete a session by ID.
   * Removes it from the store; if it was the active session, the store resets.
   */
  async function deleteSession(id: string): Promise<void> {
    await store.removeSession(id)
  }

  /**
   * Trigger a browser download of the session export.
   * format: 'md' | 'json'
   */
  async function exportSession(id: string, format: 'md' | 'json' = 'md'): Promise<void> {
    await apiExport(id, format)
  }

  return {
    sessions: store.sessions,
    sessionsLoading: store.sessionsLoading,
    sessionsError: store.sessionsError,
    loadSessions,
    resumeSession,
    deleteSession,
    exportSession,
  }
}
