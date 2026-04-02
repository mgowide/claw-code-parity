# Building a Desktop App with Tauri

This is the most complete UI option. Tauri gives you a native desktop window with a web frontend that talks directly to the Rust `runtime` crate — no subprocess, no HTTP overhead, no network exposure.

---

## Why Tauri for Claw Code

| Property | Benefit |
|---|---|
| Backend is Rust | Import the `runtime`, `api`, `tools` crates directly as library deps — zero IPC serialization overhead for internal calls |
| Frontend is HTML/JS | Use React, Svelte, Vue, or plain HTML — your choice |
| Native window | System tray, OS notifications, file dialogs, keyboard shortcuts |
| Small binary | Tauri uses the OS WebView (WebView2 on Windows, WebKit on macOS/Linux) — no bundled Chromium |
| Cross-platform | Windows, macOS, Linux from one codebase |
| Secure by default | CSP enforcement, scoped filesystem access, explicit command allowlisting |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Native Window (Tauri WebView)                               │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Frontend (React / Svelte / HTML)                     │   │
│  │                                                       │   │
│  │  ┌────────────┐  ┌───────────┐  ┌────────────────┐  │   │
│  │  │  Chat View  │  │ Tool Log  │  │  Sidebar       │  │   │
│  │  │  (messages  │  │ (live     │  │  - Sessions    │  │   │
│  │  │   stream)   │  │  status)  │  │  - Tools       │  │   │
│  │  └────────────┘  └───────────┘  │  - Settings     │  │   │
│  │                                  └────────────────┘  │   │
│  │  ┌──────────────────────────────────────────────┐    │   │
│  │  │  Input Bar  (multiline, slash completion)     │    │   │
│  │  └──────────────────────────────────────────────┘    │   │
│  │  ┌──────────────────────────────────────────────┐    │   │
│  │  │  Status Bar: model │ tokens │ cost │ branch   │    │   │
│  │  └──────────────────────────────────────────────┘    │   │
│  └──────────────────────────────────────────────────────┘   │
│                          ↕ invoke / emit                     │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Tauri Rust Backend                                   │   │
│  │                                                       │   │
│  │  Commands (IPC):                                      │   │
│  │    create_session, send_message, list_sessions,       │   │
│  │    get_status, compact_session, switch_model,         │   │
│  │    get_config, approve_permission                     │   │
│  │                                                       │   │
│  │  Events (pushed to frontend):                         │   │
│  │    text_delta, tool_use, tool_result, usage,          │   │
│  │    turn_complete, permission_request                   │   │
│  │                                                       │   │
│  │  Directly calls:                                      │   │
│  │    runtime::ConversationRuntime                       │   │
│  │    api::ProviderClient                                │   │
│  │    tools::GlobalToolRegistry                          │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## Project Setup

### 1. Install Tauri CLI

```bash
cargo install tauri-cli --version "^2.0"
```

### 2. Scaffold the Project

Create a new crate alongside the existing ones:

```bash
mkdir -p rust/crates/desktop/src-tauri/src
mkdir -p rust/crates/desktop/src       # frontend source
```

Directory structure:

```
rust/crates/desktop/
├── src-tauri/
│   ├── Cargo.toml              # Tauri Rust backend
│   ├── tauri.conf.json         # Tauri config (window, permissions, etc.)
│   ├── capabilities/
│   │   └── default.json        # allowed Tauri APIs
│   ├── src/
│   │   ├── main.rs             # Tauri entry point
│   │   ├── commands.rs         # IPC command handlers
│   │   ├── state.rs            # Shared app state
│   │   ├── events.rs           # Backend → frontend event bridge
│   │   └── runtime_bridge.rs   # ConversationRuntime wrapper
│   └── icons/                  # App icons
├── src/                        # Frontend source
│   ├── index.html
│   ├── main.js                 # or main.tsx for React
│   ├── styles.css
│   └── components/
│       ├── ChatView.js
│       ├── ToolLog.js
│       ├── Sidebar.js
│       ├── InputBar.js
│       └── StatusBar.js
├── package.json                # Frontend deps (if using npm)
└── vite.config.js              # Frontend bundler
```

### 3. Cargo.toml (Tauri Backend)

