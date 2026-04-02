# Phase 1 — Frontend: Vue 3 Chat Shell

> Steps 1.5–1.12: Scaffold the Vue project, build chat components, wire WebSocket.

---

## Step 1.5 — Scaffold Vue 3 + Vite Project

Create `web/` at workspace root (next to `rust/`, `docs/`, `docs-webui/`):

```bash
npm create vite@latest web -- --template vue-ts
```

### Project Structure After Scaffold

```
web/
├── package.json
├── vite.config.ts
├── tsconfig.json
├── index.html
├── public/
└── src/
    ├── main.ts
    ├── App.vue
    └── ...
```

### `vite.config.ts`

```typescript
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  server: {
    port: 5173,
    proxy: {
      '/api': 'http://localhost:3100',
      '/ws': { target: 'ws://localhost:3100', ws: true }
    }
  }
})
```

The proxy config means the Vue dev server forwards `/api/*` and `/ws` to the Axum backend. No CORS issues in dev.

---

## Step 1.6 — Install Frontend Dependencies

```bash
cd web

# Core
npm install vue vue-router pinia

# UI
npm install naive-ui @vueuse/core

# Dev
npm install -D typescript vite @vitejs/plugin-vue vue-tsc
npm install -D tailwindcss @tailwindcss/vite autoprefixer
```

Phase 1 only needs these. Monaco, xterm, v-code-diff, Three.js etc. come later.

### Tailwind Setup (`src/main.css`)

```css
@import "tailwindcss";
```

---

## Step 1.7 — AppShell + StatusBar

### `src/components/layout/AppShell.vue`

```
┌─────────────────────────────────────────────────────┐
│  ┌──────────┐  ┌────────────────────────────────┐   │
│  │ Sidebar  │  │  <RouterView />                │   │
│  │ (stub)   │  │  (ChatView by default)         │   │
│  │          │  │                                 │   │
│  │          │  │                                 │   │
│  │          │  │                                 │   │
│  └──────────┘  └────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────┐   │
│  │ StatusBar: model │ tokens │ connection        │   │
│  └──────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

- Sidebar is a stub in Phase 1 (just the app name + "New Chat" button)
- Main area holds the router view
- StatusBar at the bottom

### `src/components/layout/StatusBar.vue`

Displays:
- **Model name** — from `sessionStore.model` (e.g., "claude-sonnet-4-20250514")
- **Connection indicator** — green dot (connected), yellow (reconnecting), red (disconnected)
- **Token count** — `input: 0 / output: 0` (updates on `usage` events)

---

## Step 1.8 — ChatView + ChatThread

### `src/views/ChatView.vue`

The default route (`/`). Layout:

```
┌──────────────────────────┐
│  ChatThread              │  ← scrollable message list
│  (takes most of screen)  │
│                          │
│                          │
│                          │
├──────────────────────────┤
│  InputComposer           │  ← fixed at bottom
└──────────────────────────┘
```

### `src/components/chat/ChatThread.vue`

- `v-for` over `sessionStore.messages`
- Each message → `<MessageBubble />`
- Auto-scrolls to bottom on new messages
- Uses `nextTick()` + `scrollIntoView({ behavior: 'smooth' })`
- Shows empty state when no messages (welcome message / prompt suggestions)

---

## Step 1.9 — MessageBubble + StreamingText

### `src/components/chat/MessageBubble.vue`

Props: `{ role: 'user' | 'assistant', content: string, isStreaming: boolean }`

- **User messages**: right-aligned, blue background, plain text
- **Assistant messages**: left-aligned, dark gray background
  - If `isStreaming` → renders `<StreamingText :text="content" />`
  - If complete → renders as markdown (basic for now, enhanced in Phase 2)

### `src/components/chat/StreamingText.vue`

Props: `{ text: string }`

- Renders text with a blinking cursor `▌` at the end
- The cursor uses CSS animation:
  ```css
  @keyframes blink { 0%, 50% { opacity: 1 } 51%, 100% { opacity: 0 } }
  ```
- Text updates reactively as `text_delta` events append to the store
- Cursor disappears when `turn_complete` fires

---

## Step 1.10 — InputComposer

### `src/components/composer/InputComposer.vue`

- `<textarea>` with auto-resize (grows with content, max 200px)
- **Send button** — active when textarea is non-empty and not currently streaming
- **Ctrl+Enter** sends (or Enter sends, Shift+Enter for newline — configurable)
- Disabled state while assistant is responding (visual: dimmed input, spinner on send button)
- Emits `send(text: string)` event to ChatView, which dispatches to WebSocket

---

## Step 1.11 — useWebSocket Composable

### `src/composables/useWebSocket.ts`

```typescript
// Returns:
// - status: Ref<'connected' | 'connecting' | 'disconnected'>
// - send(command: ClientCommand): void
// - onEvent(handler: (event: ServerEvent) => void): void

