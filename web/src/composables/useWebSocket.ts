import { ref, onUnmounted } from 'vue'
import type { ClientCommand, ServerEvent } from '@/types/events'

type EventHandler = (event: ServerEvent) => void

export function useWebSocket() {
  const status = ref<'connected' | 'connecting' | 'disconnected'>('disconnected')
  let ws: WebSocket | null = null
  let reconnectTimer: ReturnType<typeof setTimeout> | null = null
  let reconnectDelay = 1000
  const maxReconnectDelay = 30000
  const handlers: EventHandler[] = []

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
    }

    ws.onmessage = (e) => {
      try {
        const event: ServerEvent = JSON.parse(e.data)
        for (const handler of handlers) {
          handler(event)
        }
      } catch {
        // Ignore malformed messages
      }
    }

    ws.onclose = () => {
      status.value = 'disconnected'
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