```toml
# rust/crates/desktop/src-tauri/Cargo.toml
[package]
name = "claw-desktop"
version.workspace = true
edition.workspace = true

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json.workspace = true
tokio = { version = "1", features = ["rt-multi-thread", "sync", "macros"] }
uuid = { version = "1", features = ["v4"] }

# Import Claw Code crates directly
runtime = { path = "../../runtime" }
api = { path = "../../api" }
tools = { path = "../../tools" }
plugins = { path = "../../plugins" }
commands = { path = "../../commands" }

[lib]
name = "claw_desktop_lib"
crate-type = ["staticlib", "cdylib", "rlib"]
```

### 4. Tauri Configuration

```json
// rust/crates/desktop/src-tauri/tauri.conf.json
{
  "productName": "Claw Code",
  "version": "0.1.0",
  "identifier": "com.clawcode.desktop",
  "build": {
    "frontendDist": "../src",
    "devUrl": "http://localhost:5173",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "app": {
    "withGlobalTauri": true,
    "windows": [
      {
        "title": "Claw Code",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "decorations": true,
        "transparent": false
      }
    ],
    "trayIcon": {
      "iconPath": "icons/icon.png",
      "tooltip": "Claw Code"
    }
  }
}
```

### 5. Capability Permissions

```json
// rust/crates/desktop/src-tauri/capabilities/default.json
{
  "identifier": "default",
  "description": "Claw Code default capability set",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "shell:allow-open",
    "core:event:default"
  ]
}
```

---

## Rust Backend Implementation

### App State

```rust
// src-tauri/src/state.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};
use runtime::{Session, PermissionPolicy, UsageTracker};

pub struct SessionHandle {
    pub id: String,
    pub user_tx: mpsc::Sender<UserAction>,
    pub model: String,
    pub usage: Arc<RwLock<UsageInfo>>,
}

pub enum UserAction {
    SendMessage(String),
    ApprovePermission(String),
    DenyPermission(String),
    Compact,
    Cancel,
}

#[derive(Clone, Default, serde::Serialize)]
pub struct UsageInfo {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cost_usd: f64,
    pub turn_count: usize,
}

pub struct AppState {
    pub sessions: Mutex<HashMap<String, SessionHandle>>,
    pub active_session_id: RwLock<Option<String>>,
    pub model: RwLock<String>,
    pub permission_mode: RwLock<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            active_session_id: RwLock::new(None),
            model: RwLock::new("claude-opus-4-6".into()),
            permission_mode: RwLock::new("danger-full-access".into()),
        }
    }
}
```

### Events Bridge (Rust → Frontend)

```rust
// src-tauri/src/events.rs
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
#[serde(tag = "type")]
pub enum ClawEvent {
    TextDelta { text: String },
    ThinkingStart,
    ThinkingEnd,
    ToolUseStart { id: String, name: String, input: String },
    ToolResult { id: String, output: String, is_error: bool },
    Usage { input_tokens: u32, output_tokens: u32, cost_usd: f64 },
    TurnComplete { turn_index: usize },
    PermissionRequest { tool_name: String, description: String, request_id: String },
    SessionCreated { id: String },
    Error { message: String },
}

pub fn emit(app: &AppHandle, event: ClawEvent) {
    let event_name = match &event {
        ClawEvent::TextDelta { .. } => "claw://text_delta",
        ClawEvent::ThinkingStart => "claw://thinking_start",
        ClawEvent::ThinkingEnd => "claw://thinking_end",
        ClawEvent::ToolUseStart { .. } => "claw://tool_use_start",
        ClawEvent::ToolResult { .. } => "claw://tool_result",
        ClawEvent::Usage { .. } => "claw://usage",
        ClawEvent::TurnComplete { .. } => "claw://turn_complete",
        ClawEvent::PermissionRequest { .. } => "claw://permission_request",
        ClawEvent::SessionCreated { .. } => "claw://session_created",
        ClawEvent::Error { .. } => "claw://error",
    };
    let _ = app.emit(event_name, event);
}
```

### IPC Commands

