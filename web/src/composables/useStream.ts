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
      case 'tool_use_start':
        store.addToolCall({
          id: event.id,
          name: event.name,
          input: event.input as Record<string, unknown>,
          status: 'running',
          startTime: Date.now(),
        })
        break
      case 'tool_result':
        store.resolveToolCall(event.id, {
          output: event.output,
          isError: event.is_error,
          status: event.is_error ? 'error' : 'success',
          endTime: Date.now(),
        })
        break
      case 'diff':
        // Attach diff to the most recent running tool call
        {
          const running = store.toolCalls.findLast((t) => t.status === 'running')
          if (running) {
            store.attachDiff(running.id, {
              path: event.path,
              oldContent: event.old_content,
              newContent: event.new_content,
            })
          }
        }
        break
      case 'permission_request':
        store.addPermissionRequest({
          id: event.id,
          tool: event.tool,
          description: event.description,
        })
        break
      case 'session_compacted':
        // Informational — could show a toast later
        break
    }
  }

  return { handleEvent }
}
