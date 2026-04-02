# Phase 1 — Backend: Axum Web Server

> Steps 1.1–1.4 and 1.14: Create the Rust crate, HTTP server, WebSocket handler, and session management.

---

## Step 1.1 — Create `web-server` Rust Crate

### Cargo.toml

```toml
[package]
name = "web-server"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
# Workspace crates
runtime = { path = "../runtime" }
api = { path = "../api" }
tools = { path = "../tools" }

# Web framework
axum = { version = "0.8", features = ["ws"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "fs", "compression-gzip"] }

# WebSocket
tokio-tungstenite = "0.26"

# Async runtime
tokio = { version = "1", features = ["full"] }

# Serialization  
serde = { version = "1", features = ["derive"] }
serde_json.workspace = true

# Utilities
uuid = { version = "1", features = ["v4"] }
tracing = "0.1"
tracing-subscriber = "0.3"

[lints]
workspace = true
```

### Workspace Registration

Add to `rust/Cargo.toml` members — the crate automatically picks up because it's in `crates/*`.

---

## Step 1.2 — Axum Server + Health Route

### `src/main.rs`

```rust
// Entry point: build router, bind to port, serve
// - Loads config (API key, model) from environment / .claude.json
// - Creates shared AppState
// - Mounts routes: /api/status, /api/sessions, /ws
// - Serves static files from web/dist/ in production
// - Starts on 0.0.0.0:3100
```

### `src/state.rs`

```rust
// AppState — shared across all handlers via Axum's State extractor
pub struct AppState {
    pub sessions: DashMap<String, SessionHandle>,  // active sessions
    pub config: RuntimeConfig,                      // loaded from .claude.json
    pub api_key: String,                            // from env ANTHROPIC_API_KEY
    pub model: String,                              // default model
}
```

### `src/routes/health.rs`

```rust
// GET /api/status
// Returns: { "status": "ok", "model": "claude-sonnet-4-20250514", "version": "0.1.0" }
```

---

## Step 1.3 — WebSocket Endpoint + Event Types

### `src/ws/handler.rs`

WebSocket upgrade at `/ws`:

```rust
// 1. Accept WebSocket upgrade
// 2. Split into sender (tx) + receiver (rx)  
// 3. Spawn two tasks:
//    a. Read task: rx → parse ClientCommand → process
//    b. Write task: broadcast_rx → serialize ServerEvent → tx
// 4. On disconnect: clean up session reference
```

### `src/ws/events.rs` — Server → Client

```rust
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum ServerEvent {
    TextDelta { text: String },
    ThinkingStart {},
    ThinkingEnd {},
    ToolUseStart { id: String, name: String, input: serde_json::Value },
    ToolResult { id: String, output: String, is_error: bool },
    Diff { path: String, old_content: String, new_content: String },
    Usage { input_tokens: u64, output_tokens: u64, cache_hits: u64, cost: f64 },
    TurnComplete { turn_index: u64 },
    PermissionRequest { id: String, tool: String, description: String },
    SessionCompacted { removed_messages: u64 },
    Error { message: String },
    Connected { session_id: String, model: String },
}
```

### `src/ws/commands.rs` — Client → Server

```rust
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientCommand {
    SendMessage { session_id: String, text: String },
    CancelTurn { session_id: String },
    ApprovePermission { request_id: String },
    DenyPermission { request_id: String },
    Compact { session_id: String },
    SwitchModel { model: String },
    ResumeSession { session_id: String },
}
```

### Wire Format

All messages are JSON. Example:

```json
// Server → Client
{"type": "text_delta", "text": "Hello"}
{"type": "tool_use_start", "id": "t1", "name": "read_file", "input": {"path": "src/main.rs"}}
{"type": "usage", "input_tokens": 1200, "output_tokens": 340, "cache_hits": 800, "cost": 0.0043}

// Client → Server
{"type": "send_message", "session_id": "abc", "text": "explain this code"}
{"type": "cancel_turn", "session_id": "abc"}
```

---

## Step 1.4 — Session Create + Message Handler

### `src/routes/session.rs`

```rust
// POST /api/sessions
// Body: { "model"?: string, "system_prompt"?: string }
// Returns: { "session_id": "uuid", "model": "...", "created_at": "..." }
//
// Creates a ConversationRuntime instance, stores in AppState.sessions
```

### Message Flow (inside WebSocket handler)

```
Client sends: {"type": "send_message", "session_id": "abc", "text": "hello"}
    ↓
ws_handler receives ClientCommand::SendMessage
    ↓
Look up session "abc" in AppState.sessions
    ↓
Call session.runtime.send_message(text) → returns Stream<AssistantEvent>
    ↓
For each AssistantEvent:
    TextDelta(s) → send ServerEvent::TextDelta { text: s }
    ToolUse { id, name, input } → send ServerEvent::ToolUseStart { ... }
    Usage(u) → send ServerEvent::Usage { ... }
    MessageStop → send ServerEvent::TurnComplete { ... }
    ↓
Stream ends → input re-enabled on client
```

### Cancel Flow

```
Client sends: {"type": "cancel_turn", "session_id": "abc"}
    ↓
session.cancel_token.cancel()
    ↓
Stream aborts → send ServerEvent::TurnComplete
```

---

## Step 1.14 — CORS + Static File Serving

### CORS (dev mode)

```rust
// Allow localhost:5173 (Vite dev server) to call localhost:3100 (Axum)
CorsLayer::new()
    .allow_origin("http://localhost:5173")
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(Any)
```

### Static Files (production)

```rust
// Serve web/dist/ as the SPA fallback
// GET / → web/dist/index.html
// GET /assets/* → web/dist/assets/*
// 404 fallback → index.html (for Vue Router history mode)
ServeDir::new("web/dist").fallback(ServeFile::new("web/dist/index.html"))
```

---

## New Dependencies Added

| Crate | Version | Purpose | Risk |
|---|---|---|---|
| axum | 0.8 | HTTP server + WebSocket | Very low — by Tokio team |
| tower | 0.5 | Middleware framework | Very low — by Tokio team |
| tower-http | 0.6 | CORS, compression, static files | Very low — by Tokio team |
| tokio-tungstenite | 0.26 | WebSocket protocol | Low — widely used |
| uuid | 1 | Session ID generation | Very low — 15M downloads/week |
| tracing | 0.1 | Structured logging | Very low — by Tokio team |
| tracing-subscriber | 0.3 | Log formatting | Very low — by Tokio team |

All crates are maintained by the Tokio team or top-tier ecosystem authors. Zero risk additions.

---

*Next: [frontend.md](frontend.md) — Vue 3 shell, chat components, composables*