```rust
// src-tauri/src/commands.rs
use tauri::State;
use std::sync::Arc;
use uuid::Uuid;

use crate::state::{AppState, UserAction};
use crate::runtime_bridge;

#[tauri::command]
pub async fn create_session(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    let model = state.model.read().await.clone();

    runtime_bridge::spawn_session(app, state.inner().clone(), &id, &model).await?;

    *state.active_session_id.write().await = Some(id.clone());
    Ok(id)
}

#[tauri::command]
pub async fn send_message(
    state: State<'_, Arc<AppState>>,
    session_id: String,
    message: String,
) -> Result<(), String> {
    let sessions = state.sessions.lock().await;
    let handle = sessions.get(&session_id).ok_or("session not found")?;
    handle.user_tx.send(UserAction::SendMessage(message))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn approve_permission(
    state: State<'_, Arc<AppState>>,
    session_id: String,
    request_id: String,
) -> Result<(), String> {
    let sessions = state.sessions.lock().await;
    let handle = sessions.get(&session_id).ok_or("session not found")?;
    handle.user_tx.send(UserAction::ApprovePermission(request_id))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn deny_permission(
    state: State<'_, Arc<AppState>>,
    session_id: String,
    request_id: String,
) -> Result<(), String> {
    let sessions = state.sessions.lock().await;
    let handle = sessions.get(&session_id).ok_or("session not found")?;
    handle.user_tx.send(UserAction::DenyPermission(request_id))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_status(
    state: State<'_, Arc<AppState>>,
) -> Result<serde_json::Value, String> {
    let model = state.model.read().await.clone();
    let perm = state.permission_mode.read().await.clone();
    let active = state.active_session_id.read().await.clone();
    let session_count = state.sessions.lock().await.len();

    Ok(serde_json::json!({
        "model": model,
        "permission_mode": perm,
        "active_session": active,
        "sessions_count": session_count,
    }))
}

#[tauri::command]
pub async fn switch_model(
    state: State<'_, Arc<AppState>>,
    model: String,
) -> Result<(), String> {
    *state.model.write().await = model;
    Ok(())
}

#[tauri::command]
pub async fn compact_session(
    state: State<'_, Arc<AppState>>,
    session_id: String,
) -> Result<(), String> {
    let sessions = state.sessions.lock().await;
    let handle = sessions.get(&session_id).ok_or("session not found")?;
    handle.user_tx.send(UserAction::Compact)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_sessions(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<String>, String> {
    let sessions = state.sessions.lock().await;
    Ok(sessions.keys().cloned().collect())
}
```

### Runtime Bridge

This is the core — wraps `ConversationRuntime` and pushes `AssistantEvent`s to the frontend via Tauri events:

```rust
// src-tauri/src/runtime_bridge.rs
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::mpsc;

use api::ProviderClient;
use runtime::{
    AssistantEvent, ConversationRuntime, PermissionMode, PermissionPolicy,
    Session, load_system_prompt, ProjectContext,
};
use tools::GlobalToolRegistry;

use crate::events::{self, ClawEvent};
use crate::state::{AppState, SessionHandle, UserAction};

pub async fn spawn_session(
    app: AppHandle,
    state: Arc<AppState>,
    session_id: &str,
    model: &str,
) -> Result<(), String> {
    let (user_tx, mut user_rx) = mpsc::channel::<UserAction>(32);

    let session_id_owned = session_id.to_string();
    let model_owned = model.to_string();
    let app_clone = app.clone();

    // Spawn the runtime on a background tokio task
    tokio::spawn(async move {
        let provider = ProviderClient::from_model(&model_owned)
            .map_err(|e| e.to_string()).unwrap();

        let tool_registry = GlobalToolRegistry::builtin();
        let session = Session::new();

        let context = ProjectContext::discover_with_git(".", "2026-04-02")
            .unwrap_or_default();
        let system_prompt = load_system_prompt(&context);

        let permission_policy = PermissionPolicy::allow_all();

        // Main loop: wait for user messages, run turns, emit events
        while let Some(action) = user_rx.recv().await {
            match action {
                UserAction::SendMessage(message) => {
                    // Run the turn — each AssistantEvent maps to a ClawEvent
                    // The actual integration depends on the ConversationRuntime API
                    // This is the conceptual bridge:

                    events::emit(&app_clone, ClawEvent::ThinkingStart);

                    // ... drive ConversationRuntime, for each event:
                    // events::emit(&app_clone, ClawEvent::TextDelta { text });
                    // events::emit(&app_clone, ClawEvent::ToolUseStart { id, name, input });
                    // events::emit(&app_clone, ClawEvent::ToolResult { id, output, is_error });
                    // events::emit(&app_clone, ClawEvent::Usage { ... });

                    events::emit(&app_clone, ClawEvent::ThinkingEnd);
                    events::emit(&app_clone, ClawEvent::TurnComplete { turn_index: 0 });
                }
                UserAction::Compact => {
                    // trigger session compaction
                }
                UserAction::Cancel => break,
                _ => {}
            }
        }
    });

    // Store the session handle
    let handle = SessionHandle {
        id: session_id.to_string(),
        user_tx,
        model: model.to_string(),
        usage: Default::default(),
    };
    state.sessions.lock().await.insert(session_id.to_string(), handle);

    events::emit(&app, ClawEvent::SessionCreated { id: session_id.to_string() });

    Ok(())
}
```

