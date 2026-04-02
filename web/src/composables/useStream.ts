import type { ServerEvent } from '@/types/events'
import { useSessionStore } from '@/stores/sessionStore'

/**
 * Processes incoming ServerEvent messages and mutates the session store accordingly.
 */
export function useStream() {
  const store = useSessionStore()

  function handleEvent(event: ServerEvent) {
    switch (event.type) {
      case 'text_delta':
        store.appendDelta(event.text)
        break
      case 'thinking_start':
        store.isThinking = true
        break
      case 'thinking_end':
        store.isThinking = false
        break
      case 'usage':
        store.updateUsage({
          input_tokens: event.input_tokens,
          output_tokens: event.output_tokens,
          cache_hits: event.cache_hits,
          cost: event.cost,
        })
        break
      case 'turn_complete':
        store.completeMessage()
        break
      case 'error':
        store.appendDelta(`\n\n**Error:** ${event.message}`)
        store.completeMessage()
        break
      case 'connected':
        store.sessionId = event.session_id
        store.model = event.model
        store.connectionStatus = 'connected'
        break
    }
  }

  return { handleEvent }
}
