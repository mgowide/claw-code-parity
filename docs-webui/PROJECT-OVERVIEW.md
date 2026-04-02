# Claw Code Web UI — Project Overview

> **What we're building, why it matters, every phase in detail, and what makes it outstanding.**

---

## Table of Contents

1. [What Is This Project?](#what-is-this-project)
2. [Why a Web UI?](#why-a-web-ui)
3. [What We Already Have (Foundation)](#what-we-already-have)
4. [Architecture: How It All Connects](#architecture)
5. [Tech Stack at a Glance](#tech-stack)
6. [Phase Breakdown (Complete Roadmap)](#phase-breakdown)
   - [Phase 1 — Core Chat That Works](#phase-1--core-chat-that-works)
   - [Phase 2 — Tool Visualization](#phase-2--tool-visualization)
   - [Phase 3 — Session Management](#phase-3--session-management)
   - [Phase 4 — Composer & Shortcuts](#phase-4--composer--shortcuts)
   - [Phase 5 — Panels & Dashboards](#phase-5--panels--dashboards)
   - [Phase 6 — Polish & Visual Excellence](#phase-6--polish--visual-excellence)
7. [What Makes the UI Outstanding](#what-makes-the-ui-outstanding)
8. [UI Features That Surface Every Capability](#ui-features-by-capability)
9. [Design Principles](#design-principles)
10. [Future: From Web to Desktop (Tauri)](#future-tauri)
11. [Related Documents](#related-documents)

---

## What Is This Project?

**Claw Code Web UI** is a browser-based interface for the Claw Code agent harness — a Rust reimplementation of the Claude Code CLI. Instead of interacting with the AI agent through a terminal, you get a full visual experience:

- **See** every tool call, diff, file read, and terminal command as it happens
- **Control** permissions, model switching, session management in real time
- **Monitor** token usage, cost, and performance with live dashboards
- **Manage** sessions, files, MCP servers, and configuration visually

The CLI stays as-is. The Web UI wraps the **same Rust runtime** — no separate backend, no reimplementation. The browser talks to an Axum web server that imports the same `ConversationRuntime<C, T>` the CLI uses.

---

## Why a Web UI?

| Reason | Detail |
|---|---|
| **Visibility** | A terminal can't show inline diffs, collapsible tool cards, or token usage charts. A browser can. |
| **Accessibility** | Not everyone is comfortable in a terminal. A visual UI opens the project to designers, PMs, and learners. |
| **Developer experience** | Hot-reload with Vite, Chrome DevTools, Vue Devtools — faster iteration than terminal debugging. |
| **Structured output** | Tool results, file trees, permission requests, cost tracking — all deserve dedicated visual components, not wall-of-text. |
| **Path to desktop** | The web frontend becomes the desktop app when we wrap it with Tauri later. Zero wasted work. |

---

## What We Already Have (Foundation)

Before the Web UI, this project already has a fully functional Rust backend:

### Rust Workspace (7 crates)

| Crate | Purpose |
|---|---|
| `rusty-claude-cli` | Main binary — REPL loop, prompt rendering, terminal UI |
| `runtime` | Session lifecycle, config parsing, permissions, compaction, MCP client, OAuth, hooks |
| `api` | HTTP client for Anthropic, xAI, OpenAI-compatible providers (streaming SSE) |
| `tools` | 20 built-in tools: `bash`, `read_file`, `edit_file`, `grep_search`, `list_dir`, `write_file`, `glob`, `task`, `mcp`, etc. |
| `commands` | 25+ slash commands: `/help`, `/model`, `/compact`, `/clear`, `/config`, `/cost`, `/doctor`, etc. |
| `plugins` | Plugin system for external tool injection |
| `telemetry` | Session usage tracking |

### Core Runtime Engine

```
ConversationRuntime<C: ApiClient, T: ToolExecutor>
```

This is the heart. It manages the conversation loop:

1. User sends a message
2. Runtime assembles system prompt + context + history
3. Streams response from the API provider
4. Each chunk emits an `AssistantEvent`:
   - `TextDelta(String)` — partial text
   - `ToolUse { id, name, input }` — the model wants to call a tool
   - `Usage(TokenUsage)` — token counts
   - `PromptCache(PromptCacheEvent)` — cache hit/miss
   - `MessageStop` — turn complete
5. Tool calls are executed, results fed back
6. Repeat until assistant is done

**The Web UI does not rewrite any of this.** It just provides a new way to see and control it.

### What's Already Working in CLI

- Multi-turn conversations with streaming
- All 20 tools with permission enforcement
- Session persistence and resume
- Prompt caching and compaction
- Multi-provider support (Anthropic, xAI, OpenAI-compat)
- OAuth PKCE authentication
- Plugin loading
- `.claude.json` configuration
- MCP server integration
- Cost tracking

---

## Architecture

```
┌──────────────────────┐        WebSocket         ┌──────────────────────┐
│                      │  ◄──────────────────────► │                      │
│   Vue 3 Frontend     │        REST API           │   Axum Backend       │
│   (Browser SPA)      │  ◄──────────────────────► │   (Rust)             │
│                      │                           │                      │
│  ┌────────────────┐  │                           │  ┌────────────────┐  │
│  │ Pinia Stores   │  │   (same binary, same     │  │ Routes         │  │
│  │ Vue Components │  │    process, direct       │  │  /api/sessions │  │
│  │ Composables    │  │    Rust import)           │  │  /api/files    │  │
│  │ Router         │  │                           │  │  /api/config   │  │
│  │ Tailwind + UI  │  │                           │  │  /api/tools    │  │
│  └────────────────┘  │                           │  │  /api/mcp      │  │
│                      │                           │  │  /ws           │  │
└──────────────────────┘                           │  └────────────────┘  │
                                                   │         │            │
                                                   │         ▼            │
                                                   │  ┌────────────────┐  │
                                                   │  │ Existing Crates│  │
                                                   │  │  runtime       │  │
                                                   │  │  api           │  │
                                                   │  │  tools         │  │
                                                   │  │  commands      │  │
                                                   │  │  plugins       │  │
                                                   │  │  telemetry     │  │
                                                   │  └────────────────┘  │
                                                   └──────────────────────┘
```

**Key point**: The Axum server is a new Rust crate (`web-server`) that sits next to the existing crates. It imports `runtime`, `api`, `tools` directly — same code, no duplication, no middleware translation layer.

---

## Tech Stack

### Backend (Rust — extends existing workspace)

| Technology | Role |
|---|---|
| **Axum 0.8** | HTTP server + WebSocket handler |
| **tower / tower-http** | Middleware (CORS, compression, static files) |
| **tokio-tungstenite** | WebSocket protocol |
| Existing crates | `runtime`, `api`, `tools`, `commands`, `plugins`, `telemetry` |

### Frontend (New — Vue 3 SPA)

| Technology | Role |
|---|---|
| **Vue 3.5 + TypeScript** | Core framework — Composition API, reactive refs, `<script setup>` |
| **Vite 6** | Bundler — sub-second HMR, native Vue plugin |
| **Pinia 3** | State management — official Vue store |
| **Naive UI** | Component library — dark theme built-in, 80+ components |
| **Tailwind CSS 4** | Utility-first styling — no CSS files to manage |
| **VueUse** | 200+ composables — `useWebSocket`, `useDraggable`, `useStorage`, etc. |
| **Vue Router 4** | Client-side routing — lazy-loaded views |
| **Monaco Editor** | Code viewer/editor — same engine as VS Code |
| **@xterm/xterm** | Terminal emulator — full ANSI support for tool output |
| **v-code-diff** | Diff viewer — side-by-side + unified diffs |
| **highlight.js** | Syntax highlighting for code blocks |
| **vue-markdown-render** | Markdown rendering with GFM support |
| **Three.js** | Strategic 3D visuals — splash screen, ambient effects, token flow |
| **D3.js** | Data visualization — cost charts, tool usage graphs |
| **Lottie** | Micro-animations — loading states, success/error indicators |
| **KaTeX** | Math equation rendering in messages |
| **lucide-vue-next** | Icon set — clean, consistent |
| **chart.js + vue-chartjs** | Dashboard charts — cost, tokens, session history |

### Explicitly Not Using

| Technology | Why Not |
|---|---|
| **axios** | Unnecessary — native `fetch()` + VueUse `useFetch` covers everything |
| **crypto-js** | Backend handles OAuth/hashing via Rust's `sha2` + `rustls`. Web Crypto API for any frontend needs. |
| **Next.js** | Adds a Node.js middleware layer between browser and Axum. This is a local tool, no SSR/SEO needed. |
| **React** | Vue 3 chosen for lighter runtime, native Vite pairing, simpler state model (Pinia vs Redux) |

---

## Phase Breakdown

### Phase 1 — Core Chat That Works

> **Goal**: Send a message, see streaming response, in a browser. One session, end-to-end.

#### Backend Work
- [ ] Create `rust/crates/web-server/` crate
- [ ] Axum server with `/ws` WebSocket endpoint
- [ ] `POST /api/sessions` — create a new session (wraps `ConversationRuntime`)
- [ ] WebSocket handler that bridges `AssistantEvent` → JSON events to browser
- [ ] `send_message` command handler (client → server)
- [ ] `cancel_turn` command handler
- [ ] Static file serving (Vite build output)
- [ ] CORS middleware for dev mode

#### Frontend Work
- [ ] Scaffold Vue 3 + TypeScript + Vite project
- [ ] Install core deps: Vue Router, Pinia, Naive UI, Tailwind, VueUse
- [ ] `AppShell.vue` — basic layout with sidebar placeholder + main area
- [ ] `ChatView.vue` — default route
- [ ] `ChatThread.vue` — scrollable message list
- [ ] `MessageBubble.vue` — user and assistant messages
- [ ] `StreamingText.vue` — token-by-token rendering with cursor
- [ ] `InputComposer.vue` — multi-line input with send button
- [ ] `useWebSocket.ts` composable — connection + auto-reconnect
- [ ] `useStream.ts` composable — process incoming events
- [ ] `sessionStore.ts` — messages array, connection state
- [ ] `StatusBar.vue` — model name, connection indicator

#### What You'll See After Phase 1
A browser page with a chat interface. Type a message, hit send, watch the AI response stream in token by token. Status bar shows which model is active. That's it — but it's the real runtime underneath, not a mock.

---

### Phase 2 — Tool Visualization

> **Goal**: When the AI calls tools (edit a file, run bash, search code), you see exactly what's happening.

#### Components Built
- [ ] `ToolCallCard.vue` — collapsible card per tool call
  - Spinner while tool is executing
  - Green border on success, red on error
  - Shows input parameters and output
  - Timestamp and duration
- [ ] `DiffViewer.vue` — inline diff for `edit_file` / `write_file` results
  - Side-by-side and unified toggle
  - Line numbers, syntax highlighting
  - Wraps `v-code-diff`
- [ ] `CodeBlock.vue` — syntax-highlighted code with:
  - Language detection badge
  - One-click copy
  - Line numbers
  - "Apply" button (sends edit back to runtime)
- [ ] `TerminalPanel.vue` — embedded xterm.js
  - Renders `bash` / `PowerShell` tool output
  - Full ANSI color support
  - Scrollback buffer
- [ ] `PermissionModal.vue` — when the AI requests a dangerous tool
  - Tool name + description
  - Risk level badge (read / write / danger)
  - "Allow once" / "Always allow" / "Deny" buttons
  - Shows exactly what the tool will do

#### WebSocket Events Added
- `tool_use_start` — triggers ToolCallCard (spinning state)
- `tool_result` — resolves the card (success/error)
- `diff` — triggers DiffViewer
- `permission_request` — triggers PermissionModal

#### What You'll See After Phase 2
The AI says "I'll edit your file" → a tool card appears with a spinner → it resolves showing the diff in color. Bash commands show terminal output with ANSI colors. Permission requests pop up as modals you can approve/deny.

---

### Phase 3 — Session Management

> **Goal**: Multiple sessions, history, resume, search.

#### Backend Work
- [ ] `GET /api/sessions` — list all sessions (name, created, token count, cost)
- [ ] `GET /api/sessions/:id` — full transcript
- [ ] `DELETE /api/sessions/:id` — delete session
- [ ] `GET /api/sessions/:id/export` — export as JSON or Markdown
- [ ] `resume_session` WebSocket command
- [ ] Session persistence to disk (JSON files or SQLite)

#### Frontend Work
- [ ] `SessionsView.vue` — full page listing all sessions
  - Cards with session name, date, token count, cost preview
  - Search bar (full-text across transcripts)
  - Sort by date, cost, tokens
  - Delete with confirmation
  - Export button (JSON / Markdown download)
- [ ] `SessionList.vue` — sidebar component
  - Condensed list of recent sessions
  - Click to switch
  - "New session" button
- [ ] `useSession.ts` composable — active session management
- [ ] Session switching without page reload (Pinia store swap)

#### What You'll See After Phase 3
A sidebar showing all your past sessions. Click one to resume right where you left off. Search across all transcripts to find "that conversation where I set up the database." Export any session as a shareable Markdown file.

---

### Phase 4 — Composer & Shortcuts

> **Goal**: Power-user input experience — slash commands, file attach, keyboard-driven.

#### Components Built
- [ ] `SlashCommandMenu.vue` — autocomplete dropdown
  - Triggered by typing `/`
  - Fuzzy search across all 25+ commands
  - Shows command name + description
  - Arrow keys + Enter to select
  - Groups: session, model, tools, debug
- [ ] `FileAttachBar.vue` — drag + drop file attachment
  - Drop zone with visual feedback
  - File pills showing attached files
  - Click to remove
  - Files sent as context with next message
- [ ] `ModelSelector.vue` — quick model switch
  - Dropdown in status bar or `Ctrl+K`
  - Shows current model, available providers
  - Switch mid-conversation
- [ ] `useKeyboardShortcuts.ts` composable
  - `Ctrl+Enter` — send message
  - `Ctrl+N` — new session
  - `Ctrl+K` — model selector
  - `Ctrl+/` — command palette
  - `Escape` — cancel current turn
  - `Ctrl+L` — clear chat view (not session)
  - `Ctrl+Shift+E` — toggle file tree
  - `Ctrl+B` — toggle sidebar

#### What You'll See After Phase 4
Type `/` and a beautiful autocomplete appears. Drop files onto the input to attach them. `Ctrl+K` opens a model switcher. Every major action has a keyboard shortcut. Power users can fly through interactions without touching the mouse.

---

### Phase 5 — Panels & Dashboards

> **Goal**: Full visibility into everything the runtime exposes — files, todos, cost, tokens, config, MCP.

#### Right Panel Components
- [ ] `RightPanel.vue` — collapsible right sidebar with tabs:
  - **Context**: files currently in context, token count per file
  - **Todos**: kanban board from `TodoWrite` tool output
  - **Tokens**: live gauge showing input/output/cache tokens
  - **Cost**: running total with per-turn breakdown

#### Individual Panel Components
- [ ] `TodoBoard.vue` — kanban-style todo board
  - Columns: Pending → In Progress → Completed
  - Drag to reorder
  - Fed by `TodoWrite` tool events
- [ ] `TokenCounter.vue` — live token usage display
  - Circular gauge or bar
  - Input tokens (blue), output tokens (green), cache hits (yellow)
  - Context window usage percentage
- [ ] `CostTracker.vue` — running cost display
  - Per-session total
  - Per-turn breakdown
  - Daily/weekly graph (chart.js)

#### Full Page Views
- [ ] `FilesView.vue` — workspace file browser
  - Tree view with expand/collapse
  - Click to open in Monaco editor (read-only by default)
  - File size, last modified
  - Syntax highlighting for 100+ languages
- [ ] `ToolsView.vue` — tool dashboard
  - All 20 built-in tools + plugin tools
  - Per-tool usage count this session
  - Permission level badge (read / write / admin)
  - Enable/disable toggle
- [ ] `SettingsView.vue` — visual config editor
  - Maps to `.claude.json` structure
  - Model selection with dropdown
  - Permission mode toggle (default / relaxed / strict)
  - Hook configuration
  - Sandbox settings
  - Save validates JSON before writing
- [ ] `CostView.vue` — cost analytics page
  - Per-session cost chart
  - Per-tool cost breakdown
  - Daily/weekly/monthly usage graphs
  - Token usage by category (input/output/cache)
  - Exportable as CSV
- [ ] `MCPView.vue` — MCP server management
  - List connected servers
  - Show available tools per server
  - Health status (connected / disconnected / error)
  - Add new server (name, transport, URL)
  - Remove server

#### Backend Routes Added
- [ ] `GET /api/files?path=` — list directory
- [ ] `GET /api/files/read?path=&range=` — read file content
- [ ] `GET /api/config` — read `.claude.json`
- [ ] `PUT /api/config` — write `.claude.json`
- [ ] `GET /api/tools` — list all tools + permissions
- [ ] `GET /api/mcp/servers` — list MCP servers
- [ ] `POST /api/mcp/servers` — add MCP server
- [ ] `DELETE /api/mcp/servers/:id` — remove MCP server

#### What You'll See After Phase 5
A right panel showing live token counts, running cost, and todos. A file browser with Monaco-powered code viewing. A settings page where you visually configure everything. A cost dashboard with charts showing where your money goes. MCP servers manageable through the UI.

---

### Phase 6 — Polish & Visual Excellence

> **Goal**: Make it not just functional but genuinely beautiful and delightful to use.

#### Visual Enhancements
- [ ] **Theme system** — dark (default), light, solarized, high-contrast
  - CSS custom properties
  - Theme picker in settings
  - Stored in localStorage via VueUse `useStorage`
- [ ] **Animations** — via `@vueuse/motion`
  - Message bubbles slide in
  - Tool cards expand/collapse smoothly
  - Sidebar items fade in on load
  - Page transitions between views
  - Thinking indicator pulse animation
- [ ] **Three.js strategic visuals**
  - Splash/loading screen with 3D particle effect
  - Token flow visualization (ambient background in cost view)
  - Subtle ambient particle field behind chat (optional, togglable)
  - NOT used for main UI chrome — performance first
- [ ] **Lottie micro-animations**
  - Success checkmark on tool completion
  - Error shake on failures
  - Loading spinners
  - Empty state illustrations
- [ ] **Resizable panels**
  - Drag-to-resize sidebar width
  - Drag-to-resize right panel
  - Drag-to-resize terminal height
  - Persisted via `useStorage`

#### Reliability & Polish
- [ ] **WebSocket reconnect** — auto-reconnect with exponential backoff + visual indicator
- [ ] **Error boundaries** — graceful error handling per component, not page-level crash
- [ ] **Toast notifications** — session compacted, connection lost, permission denied, etc.
- [ ] **Loading skeletons** — Naive UI skeleton components while data loads
- [ ] **Empty states** — illustrated empty states for no sessions, no tools, no messages
- [ ] **Responsive layout** — sidebar collapses on narrow screens, panels stack vertically
- [ ] **PWA support** — service worker, installable, works offline for reading past sessions
- [ ] **Accessibility** — keyboard navigation, ARIA labels, focus management, screen reader support

#### What You'll See After Phase 6
Everything feels smooth, fast, and polished. Animations make state changes feel intentional. Themes let you match your editor. The app recovers gracefully from connection drops. It works on mobile (collapsed layout). It's installable as a PWA. It looks and feels like a professional product.

---

## What Makes the UI Outstanding

This isn't just "a chat interface for an API." Every single runtime capability gets its own visual surface:

### 1. Streaming Intelligence
Messages render token-by-token with a blinking cursor, just like watching someone type in real time. Not a wall of text that appears at once — you see the AI think.

### 2. Tool Transparency
Every tool call is a card. It spins while running, shows exactly what was called and with what parameters, resolves with a green (success) or red (error) border, and for file edits, auto-expands an inline diff. You never wonder "what is the AI doing?"

### 3. Diff-First Editing
When the AI edits a file, you don't just see "file edited." You see the exact changes in a colored diff viewer — old code in red, new code in green, with line numbers and syntax highlighting. One click to view in full Monaco editor.

### 4. Permission Control
When the AI wants to do something risky (delete a file, run a command), a modal pops up showing exactly what will happen, with a risk level badge. "Always allow" remembers your preference. You're always in control.

### 5. Cost Awareness
Token usage and cost are always visible — in the status bar, in the right panel, and in a dedicated dashboard with charts. You know exactly what every session and every tool call costs.

### 6. Session Continuity
Resume any past session exactly where you left off. Search across all transcripts. Export as Markdown. Your work is never lost.

### 7. Power-User Speed
Keyboard shortcuts for everything. Slash command autocomplete. Model switching without leaving the chat. File drag-and-drop. The UI gets out of your way.

### 8. MCP Ecosystem
Manage MCP servers visually — see what tools they provide, their connection status, add or remove them without editing config files.

### 9. Live State Panels
The right panel shows everything happening in the session: files in context, active todos, token counts, running cost. All updating in real time.

### 10. Progressive Disclosure
Simple by default, deep on demand. A new user sees a clean chat interface. A power user opens panels, keyboards, dashboards, and terminal views. The complexity is there when you want it, hidden when you don't.

---

## UI Features by Capability

Every benefit of the Claw Code project gets a dedicated UI surface:

| Project Capability | UI Surface |
|---|---|
| **Fully functional AI agent CLI** | Chat view with streaming, tool cards, permission modals |
| **Hackable open-source** | File browser + Monaco editor + Settings UI for live configuration |
| **Memory-safe Rust runtime** | Status bar showing backend health, connection status, no-crash guarantee |
| **Learning resource** | Tool dashboard showing all 20 tools with descriptions, usage counts |
| **MCP integration** | Dedicated MCP manager page — servers, tools, health, add/remove |
| **Multi-provider support** | Model selector — switch between Anthropic, xAI, OpenAI-compat mid-conversation |
| **Sandboxed execution** | Permission modal with risk badges, "always allow" memory, execution logs |
| **Session continuity** | Session list, resume, search, export — never lose a conversation |
| **Cost tracking** | Cost dashboard — per-session, per-tool, daily/weekly charts, CSV export |
| **Plugin system** | Tools view shows plugin-provided tools alongside built-in ones |
| **Prompt caching** | Token counter shows cache hits (yellow) — visual proof caching is saving money |
| **Compaction** | Toast notification when session is compacted, token gauge drops visually |
| **Slash commands** | Autocomplete dropdown with all 25+ commands, fuzzy search |
| **Configuration** | Visual settings editor mapping to `.claude.json` — no manual JSON editing |
| **OAuth authentication** | Login flow in the browser with PKCE — seamless Anthropic account connection |

---

## Design Principles

1. **Information density without clutter** — Show lots of data, but use progressive disclosure. Panels collapse. Cards collapse. Details are one click away, not always visible.

2. **Real-time by default** — Everything streams. Messages, tool results, token counts, cost. No "refresh" buttons. The UI is always current.

3. **Dark-first** — Developer tool = dark theme default. Light theme available for outdoor/accessibility use.

4. **Keyboard-navigable** — Every action has a shortcut. Tab order is meaningful. Focus states are visible.

5. **Respect existing architecture** — The UI wraps the runtime, it doesn't replace it. Same `ConversationRuntime`, same tools, same permissions. If it works in CLI, it works in Web UI.

6. **No loading spinners that lie** — If something is loading, show a skeleton. If something failed, show the error immediately. No ambiguous states.

7. **Local-first** — This is a developer tool running on localhost. No cloud dependency. No telemetry. Sessions stored locally. PWA for offline reading.

---

## Future: From Web to Desktop (Tauri)

After the Web UI is complete, wrapping it as a native desktop app is straightforward:

```
Web UI (Vue 3 SPA)  →  Tauri 2 Shell  →  Native Desktop App
```

| What Changes | What Stays |
|---|---|
| WebSocket → Tauri IPC (invoke/emit) | All Vue components |
| Axum routes → Tauri commands | All Pinia stores |
| Browser → Native window | All composables |
| `fetch()` → `invoke()` | All styling/themes |

The Vue 3 frontend is reused almost entirely. The backend calls change from HTTP/WebSocket to Tauri's Rust command system. This is why we chose web-first — it's not throwaway work.

---

## Related Documents

| Document | What It Covers |
|---|---|
| [PLAN.md](PLAN.md) | Detailed technical plan — component tree, WebSocket protocol, API routes, npm dependencies |
| [STACK-REPORT.md](STACK-REPORT.md) | Full dependency audit — every crate assessed for risk, maintainer quality, download counts |
| [Architecture Docs](../docs/architecture.md) | Core Rust crate architecture and conversation loop |
| [Tools Reference](../docs/tools.md) | All 20 built-in tools with JSON schemas |
| [Slash Commands](../docs/slash-commands.md) | All 25+ slash commands |
| [Configuration](../docs/configuration.md) | `.claude.json` format and options |
| [Web UI Guide](../docs/guide-web-ui.md) | Original simpler Web UI guide (Axum + SSE) |
| [Tauri Desktop Guide](../docs/guide-tauri-desktop.md) | Full Tauri 2 desktop implementation guide |

---

*Document generated for Claw Code Web UI project planning — April 2, 2026*