### Tauri Entry Point

```rust
// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

mod commands;
mod events;
mod runtime_bridge;
mod state;

fn main() {
    let app_state = Arc::new(state::AppState::new());

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::create_session,
            commands::send_message,
            commands::approve_permission,
            commands::deny_permission,
            commands::get_status,
            commands::switch_model,
            commands::compact_session,
            commands::list_sessions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Frontend Implementation

### package.json (minimal Vite + vanilla JS)

```json
{
  "name": "claw-desktop-frontend",
  "private": true,
  "scripts": {
    "dev": "vite",
    "build": "vite build"
  },
  "devDependencies": {
    "vite": "^6.0.0"
  }
}
```

### Frontend: index.html

```html
<!-- src/index.html -->
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Claw Code</title>
  <link rel="stylesheet" href="./styles.css">
</head>
<body>
  <div id="app">
    <aside id="sidebar">
      <h2>Sessions</h2>
      <ul id="session-list"></ul>
      <button id="new-session">+ New Session</button>
      <hr>
      <div id="status-panel">
        <div>Model: <span id="s-model">—</span></div>
        <div>Mode: <span id="s-mode">—</span></div>
      </div>
    </aside>

    <main id="main-area">
      <div id="messages"></div>

      <div id="tool-notifications"></div>

      <footer>
        <div id="status-bar">
          <span id="s-tokens">0↓ 0↑</span>
          <span id="s-cost">$0.00</span>
          <span id="s-thinking" class="hidden">🧠 Thinking...</span>
        </div>
        <div id="input-row">
          <textarea id="input" rows="3" placeholder="Ask anything..."></textarea>
          <button id="send-btn" onclick="sendMessage()">Send</button>
        </div>
      </footer>
    </main>

    <div id="permission-modal" class="hidden">
      <div class="modal-content">
        <h3>Permission Required</h3>
        <p id="perm-description"></p>
        <div class="modal-actions">
          <button id="perm-allow" class="allow">Allow</button>
          <button id="perm-deny" class="deny">Deny</button>
        </div>
      </div>
    </div>
  </div>
  <script type="module" src="./main.js"></script>
</body>
</html>
```

### Frontend: main.js (Tauri IPC)

```javascript
// src/main.js
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

let currentSessionId = null;
let currentAssistantEl = null;

// ── Tauri Event Listeners ──────────────────────────────────

listen('claw://text_delta', (event) => {
  const { text } = event.payload;
  appendAssistantText(text);
});

listen('claw://thinking_start', () => {
  document.getElementById('s-thinking').classList.remove('hidden');
});

listen('claw://thinking_end', () => {
  document.getElementById('s-thinking').classList.add('hidden');
});

listen('claw://tool_use_start', (event) => {
  const { name, input } = event.payload;
  addToolNotification(`⟳ ${name}`, 'running');
});

listen('claw://tool_result', (event) => {
  const { id, output, is_error } = event.payload;
  const icon = is_error ? '✗' : '✓';
  addToolNotification(`${icon} Tool complete`, is_error ? 'error' : 'success');
});

listen('claw://usage', (event) => {
  const { input_tokens, output_tokens, cost_usd } = event.payload;
  document.getElementById('s-tokens').textContent = `${input_tokens}↓ ${output_tokens}↑`;
  document.getElementById('s-cost').textContent = `$${cost_usd.toFixed(2)}`;
});

listen('claw://turn_complete', () => {
  currentAssistantEl = null;
});

listen('claw://permission_request', (event) => {
  const { tool_name, description, request_id } = event.payload;
  showPermissionModal(tool_name, description, request_id);
});

listen('claw://session_created', (event) => {
  const { id } = event.payload;
  addSessionToList(id);
});

// ── Commands ───────────────────────────────────────────────

async function createSession() {
  currentSessionId = await invoke('create_session');
  await refreshStatus();
}

