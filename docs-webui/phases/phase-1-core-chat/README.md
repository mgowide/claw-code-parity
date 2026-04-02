# Phase 1 — Core Chat That Works

> **Goal**: Send a message in the browser, see the AI response stream token-by-token. One session, end-to-end.

---

## What This Phase Delivers

A browser page at `localhost:5173` (dev) or `localhost:3100` (prod) with:
- A chat interface with message bubbles
- Multi-line input with send button
- Streaming text that appears token-by-token
- Status bar showing model name and WebSocket connection state
- A real `ConversationRuntime` underneath — not a mock

## Architecture

```
Browser (Vue 3)                    Axum Server (Rust)
┌─────────────┐                   ┌─────────────────────┐
│ ChatView    │  ──WebSocket──►   │ ws_handler()        │
│ InputComposer│                  │   ↓                  │
│ StreamingText│  ◄──WS events──  │ ConversationRuntime  │
│ StatusBar   │                   │   ↓                  │
│ sessionStore│                   │ api::stream_response│
└─────────────┘                   └─────────────────────┘
```

## Steps

| # | Task | Type | Detail Doc |
|---|---|---|---|
| 1.1 | Create `web-server` Rust crate | Backend | [backend.md](backend.md) |
| 1.2 | Axum server + `/api/status` health route | Backend | [backend.md](backend.md) |
| 1.3 | WebSocket `/ws` endpoint + event/command types | Backend | [backend.md](backend.md) |
| 1.4 | Session create + message handler | Backend | [backend.md](backend.md) |
| 1.5 | Scaffold Vue 3 + Vite project | Frontend | [frontend.md](frontend.md) |
| 1.6 | Install frontend dependencies | Frontend | [frontend.md](frontend.md) |
| 1.7 | `AppShell.vue` + `StatusBar.vue` | Frontend | [frontend.md](frontend.md) |
| 1.8 | `ChatView.vue` + `ChatThread.vue` | Frontend | [frontend.md](frontend.md) |
| 1.9 | `MessageBubble.vue` + `StreamingText.vue` | Frontend | [frontend.md](frontend.md) |
| 1.10 | `InputComposer.vue` | Frontend | [frontend.md](frontend.md) |
| 1.11 | `useWebSocket.ts` composable | Frontend | [frontend.md](frontend.md) |
| 1.12 | `useStream.ts` + `sessionStore.ts` | Frontend | [frontend.md](frontend.md) |
| 1.13 | Wire frontend ↔ backend end-to-end | Integration | both |
| 1.14 | CORS + static file serving | Backend | [backend.md](backend.md) |
| 1.15 | Test full chat flow | Validation | both |

## Acceptance Criteria

- [ ] `cargo build -p web-server` compiles with no warnings
- [ ] `cargo run -p web-server` starts HTTP server on port 3100
- [ ] `GET /api/status` returns `{ "status": "ok", "model": "..." }`
- [ ] WebSocket connects at `ws://localhost:3100/ws`
- [ ] Sending `send_message` command triggers streaming `text_delta` events
- [ ] Sending `cancel_turn` stops the stream
- [ ] Vue app renders chat with streaming text in browser
- [ ] Status bar shows model name and connection status (green dot)
- [ ] `cargo clippy -p web-server` passes with no warnings
- [ ] `npm run build` produces production bundle served by Axum

## Files Created

### Backend (`rust/crates/web-server/`)
```
Cargo.toml
src/
├── main.rs          # Server entry, router setup
├── state.rs         # AppState (shared runtime state)
├── routes/
│   ├── mod.rs
│   ├── health.rs    # GET /api/status
│   └── session.rs   # POST /api/sessions
├── ws/
│   ├── mod.rs
│   ├── handler.rs   # WebSocket upgrade + message loop
│   ├── events.rs    # ServerEvent enum (text_delta, usage, etc.)
│   └── commands.rs  # ClientCommand enum (send_message, cancel_turn)
└── error.rs         # Error types
```

### Frontend (`web/`)
```
package.json
vite.config.ts
tsconfig.json
tailwind.config.ts
index.html
src/
├── main.ts
├── App.vue
├── router/index.ts
├── views/ChatView.vue
├── components/
│   ├── layout/AppShell.vue
│   ├── layout/StatusBar.vue
│   ├── chat/ChatThread.vue
│   ├── chat/MessageBubble.vue
│   ├── chat/StreamingText.vue
│   └── composer/InputComposer.vue
├── composables/
│   ├── useWebSocket.ts
│   └── useStream.ts
├── stores/sessionStore.ts
├── types/events.ts
└── lib/ws.ts
```

---

*Detail docs: [backend.md](backend.md) | [frontend.md](frontend.md)*
