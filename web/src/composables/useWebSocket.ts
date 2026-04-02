import { ref, onUnmounted } from 'vue'
import type { ClientCommand, ServerEvent } from '@/types/events'
import { useToastStore } from '@/stores/toastStore'

type EventHandler = (event: ServerEvent) => void

export function useWebSocket() {
  const status = ref<'connected' | 'connecting' | 'disconnected'>('disconnected')
  let ws: WebSocket | null = null
  let reconnectTimer: ReturnType<typeof setTimeout> | null = null
  let reconnectDelay = 1000
  const maxReconnectDelay = 30000
  const handlers: EventHandler[] = []
  let toast: ReturnType<typeof useToastStore> | null = null

  function getToast() {
    if (!toast) {
      try { toast = useToastStore() } catch { /* pinia not ready yet */ }
    }
    return toast
  }

  function connect() {
    if (ws && (ws.readyState === WebSocket.OPEN || ws.readyState === WebSocket.CONNECTING)) {
      return
    }

    status.value = 'connecting'
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
    const url = `${protocol}//${window.location.host}/ws`
    ws = new WebSocket(url)

    ws.onopen = () => {
      status.value = 'connected'
      reconnectDelay = 1000
      if (reconnectTimer) getToast()?.success('Reconnected to server')
    }

    ws.onmessage = (e) => {
      try {
        const event: ServerEvent = JSON.parse(e.data)
        // Surface error events as toasts
        if (event.type === 'error') getToast()?.error(event.message)
        if (event.type === 'session_compacted') getToast()?.info('Session context was compacted')
        for (const handler of handlers) {
          handler(event)
        }
      } catch {
        // Ignore malformed messages
      }
    }

    ws.onclose = () => {
      status.value = 'disconnected'
      getToast()?.warning('Disconnected — reconnecting…')
      scheduleReconnect()
    }

    ws.onerror = () => {
      ws?.close()
    }
  }

  function scheduleReconnect() {
    if (reconnectTimer) return
    reconnectTimer = setTimeout(() => {
      reconnectTimer = null
      reconnectDelay = Math.min(reconnectDelay * 2, maxReconnectDelay)
      connect()
    }, reconnectDelay)
  }

  function send(command: ClientCommand) {
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(command))
    }
  }

  function onEvent(handler: EventHandler) {
    handlers.push(handler)
  }

  function disconnect() {
    if (reconnectTimer) {
      clearTimeout(reconnectTimer)
      reconnectTimer = null
    }
    ws?.close()
    ws = null
    status.value = 'disconnected'
  }

  // Auto-connect on first use
  connect()

  onUnmounted(() => {
    disconnect()
  })

  return { status, send, onEvent, connect, disconnect }
}
