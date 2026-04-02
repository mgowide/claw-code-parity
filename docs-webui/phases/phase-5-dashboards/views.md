# Phase 5 — Dashboard Views

> Full-page views: Files, Tools, Settings, Cost, MCP.

---

## FilesView.vue

**Route**: `/files`

### Layout

```
┌─────────────────────┬────────────────────────────────────┐
│  File Tree           │  Monaco Editor (read-only)         │
│                      │                                    │
│  📁 src/             │  1 │ fn main() {                   │
│    📁 routes/        │  2 │     let app = Router::new()   │
│      📄 health.rs    │  3 │         .route("/api/status") │
│      📄 session.rs   │  4 │         .get(health);         │
│    📁 ws/            │  5 │ }                             │
│      📄 handler.rs ← │  6 │                               │
│    📄 main.rs        │                                    │
│    📄 state.rs       │  File: src/ws/handler.rs           │
│                      │  Lines: 145 │ Language: Rust        │
└─────────────────────┴────────────────────────────────────┘
```

### Features

- **Tree view**: expand/collapse folders, file icons by extension
- **Monaco viewer**: syntax highlighting, read-only by default
- **File info**: line count, language, file size
- Populated from `GET /api/files?path=.` (recursive tree)
- File content from `GET /api/files/read?path=src/main.rs`

---

## ToolsView.vue

**Route**: `/tools`

### Layout

```
┌─────────────────────────────────────────────────────────┐
│  Tools (20 built-in + 3 plugins)                        │
│                                                          │
│  ┌─────────────────────────────────────────────────┐    │
│  │ 🔧 bash              ██████████  12 calls  🔴   │    │
│  │ Execute shell commands                           │    │
│  │ Permission: DANGER │ Status: Enabled             │    │
│  ├─────────────────────────────────────────────────┤    │
│  │ 🔧 read_file         ████████░░   8 calls  🔵   │    │
│  │ Read file contents with line range               │    │
│  │ Permission: READ │ Status: Enabled               │    │
│  ├─────────────────────────────────────────────────┤    │
│  │ 🔧 edit_file         ██████░░░░   6 calls  🟡   │    │
│  │ Edit file with search-replace                    │    │
│  │ Permission: WRITE │ Status: Enabled              │    │
│  └─────────────────────────────────────────────────┘    │
│                                                          │
│  Sort by: [Usage ▼] [Name] [Permission]                  │
│  Filter: [All] [Read] [Write] [Danger] [Plugin]          │
└─────────────────────────────────────────────────────────┘
```

### Data Source

`GET /api/tools` returns:

```json
{
  "tools": [
    { "name": "bash", "description": "Execute shell commands", "permission": "danger", "source": "builtin", "call_count": 12, "enabled": true },
    { "name": "read_file", "description": "Read file contents", "permission": "read", "source": "builtin", "call_count": 8, "enabled": true }
  ]
}
```

---

## SettingsView.vue

**Route**: `/settings`

### Layout

```
┌─────────────────────────────────────────────────────────┐
│  Settings                                    [Save] [↻] │
│                                                          │
│  Model                                                   │
│  ┌────────────────────────────────────────────────┐     │
│  │ claude-sonnet-4-20250514                    ▼  │     │
│  └────────────────────────────────────────────────┘     │
│                                                          │
│  Permission Mode                                         │
│  ○ Default  ● Relaxed  ○ Strict                          │
│                                                          │
│  Allowed Tools                                           │
│  ☑ bash  ☑ read_file  ☑ edit_file  ☐ multi_edit         │
│                                                          │
│  Hooks                                                   │
│  ┌────────────────────────────────────────────────┐     │
│  │ pre_tool_call: "echo running $TOOL_NAME"       │     │
│  │ post_session: ""                                │     │
│  └────────────────────────────────────────────────┘     │
│                                                          │
│  MCP Servers: [Managed on MCP page →]                    │
│                                                          │
│  Raw JSON  [Toggle]                                      │
│  ┌────────────────────────────────────────────────┐     │
│  │ { "model": "claude-sonnet-4-20250514", ...     │     │
│  └────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────┘
```

### Features

- Visual form fields map to `.claude.json` structure
- Toggle to raw JSON editor (Monaco)
- Validates before saving (`PUT /api/config`)
- Reset button reverts to saved state
- Shows which fields differ from defaults

---

## CostView.vue

**Route**: `/cost`

### Layout

```
┌─────────────────────────────────────────────────────────┐
│  Cost Analytics                          [Export CSV]     │
│                                                          │
│  Total: $1.24  │  Today: $0.42  │  Sessions: 12         │
│                                                          │
│  Per-Session Cost                                        │
│  ┌────────────────────────────────────────────────┐     │
│  │  ▓▓▓▓▓▓▓▓▓▓                                   │     │
│  │  ▓▓▓▓▓                                         │     │
│  │  ▓▓▓▓▓▓▓                                       │     │
│  │  Session 1   Session 2   Session 3              │     │
│  └────────────────────────────────────────────────┘     │
│                                                          │
│  Per-Tool Cost Breakdown          Daily Usage            │
│  ┌──────────────────────┐  ┌───────────────────────┐   │
│  │ bash:      $0.45     │  │      ___              │   │
│  │ edit_file: $0.32     │  │  ___/   \___          │   │
│  │ read_file: $0.18     │  │ /           \___      │   │
│  │ other:     $0.29     │  │ Mon Tue Wed Thu Fri   │   │
│  └──────────────────────┘  └───────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Charts

- **Per-session bar chart** — chart.js via vue-chartjs
- **Per-tool pie/bar chart** — shows which tools cost the most
- **Daily line chart** — trend over time
- **Token breakdown** — input vs output vs cache savings

---

## MCPView.vue

**Route**: `/mcp`

### Layout

```
┌─────────────────────────────────────────────────────────┐
│  MCP Servers                              [+ Add Server] │
│                                                          │
│  ┌─────────────────────────────────────────────────┐    │
│  │ 🟢 filesystem-server                            │    │
│  │ Transport: stdio │ Tools: 5 │ Connected          │    │
│  │ Tools: read_file, write_file, list_dir, ...      │    │
│  │                                          [Remove] │    │
│  ├─────────────────────────────────────────────────┤    │
│  │ 🔴 postgres-server                              │    │
│  │ Transport: sse │ Tools: 0 │ Disconnected         │    │
│  │ Error: Connection refused                        │    │
│  │                                [Retry] [Remove]  │    │
│  └─────────────────────────────────────────────────┘    │
│                                                          │
│  Add Server Dialog:                                      │
│  Name: [________________]                                │
│  Transport: [stdio ▼]                                    │
│  Command/URL: [________________]                         │
│  [Cancel]                                    [Connect]   │
└─────────────────────────────────────────────────────────┘
```

### Features

- List all connected MCP servers with health status
- Show tools provided by each server
- Add new server (name, transport type, command/URL)
- Remove server
- Retry connection for disconnected servers

---

*Back: [README](README.md) | Panels: [panels.md](panels.md) | API: [api.md](api.md)*
