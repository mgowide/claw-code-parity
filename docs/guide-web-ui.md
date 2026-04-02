# Building a Web UI with Axum

This guide explains how to build a browser-based frontend for Claw Code by wrapping the `runtime` crate in an Axum web server.

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│  Browser (React / Svelte / plain HTML + JS)          │
│                                                      │
│  ┌──────────┐  ┌──────────┐  ┌───────────────────┐  │
│  │ Chat UI   │  │ Tool log │  │ Status / Sidebar  │  │
│  └──────────┘  └──────────┘  └───────────────────┘  │
└─────────────────────┬───────────────────────────────┘
                      │  WebSocket / SSE
┌─────────────────────┴───────────────────────────────┐
│  Axum Web Server  (new crate: crates/web-server/)    │
│                                                      │
│  POST /api/sessions          → create session        │
│  GET  /api/sessions          → list sessions         │
│  GET  /api/sessions/:id      → get session           │
│  POST /api/sessions/:id/turn → send user message     │
│  GET  /api/sessions/:id/stream → SSE event stream    │
│  POST /api/sessions/:id/compact → trigger compaction │
│  GET  /api/status            → health + model info   │
└─────────────────────┬───────────────────────────────┘
                      │  direct Rust calls
┌─────────────────────┴───────────────────────────────┐
│  runtime crate                                       │
│  ConversationRuntime<C, T>                           │
│  Session, PermissionPolicy, UsageTracker, etc.       │
└──────────────────────────────────────────────────────┘
```

`ConversationRuntime` is a pure library — it never touches stdout. Its `AssistantEvent` stream (`TextDelta`, `ToolUse`, `Usage`, `MessageStop`) maps directly to server-sent events or WebSocket frames.

---

## Project Setup

### 1. Create the Crate

```bash
mkdir -p rust/crates/web-server/src
```

```toml
# rust/crates/web-server/Cargo.toml
[package]
name = "claw-web"
version.workspace = true
edition.workspace = true

[[bin]]
name = "claw-web"
path = "src/main.rs"

[dependencies]
runtime = { path = "../runtime" }
api = { path = "../api" }
tools = { path = "../tools" }
plugins = { path = "../plugins" }

axum = { version = "0.8", features = ["ws"] }
tokio = { version = "1", features = ["rt-multi-thread", "signal", "macros"] }
tower-http = { version = "0.6", features = ["cors", "fs"] }
serde.workspace = true
serde_json.workspace = true
uuid = { version = "1", features = ["v4"] }
```

Add it to the workspace:

```toml
# rust/Cargo.toml
[workspace]
members = ["crates/*"]
```

### 2. Static File Serving

Put your frontend in `rust/crates/web-server/static/`. Axum serves it at `/`:

```rust
use tower_http::services::ServeDir;

let app = Router::new()
    .nest("/api", api_routes())
    .fallback_service(ServeDir::new("static"));
```

---

## Server Implementation

### Main Entry Point

```rust
// crates/web-server/src/main.rs
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{Router, routing::{get, post}};

mod routes;
mod state;
mod stream;

