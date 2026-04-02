# Claw Code — Advanced Web UI Plan

## Tech Stack

| Layer | Technology | Why |
|---|---|---|
| **Backend** | Axum (Rust) | Direct import of `runtime`, `api`, `tools` crates — zero overhead |
| **Real-time** | WebSocket (bidirectional) | Frontend can cancel turns, approve permissions, send commands mid-stream |
| **Frontend Framework** | Vue 3.5 + TypeScript | Composition API, reactive refs, native Vite pairing |
| **Bundler** | Vite 6 | Created by Evan You (same as Vue) — canonical pairing |
| **Styling** | Tailwind CSS 4 + Naive UI | Utility-first + dark-theme component library built-in |
| **Markdown** | `vue-markdown-render` + `highlight.js` | Full GFM, syntax highlighting, math (KaTeX) |
| **Diff viewer** | `v-code-diff` | Side-by-side + unified diffs for `edit_file` |
| **Terminal** | `@xterm/xterm` | Embedded terminal for `bash`/`PowerShell` tool output with ANSI color |
| **Code editor** | Monaco Editor (mounted via `onMounted`) | For file viewing/editing with syntax highlighting |
| **State** | Pinia 3 | Official Vue state library, simpler than Redux/Zustand |
| **Composables** | VueUse | 200+ ready composables: `useWebSocket`, `useDraggable`, `useStorage`, etc. |
| **Routing** | Vue Router 4 | Official Vue router |
| **Icons** | Lucide Vue Next | Clean, consistent icon set |
| **Animations** | `@vueuse/motion` | Lighter than Framer Motion, Vue-native |
| **Auth** | OAuth PKCE flow (already in `runtime::oauth`) | Anthropic account login |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│  Vue 3 Frontend (SPA)                                                │
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  App Shell                                                    │    │
│  │  ┌──────────┐ ┌──────────────────────────────┐ ┌──────────┐ │    │
│  │  │ Sidebar   │ │  Main Panel                   │ │ Right    │ │    │
│  │  │           │ │                                │ │ Panel    │ │    │
│  │  │ Sessions  │ │  Chat Thread                   │ │          │ │    │
│  │  │ Files     │ │  ┌─────────────────────────┐  │ │ Context  │ │    │
│  │  │ Tools     │ │  │ Message Bubbles          │  │ │ Files    │ │    │
│  │  │ MCP       │ │  │ ├─ User                  │  │ │ Todos    │ │    │
│  │  │ Settings  │ │  │ ├─ Assistant (streaming)  │  │ │ Tokens   │ │    │
│  │  │           │ │  │ ├─ Tool Calls (animated)  │  │ │ Cost     │ │    │
│  │  │           │ │  │ └─ Diffs (inline)         │  │ │ Model    │ │    │
│  │  │           │ │  └─────────────────────────┘  │ │          │ │    │
│  │  │           │ │                                │ │          │ │    │
│  │  │           │ │  ┌─────────────────────────┐  │ │          │ │    │
│  │  │           │ │  │ Input Composer            │  │ │          │ │    │
│  │  │           │ │  │ (multi-line, /commands,   │  │ │          │ │    │
│  │  │           │ │  │  file attach, drag+drop)  │  │ │          │ │    │
│  │  │           │ │  └─────────────────────────┘  │ │          │ │    │
│  │  └──────────┘ └──────────────────────────────┘ └──────────┘ │    │
│  │  ┌──────────────────────────────────────────────────────────┐│    │
│  │  │ Status Bar: model │ tokens │ cost │ latency │ git branch  ││    │
│  │  └──────────────────────────────────────────────────────────┘│    │
│  └─────────────────────────────────────────────────────────────┘    │
│                            ↕ WebSocket                               │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Axum Backend                 REST + WS                       │   │
│  │  /api/auth/*                  OAuth PKCE login                │   │
│  │  /api/sessions/*              CRUD + resume                   │   │
│  │  /api/config/*                Read/write .claude.json         │   │
│  │  /api/files/*                 Workspace file tree + read      │   │
│  │  /api/mcp/*                   MCP server management           │   │
│  │  /ws                          Bidirectional event stream      │   │
│  │                                                                │   │
│  │  ConversationRuntime<C, T>  ←  direct Rust import             │   │
│  └──────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Pages / Views

| Page | What It Does |
|---|---|
| **Chat** | Main conversation view — streaming messages, tool calls, diffs, permission modals |
| **Sessions** | List all sessions, search, resume, delete, export transcript |
| **Files** | Workspace file tree with Monaco code viewer (read-only or editable) |
| **Tools** | Dashboard of all 20 tools + plugin tools, usage stats per tool |
| **Todos** | Kanban-style board from `TodoWrite` output — pending / in-progress / completed |
| **Terminal** | Embedded xterm.js showing bash/PowerShell tool output with full ANSI support |
| **Settings** | Visual editor for `.claude.json` — model, permissions, hooks, MCP servers |
| **Cost** | Token usage charts, cost-per-session, cost-per-tool breakdown |
| **MCP** | Connected MCP servers, their tools, health status, add/remove |

---

## Frontend Component Tree

```
src/
├── router/
│   └── index.ts                   # Vue Router 4 route definitions
├── views/                         # One file per page (Vue Router targets)
│   ├── ChatView.vue               # Chat (default view)
│   ├── SessionsView.vue
│   ├── FilesView.vue
│   ├── ToolsView.vue
│   ├── TodosView.vue
│   ├── SettingsView.vue
│   └── CostView.vue
├── components/
│   ├── layout/
│   │   ├── AppShell.vue           # Root layout: sidebar + main + right panel
│   │   └── StatusBar.vue          # Bottom bar: model, tokens, cost, latency
│   ├── chat/
│   │   ├── ChatThread.vue         # Scrollable message list
│   │   ├── MessageBubble.vue      # Single message (user or assistant)
│   │   ├── StreamingText.vue      # Animated token-by-token text
│   │   ├── ToolCallCard.vue       # Collapsible tool execution panel
│   │   ├── DiffViewer.vue         # Inline diff for edit_file results
│   │   ├── CodeBlock.vue          # Syntax-highlighted code with copy button
│   │   ├── PermissionModal.vue    # Approve/deny permission requests
│   │   └── ThinkingIndicator.vue  # Animated "thinking" state
│   ├── composer/
│   │   ├── InputComposer.vue      # Multi-line input with slash commands
│   │   ├── SlashCommandMenu.vue   # Autocomplete dropdown for /commands
│   │   ├── FileAttachBar.vue      # Drag+drop file attachment
│   │   └── ModelSelector.vue      # Quick model switch dropdown
│   ├── sidebar/
│   │   ├── AppSidebar.vue         # Main nav sidebar
│   │   ├── SessionList.vue        # Session history with search
│   │   ├── FileTree.vue           # Workspace file explorer
│   │   └── ToolList.vue           # Available tools with status
│   └── panels/
│       ├── RightPanel.vue         # Context panel (files, todos, cost)
│       ├── TodoBoard.vue          # Kanban columns from TodoWrite
│       ├── TokenCounter.vue       # Live token usage gauge
│       ├── CostTracker.vue        # Running cost with session breakdown
│       └── TerminalPanel.vue      # xterm.js embedded terminal
├── composables/                   # Vue composables (replaces React hooks)
│   ├── useWebSocket.ts            # WebSocket connection + auto-reconnect
│   ├── useSession.ts              # Active session state
│   ├── useStream.ts               # Process incoming AssistantEvents
│   └── useKeyboardShortcuts.ts    # Global hotkeys
├── stores/                        # Pinia stores
│   ├── sessionStore.ts            # Sessions, messages, tool calls
│   ├── uiStore.ts                 # Panel visibility, theme, layout
│   └── configStore.ts             # Runtime config mirror
├── lib/
│   ├── ws.ts                      # WebSocket protocol handler
│   ├── api.ts                     # REST API client (typed)
│   ├── markdown.ts                # Markdown → HTML pipeline
│   └── diff.ts                    # Diff parsing utilities
├── types/
│   ├── events.ts                  # ClawEvent discriminated union
│   ├── session.ts                 # Session, Message, ToolCall types
│   └── config.ts                  # Config types matching .claude.json
├── App.vue                        # Root component
└── main.ts                        # Entry: createApp, Pinia, Router
```

---

## WebSocket Protocol

### Server → Client (pushed events)

| Event | Payload | Renders As |
|---|---|---|
| `text_delta` | `{ text: string }` | Appended to streaming message |
| `thinking_start` | `{}` | Animated thinking indicator |
| `thinking_end` | `{}` | Hide indicator |
| `tool_use_start` | `{ id, name, input }` | Tool card appears (spinning) |
| `tool_result` | `{ id, output, is_error }` | Tool card resolves (green/red) |
| `diff` | `{ path, old, new }` | Inline diff viewer |
| `usage` | `{ input_tokens, output_tokens, cost }` | Status bar update |
| `turn_complete` | `{ turn_index }` | Streaming ends, input re-enabled |
| `permission_request` | `{ id, tool, description }` | Modal appears |
| `session_compacted` | `{ removed }` | Toast notification |
| `error` | `{ message }` | Error toast |

### Client → Server (commands)

| Command | Payload |
|---|---|
| `send_message` | `{ session_id, text, attachments? }` |
| `cancel_turn` | `{ session_id }` |
| `approve_permission` | `{ request_id }` |
| `deny_permission` | `{ request_id }` |
| `compact` | `{ session_id }` |
| `switch_model` | `{ model }` |
| `resume_session` | `{ session_id }` |

---

## Backend API Surface (Axum Routes)

```
REST:
  POST   /api/auth/login              OAuth PKCE init
  GET    /api/auth/callback            OAuth callback
  GET    /api/status                    Health + model info
  GET    /api/sessions                  List all sessions
  POST   /api/sessions                  Create session
  GET    /api/sessions/:id              Get session transcript
  DELETE /api/sessions/:id              Delete session
  GET    /api/sessions/:id/export       Export as JSON/Markdown
  GET    /api/files?path=               List directory
  GET    /api/files/read?path=&range=   Read file content
  GET    /api/config                    Read .claude.json
  PUT    /api/config                    Write .claude.json
  GET    /api/tools                     List all tools + permissions
  GET    /api/mcp/servers               List MCP servers
  POST   /api/mcp/servers               Add MCP server
  DELETE /api/mcp/servers/:id           Remove MCP server

WebSocket:
  /ws                                   Bidirectional event stream
```

---

## What Makes It Outstanding

| Feature | Detail |
|---|---|
| **Streaming text animation** | Token-by-token rendering with cursor blink, like a real terminal |
| **Tool call cards** | Each tool call is a collapsible card: spinning while running, shows input/output, green/red border on completion. `edit_file` cards auto-expand a diff viewer |
| **Inline diffs** | `edit_file` and `write_file` results show colored side-by-side diffs right in the chat thread |
| **Embedded terminal** | `bash` and `PowerShell` outputs render in an xterm.js widget with full ANSI color support |
| **Code blocks** | Syntax highlighted with language detection, one-click copy, "Apply" button to send edit back |
| **Slash command autocomplete** | Type `/` and get a fuzzy-search dropdown of all 25+ commands with descriptions |
| **File drag & drop** | Drop files onto the input to attach them as context |
| **Keyboard shortcuts** | `Ctrl+Enter` send, `Ctrl+K` model switch, `Ctrl+N` new session, `Ctrl+/` command palette, `Escape` cancel turn |
| **Resizable panels** | Drag-to-resize sidebar, right panel, terminal pane |
| **Theme system** | Dark (default), light, solarized, high-contrast — stored in localStorage, CSS variables |
| **Session search** | Full-text search across all session transcripts |
| **Cost dashboard** | Per-session cost chart, per-tool cost breakdown, daily/weekly usage graphs |
| **Permission UX** | Modal with tool name, description, risk level badge (read/write/danger), "Always allow" checkbox |
| **Responsive** | Collapses sidebar on mobile, stacks panels vertically |
| **PWA** | Service worker, installable, works offline for reading past sessions |

---

## Development Phases

| Phase | Deliverable | Key Work |
|---|---|---|
| **Phase 1** | **Core chat that works** | Axum server, WebSocket, Vue 3 shell, streaming text, send/receive messages, one session |
| **Phase 2** | **Tool visualization** | ToolCallCard, DiffViewer, CodeBlock, TerminalPanel, permission modal |
| **Phase 3** | **Session management** | Multi-session, session list, resume, search, delete, export |
| **Phase 4** | **Composer & shortcuts** | Slash command autocomplete, file drag+drop, keyboard shortcuts, model selector |
| **Phase 5** | **Panels & dashboards** | Right panel (todos, cost, tokens), file tree, settings UI, MCP manager |
| **Phase 6** | **Polish** | Animations, themes, responsive, PWA, error handling, reconnect logic |

---

## npm Dependencies (Frontend)

```json
{
  "dependencies": {
    "vue": "^3.5",
    "vue-router": "^4.0",
    "pinia": "^3.0",
    "@vueuse/core": "^13.0",
    "@vueuse/motion": "^3.0",
    "naive-ui": "^2.0",
    "vue-markdown-render": "^2.0",
    "highlight.js": "^11.0",
    "v-code-diff": "^1.0",
    "@xterm/xterm": "^5.0",
    "@xterm/addon-fit": "^0.10",
    "monaco-editor": "^0.52",
    "lucide-vue-next": "^0.500",
    "katex": "^0.16"
  },
  "devDependencies": {
    "typescript": "^5.7",
    "vite": "^6.0",
    "@vitejs/plugin-vue": "^5.0",
    "vue-tsc": "^2.0",
    "tailwindcss": "^4.0",
    "autoprefixer": "^10.0"
  }
}
```

---

## Build Inventory Summary

| Category | Count | Items |
|---|---|---|
| **Rust backend** | ~8 files | main, routes, ws handler, state, session manager, file API, config API, MCP API |
| **Vue views** | 7 | Chat, Sessions, Files, Tools, Todos, Settings, Cost |
| **Vue components** | ~30 | Chat thread, message bubble, streaming text, tool card, diff viewer, code block, terminal, composer, sidebar, etc. |
| **Composables** | 4 | WebSocket, session, streams, keyboard |
| **Pinia stores** | 3 | Sessions, UI, config |
| **Lib utilities** | 4 | WS protocol, REST client, markdown pipeline, diff parser |
| **Type definitions** | 3 | Events, sessions, config |