// Behavior:
// - Connects to ws://localhost:3100/ws (or relative /ws via proxy)
// - Auto-reconnect with exponential backoff (1s, 2s, 4s, 8s, max 30s)
// - Heartbeat ping every 30s
// - Parses incoming JSON as ServerEvent
// - Serializes outgoing ClientCommand as JSON
// - Exposes reactive connection status
```

Uses VueUse's `useWebSocket` under the hood with custom reconnect logic.

---

## Step 1.12 — useStream + sessionStore

### `src/composables/useStream.ts`

Processes `ServerEvent` variants into Pinia store mutations:

```typescript
// text_delta → append text to current assistant message
// thinking_start → set isThinking = true
// thinking_end → set isThinking = false
// usage → update token counts + cost
// turn_complete → mark message as complete, re-enable input
// error → show error in UI
// connected → store session_id and model
```

### `src/stores/sessionStore.ts`

```typescript
// State:
interface SessionState {
  sessionId: string | null
  model: string
  messages: Message[]
  isStreaming: boolean
  isThinking: boolean
  inputTokens: number
  outputTokens: number
  cacheHits: number
  cost: number
  connectionStatus: 'connected' | 'connecting' | 'disconnected'
}

// Message type:
interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  isStreaming: boolean
  timestamp: number
}

// Actions:
// - addUserMessage(text) → append user message, set isStreaming
// - appendDelta(text) → append to last assistant message
// - completeMessage() → set isStreaming = false
// - updateUsage(tokens) → update counters
// - reset() → clear for new session
```

---

## Step 1.13 — Wiring End-to-End

The data flow:

```
1. User types in InputComposer → clicks Send
2. ChatView calls sessionStore.addUserMessage(text)
3. ChatView calls ws.send({ type: 'send_message', session_id, text })
4. Axum ws_handler receives → calls ConversationRuntime.send_message()
5. Runtime streams AssistantEvents → handler converts to ServerEvents → sends on WS
6. useStream receives text_delta → calls sessionStore.appendDelta(text)
7. ChatThread re-renders → MessageBubble shows StreamingText with cursor
8. ServerEvent::TurnComplete arrives → sessionStore.completeMessage()
9. InputComposer re-enables
```

---

## Type Definitions

### `src/types/events.ts`

```typescript
// Discriminated union matching backend's ServerEvent
export type ServerEvent =
  | { type: 'text_delta'; text: string }
  | { type: 'thinking_start' }
  | { type: 'thinking_end' }
  | { type: 'tool_use_start'; id: string; name: string; input: unknown }
  | { type: 'tool_result'; id: string; output: string; is_error: boolean }
  | { type: 'diff'; path: string; old_content: string; new_content: string }
  | { type: 'usage'; input_tokens: number; output_tokens: number; cache_hits: number; cost: number }
  | { type: 'turn_complete'; turn_index: number }
  | { type: 'permission_request'; id: string; tool: string; description: string }
  | { type: 'error'; message: string }
  | { type: 'connected'; session_id: string; model: string }

export type ClientCommand =
  | { type: 'send_message'; session_id: string; text: string }
  | { type: 'cancel_turn'; session_id: string }
  | { type: 'approve_permission'; request_id: string }
  | { type: 'deny_permission'; request_id: string }
  | { type: 'compact'; session_id: string }
  | { type: 'switch_model'; model: string }
  | { type: 'resume_session'; session_id: string }
```

---

*Back: [README](README.md) | Backend: [backend.md](backend.md)*