#[tokio::main]
async fn main() {
    let state = Arc::new(state::AppState::new());

    let app = Router::new()
        .route("/api/status", get(routes::status))
        .route("/api/sessions", get(routes::list_sessions))
        .route("/api/sessions", post(routes::create_session))
        .route("/api/sessions/{id}", get(routes::get_session))
        .route("/api/sessions/{id}/turn", post(routes::send_turn))
        .route("/api/sessions/{id}/stream", get(routes::stream_events))
        .route("/api/sessions/{id}/compact", post(routes::compact_session))
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive())
        .fallback_service(tower_http::services::ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Claw Web UI running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

### Shared State

```rust
// crates/web-server/src/state.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use runtime::{Session, ConversationRuntime, PermissionPolicy};

pub struct SessionHandle {
    pub session: Session,
    pub event_tx: broadcast::Sender<StreamEvent>,
    // runtime runs on a spawned task, receives user messages via a channel
    pub user_tx: tokio::sync::mpsc::Sender<String>,
}

pub struct AppState {
    pub sessions: Mutex<HashMap<String, SessionHandle>>,
    pub model: String,
    pub permission_mode: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            model: "claude-opus-4-6".into(),
            permission_mode: "danger-full-access".into(),
        }
    }
}
```

### SSE Streaming

The critical piece — stream `AssistantEvent`s to the browser in real time:

```rust
// crates/web-server/src/stream.rs
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::stream::Stream;
use runtime::AssistantEvent;
use serde_json::json;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Clone, serde::Serialize)]
#[serde(tag = "type")]
pub enum StreamEvent {
    TextDelta { text: String },
    ToolUseStart { id: String, name: String },
    ToolUseInput { id: String, input: String },
    ToolResult { id: String, output: String, is_error: bool },
    Usage { input_tokens: u32, output_tokens: u32 },
    TurnComplete,
    Error { message: String },
}

impl From<AssistantEvent> for StreamEvent {
    fn from(event: AssistantEvent) -> Self {
        match event {
            AssistantEvent::TextDelta(text) => StreamEvent::TextDelta { text },
            AssistantEvent::ToolUse { id, name, input } => StreamEvent::ToolUseStart { id, name },
            AssistantEvent::Usage(usage) => StreamEvent::Usage {
                input_tokens: usage.input_tokens,
                output_tokens: usage.output_tokens,
            },
            AssistantEvent::MessageStop => StreamEvent::TurnComplete,
            _ => StreamEvent::Error { message: "unknown event".into() },
        }
    }
}

pub fn sse_stream(
    rx: broadcast::Receiver<StreamEvent>,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let stream = BroadcastStream::new(rx).filter_map(|result| {
        result.ok().map(|event| {
            Ok(Event::default()
                .event(event_type(&event))
                .json_data(event)
                .unwrap())
        })
    });
    Sse::new(stream).keep_alive(KeepAlive::default())
}

fn event_type(event: &StreamEvent) -> &'static str {
    match event {
        StreamEvent::TextDelta { .. } => "text_delta",
        StreamEvent::ToolUseStart { .. } => "tool_use_start",
        StreamEvent::ToolUseInput { .. } => "tool_use_input",
        StreamEvent::ToolResult { .. } => "tool_result",
        StreamEvent::Usage { .. } => "usage",
        StreamEvent::TurnComplete => "turn_complete",
        StreamEvent::Error { .. } => "error",
    }
}
```

### Route Handlers

```rust
// crates/web-server/src/routes.rs
use axum::{extract::{Path, State}, Json, response::sse::Sse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;
use crate::stream;

#[derive(Serialize)]
pub struct StatusResponse {
    model: String,
    permission_mode: String,
    sessions_active: usize,
}

pub async fn status(State(state): State<Arc<AppState>>) -> Json<StatusResponse> {
    let sessions = state.sessions.lock().await;
    Json(StatusResponse {
        model: state.model.clone(),
        permission_mode: state.permission_mode.clone(),
        sessions_active: sessions.len(),
    })
}

#[derive(Deserialize)]
pub struct TurnRequest {
    message: String,
}

pub async fn send_turn(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<TurnRequest>,
) -> Json<serde_json::Value> {
    let sessions = state.sessions.lock().await;
    if let Some(handle) = sessions.get(&id) {
        handle.user_tx.send(body.message).await.ok();
        Json(serde_json::json!({ "status": "ok" }))
    } else {
        Json(serde_json::json!({ "error": "session not found" }))
    }
}

pub async fn stream_events(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Sse<impl futures::Stream<Item = Result<axum::response::sse::Event, std::convert::Infallible>>>
{
    let sessions = state.sessions.lock().await;
    let rx = sessions
        .get(&id)
        .expect("session not found")
        .event_tx
        .subscribe();

    stream::sse_stream(rx)
}
```

---

## Frontend (Minimal HTML + JS)

A minimal but functional chat UI that connects to the SSE stream:

```html
<!-- static/index.html -->
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Claw Code</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body { font-family: 'SF Mono', monospace; background: #1a1a2e; color: #e0e0e0; height: 100vh; display: flex; flex-direction: column; }
    #messages { flex: 1; overflow-y: auto; padding: 1rem; }
    .msg { margin-bottom: 0.75rem; line-height: 1.5; }
    .msg.user { color: #00d4ff; }
    .msg.assistant { color: #e0e0e0; }
    .msg.tool { color: #888; font-size: 0.85em; border-left: 2px solid #444; padding-left: 0.5rem; }
    #status-bar { background: #16213e; padding: 0.4rem 1rem; font-size: 0.8em; color: #aaa; display: flex; gap: 1rem; }
    #input-area { display: flex; padding: 0.5rem; background: #0f3460; }
    #input-area textarea { flex: 1; background: #1a1a2e; color: #e0e0e0; border: 1px solid #333; padding: 0.5rem; font-family: inherit; resize: none; }
    #input-area button { background: #00d4ff; color: #000; border: none; padding: 0.5rem 1rem; cursor: pointer; font-weight: bold; }
  </style>
</head>
<body>

<div id="messages"></div>
<div id="status-bar">
  <span id="s-model">model: —</span>
  <span id="s-tokens">tokens: 0↓ 0↑</span>
  <span id="s-cost">$0.00</span>
</div>
<div id="input-area">
  <textarea id="input" rows="2" placeholder="Type a message..."></textarea>
  <button onclick="send()">Send</button>
</div>

<script>
  let sessionId = null;
  let eventSource = null;
  const messages = document.getElementById('messages');

  async function init() {
    // Create a session
    const res = await fetch('/api/sessions', { method: 'POST' });
    const data = await res.json();
    sessionId = data.id;

    // Connect SSE
    eventSource = new EventSource(`/api/sessions/${sessionId}/stream`);

    eventSource.addEventListener('text_delta', (e) => {
      const { text } = JSON.parse(e.data);
      appendOrUpdate('assistant', text);
    });

    eventSource.addEventListener('tool_use_start', (e) => {
      const { name } = JSON.parse(e.data);
      addMessage('tool', `╭─ ${name} ─╮`);
    });

    eventSource.addEventListener('tool_result', (e) => {
      const { output, is_error } = JSON.parse(e.data);
      const icon = is_error ? '✗' : '✓';
      addMessage('tool', `${icon} ${output.slice(0, 200)}`);
    });

    eventSource.addEventListener('usage', (e) => {
      const { input_tokens, output_tokens } = JSON.parse(e.data);
      document.getElementById('s-tokens').textContent = `tokens: ${input_tokens}↓ ${output_tokens}↑`;
    });

    eventSource.addEventListener('turn_complete', () => {
      // Ready for next message
    });

    // Fetch status
    const status = await (await fetch('/api/status')).json();
    document.getElementById('s-model').textContent = `model: ${status.model}`;
  }

  async function send() {
    const input = document.getElementById('input');
    const text = input.value.trim();
    if (!text || !sessionId) return;

    addMessage('user', text);
    input.value = '';

    await fetch(`/api/sessions/${sessionId}/turn`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message: text }),
    });
  }

  // Enter to send, Shift+Enter for newline
  document.getElementById('input').addEventListener('keydown', (e) => {
    if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); send(); }
  });

  let currentAssistantEl = null;
  function appendOrUpdate(role, text) {
    if (role === 'assistant' && currentAssistantEl) {
      currentAssistantEl.textContent += text;
    } else {
      currentAssistantEl = addMessage(role, text);
    }
    messages.scrollTop = messages.scrollHeight;
  }

  function addMessage(role, text) {
    currentAssistantEl = null;
    const div = document.createElement('div');
    div.className = `msg ${role}`;
    div.textContent = text;
    messages.appendChild(div);
    messages.scrollTop = messages.scrollHeight;
    if (role === 'assistant') currentAssistantEl = div;
    return div;
  }

  init();
</script>
</body>
</html>
```

---

## Running

```bash
cd rust
cargo run -p claw-web
# → Claw Web UI running at http://127.0.0.1:3000
```

Open `http://127.0.0.1:3000` in your browser.

---

## Security Considerations

| Concern | Mitigation |
|---|---|
| The agent runs bash commands | Bind to `127.0.0.1` only; never expose publicly without auth |
| No authentication by default | Add token-based auth middleware before deploying anywhere shared |
| CORS | `CorsLayer::permissive()` is fine for local; lock down for production |
| Session data in memory | Sessions vanish on restart; persist to disk if needed |
| Tool permissions | `PermissionPolicy` still enforces tool restrictions server-side |

---

## Extending

| Feature | How |
|---|---|
| Markdown rendering | Use `marked.js` or `markdown-it` in the frontend |
| Syntax highlighting | `highlight.js` or `Prism.js` for code blocks |
| File tree sidebar | Add `GET /api/files?path=` endpoint backed by `glob_search` |
| Multiple concurrent sessions | Already supported — each session has its own runtime |
| Auth | Add `tower-http` `auth` layer or `axum-login` |
| Deploy | Docker container, bind to 0.0.0.0, add TLS via reverse proxy |