async function sendMessage() {
  const input = document.getElementById('input');
  const message = input.value.trim();
  if (!message || !currentSessionId) return;

  addMessage('user', message);
  input.value = '';
  currentAssistantEl = null;

  await invoke('send_message', {
    sessionId: currentSessionId,
    message: message,
  });
}

async function refreshStatus() {
  const status = await invoke('get_status');
  document.getElementById('s-model').textContent = status.model;
  document.getElementById('s-mode').textContent = status.permission_mode;
}

// ── Permission Modal ───────────────────────────────────────

function showPermissionModal(toolName, description, requestId) {
  const modal = document.getElementById('permission-modal');
  document.getElementById('perm-description').textContent =
    `${toolName}: ${description}`;

  document.getElementById('perm-allow').onclick = async () => {
    await invoke('approve_permission', {
      sessionId: currentSessionId,
      requestId: requestId,
    });
    modal.classList.add('hidden');
  };

  document.getElementById('perm-deny').onclick = async () => {
    await invoke('deny_permission', {
      sessionId: currentSessionId,
      requestId: requestId,
    });
    modal.classList.add('hidden');
  };

  modal.classList.remove('hidden');
}

// ── DOM Helpers ────────────────────────────────────────────

function addMessage(role, text) {
  const el = document.createElement('div');
  el.className = `message ${role}`;
  el.textContent = text;
  document.getElementById('messages').appendChild(el);
  el.scrollIntoView({ behavior: 'smooth' });
  return el;
}

function appendAssistantText(text) {
  if (!currentAssistantEl) {
    currentAssistantEl = addMessage('assistant', '');
  }
  currentAssistantEl.textContent += text;
  currentAssistantEl.scrollIntoView({ behavior: 'smooth' });
}

function addToolNotification(text, type) {
  const el = document.createElement('div');
  el.className = `tool-notif ${type}`;
  el.textContent = text;
  const container = document.getElementById('tool-notifications');
  container.appendChild(el);
  setTimeout(() => el.remove(), 5000);
}

function addSessionToList(id) {
  const li = document.createElement('li');
  li.textContent = id.slice(0, 8);
  li.onclick = () => { currentSessionId = id; };
  document.getElementById('session-list').appendChild(li);
}

// ── Key Bindings ───────────────────────────────────────────

document.getElementById('input').addEventListener('keydown', (e) => {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault();
    sendMessage();
  }
});

document.getElementById('new-session').onclick = createSession;

// ── Init ───────────────────────────────────────────────────

createSession();
```

### Frontend: styles.css

```css
/* src/styles.css */
:root {
  --bg: #0d1117;
  --bg-sidebar: #161b22;
  --bg-input: #1c2128;
  --fg: #e6edf3;
  --fg-muted: #8b949e;
  --accent: #58a6ff;
  --success: #3fb950;
  --error: #f85149;
  --border: #30363d;
}

* { box-sizing: border-box; margin: 0; padding: 0; }

body {
  font-family: 'SF Mono', 'Cascadia Code', 'Fira Code', monospace;
  background: var(--bg);
  color: var(--fg);
  height: 100vh;
  overflow: hidden;
}

#app {
  display: flex;
  height: 100vh;
}

/* Sidebar */
#sidebar {
  width: 220px;
  background: var(--bg-sidebar);
  border-right: 1px solid var(--border);
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

#sidebar h2 { font-size: 0.9rem; color: var(--fg-muted); text-transform: uppercase; }
#sidebar ul { list-style: none; }
#sidebar li { padding: 0.3rem 0.5rem; cursor: pointer; border-radius: 4px; font-size: 0.85rem; }
#sidebar li:hover { background: var(--border); }
#sidebar button { background: var(--accent); color: #000; border: none; padding: 0.5rem; border-radius: 4px; cursor: pointer; font-weight: 600; }
#sidebar hr { border: none; border-top: 1px solid var(--border); margin: 0.5rem 0; }
#status-panel { font-size: 0.8rem; color: var(--fg-muted); }

/* Main area */
#main-area {
  flex: 1;
  display: flex;
  flex-direction: column;
}

/* Messages */
#messages {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
}

.message {
  margin-bottom: 1rem;
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
}

.message.user {
  color: var(--accent);
  font-weight: 600;
}

.message.user::before { content: 'You: '; }
.message.assistant { color: var(--fg); }

/* Tool notifications */
#tool-notifications {
  padding: 0 1rem;
}

.tool-notif {
  font-size: 0.8rem;
  padding: 0.2rem 0.5rem;
  border-left: 2px solid var(--border);
  margin-bottom: 0.3rem;
  animation: fadeIn 0.2s;
}
.tool-notif.running { border-color: var(--accent); color: var(--accent); }
.tool-notif.success { border-color: var(--success); color: var(--success); }
.tool-notif.error { border-color: var(--error); color: var(--error); }

/* Footer */
footer {
  border-top: 1px solid var(--border);
}

#status-bar {
  display: flex;
  gap: 1rem;
  padding: 0.3rem 1rem;
  font-size: 0.75rem;
  color: var(--fg-muted);
  background: var(--bg-sidebar);
}

#input-row {
  display: flex;
  padding: 0.5rem;
  gap: 0.5rem;
  background: var(--bg-input);
}

#input-row textarea {
  flex: 1;
  background: var(--bg);
  color: var(--fg);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 0.5rem;
  font-family: inherit;
  font-size: 0.9rem;
  resize: none;
}

#input-row textarea:focus { outline: none; border-color: var(--accent); }

#send-btn {
  background: var(--accent);
  color: #000;
  border: none;
  padding: 0.5rem 1.5rem;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
  font-family: inherit;
}

/* Permission modal */
#permission-modal {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.modal-content {
  background: var(--bg-sidebar);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 1.5rem;
  max-width: 400px;
  width: 90%;
}

.modal-content h3 { margin-bottom: 0.5rem; }
.modal-content p { color: var(--fg-muted); margin-bottom: 1rem; }

.modal-actions { display: flex; gap: 0.5rem; }
.modal-actions button { flex: 1; padding: 0.5rem; border: none; border-radius: 4px; cursor: pointer; font-weight: 600; }
.modal-actions .allow { background: var(--success); color: #000; }
.modal-actions .deny { background: var(--error); color: #fff; }

.hidden { display: none !important; }

@keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }

/* Scrollbar */
::-webkit-scrollbar { width: 6px; }
::-webkit-scrollbar-track { background: transparent; }
::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }
```

---

## Building & Running

### Development

```bash
cd rust/crates/desktop

# Install frontend deps
npm install

# Run in dev mode (hot reload frontend + Rust rebuild)
cargo tauri dev
```

### Production Build

```bash
cargo tauri build
```

Output:
- **Windows**: `target/release/bundle/msi/Claw Code_0.1.0_x64_en-US.msi`
- **macOS**: `target/release/bundle/dmg/Claw Code_0.1.0_aarch64.dmg`
- **Linux**: `target/release/bundle/appimage/claw-code_0.1.0_amd64.AppImage`

---

## Feature Roadmap for the Desktop App

| Feature | Effort | Description |
|---|---|---|
| Basic chat + streaming | Medium | Core text exchange with tool call display |
| Permission modal | Small | Visual approve/deny for restricted tools |
| Session sidebar | Small | List, create, switch sessions |
| Model switcher | Small | Dropdown or command palette |
| Markdown rendering | Medium | Use `marked.js` + `highlight.js` in the frontend |
| Tool call collapsible panels | Medium | Expand/collapse tool inputs and outputs |
| File tree browser | Medium | Browse workspace files via `glob_search` |
| Settings page | Medium | Edit `.claude.json` visually |
| Diff viewer | Medium | Colored unified diffs for `edit_file` results |
| System tray | Small | Minimize to tray, quick-launch prompts |
| Keyboard shortcuts | Small | `Ctrl+N` new session, `Ctrl+K` model switcher, etc. |
| Themes | Medium | Dark/light/solarized with CSS variables |
| Auto-update | Small | Tauri's built-in updater plugin |
| Notifications | Small | OS-level notifications on turn completion |

---

## Security Notes

| Concern | How Tauri Handles It |
|---|---|
| Filesystem access | Scoped by Tauri capability permissions — frontend cannot access arbitrary files |
| Shell execution | Only through Claw Code's tool system, controlled by `PermissionPolicy` |
| IPC surface | Only explicitly registered `#[tauri::command]` functions are callable |
| CSP | Tauri enforces a Content Security Policy by default |
| No network exposure | Unlike the Web UI, no HTTP server is running — all IPC is in-process |
| Code signing | Tauri supports signing builds for Windows and macOS distribution |
