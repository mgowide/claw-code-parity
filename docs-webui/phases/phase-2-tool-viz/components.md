# Phase 2 — Tool Visualization Components

> Detailed specs for each tool visualization component.

---

## ToolCallCard.vue

**Where**: Rendered inside `ChatThread.vue` when a `tool_use_start` event arrives.

### States

| State | Visual |
|---|---|
| **Running** | Blue border, spinner icon, tool name + input params shown |
| **Success** | Green border, checkmark icon, output shown (collapsible) |
| **Error** | Red border, X icon, error message shown |

### Layout

```
┌─────────────────────────────────────────────┐
│ 🔧 read_file                    ⏱ 0.3s   ▼ │  ← header (always visible)
├─────────────────────────────────────────────┤
│ Input:                                       │  ← collapsible body
│   path: "src/main.rs"                        │
│   start_line: 1                              │
│   end_line: 50                               │
│                                              │
│ Output:                                      │
│   (file content or diff viewer)              │
└─────────────────────────────────────────────┘
```

### Behavior

- Click header to expand/collapse
- `edit_file` → auto-expand, show `<DiffViewer />`
- `bash` → show `<TerminalPanel />` for output
- `read_file` → show `<CodeBlock />` for content
- `grep_search` → show results with file links
- Duration timer starts on `tool_use_start`, stops on `tool_result`

### Props

```typescript
interface ToolCallProps {
  id: string
  name: string
  input: Record<string, unknown>
  output?: string
  isError?: boolean
  status: 'running' | 'success' | 'error'
  duration?: number
}
```

---

## DiffViewer.vue

**Where**: Inside `ToolCallCard` for `edit_file` and `write_file` results.

### Modes

- **Side-by-side** — old file on left, new on right
- **Unified** — single column with +/- lines
- Toggle button between modes

### Features

- Line numbers
- Syntax highlighting (language detected from file extension)
- Scroll sync between side-by-side panels
- Expand/collapse unchanged regions (show ±3 lines of context)

### Dependencies

Uses `v-code-diff` library:

```vue
<template>
  <CodeDiff
    :old-string="oldContent"
    :new-string="newContent"
    :file-name="filePath"
    output-format="side-by-side"
    theme="dark"
  />
</template>
```

### Props

```typescript
interface DiffViewerProps {
  filePath: string
  oldContent: string
  newContent: string
  outputFormat?: 'side-by-side' | 'unified'
}
```

---

## CodeBlock.vue

**Where**: Inside assistant messages (markdown code fences) and `ToolCallCard` output for file reads.

### Features

| Feature | Implementation |
|---|---|
| Syntax highlighting | `highlight.js` with auto-detection or ```language hint |
| Language badge | Top-right label showing detected language |
| Copy button | Copies raw code to clipboard, shows "Copied!" toast |
| Line numbers | Optional, on by default for >5 lines |
| Word wrap | Toggleable |
| Max height | 400px with scroll, "Expand" button to show full |

### Props

```typescript
interface CodeBlockProps {
  code: string
  language?: string
  showLineNumbers?: boolean
  maxHeight?: number
  fileName?: string   // shown as header if provided
}
```

---

## TerminalPanel.vue

**Where**: Inside `ToolCallCard` for `bash` tool output, also used standalone in Phase 5.

### Features

- Full xterm.js terminal emulator
- ANSI color code rendering (256 colors + bold/italic/underline)
- Auto-fit to container width (`@xterm/addon-fit`)
- Scrollback buffer (1000 lines)
- Read-only in tool card context (no input)
- Copy selection to clipboard

### Setup

```typescript
import { Terminal } from '@xterm/xterm'
import { FitAddon } from '@xterm/addon-fit'

// In onMounted:
const term = new Terminal({
  theme: { background: '#1a1a2e', foreground: '#e0e0e0' },
  fontSize: 13,
  fontFamily: 'JetBrains Mono, Consolas, monospace',
  cursorBlink: false,   // read-only
  disableStdin: true,    // read-only
  scrollback: 1000,
})
const fitAddon = new FitAddon()
term.loadAddon(fitAddon)
term.open(containerRef.value)
fitAddon.fit()
```

### Props

```typescript
interface TerminalPanelProps {
  output: string     // raw terminal output with ANSI codes
  maxHeight?: number // default: 300px
}
```

---

## PermissionModal.vue

**Where**: Overlay on top of chat, triggered by `permission_request` WebSocket event.

### Layout

```
┌─────────────────────────────────────────────┐
│           ⚠️ Permission Request              │
│                                              │
│  Tool: bash                                  │
│  Risk: ██ HIGH                               │
│                                              │
│  The assistant wants to run:                 │
│  ┌──────────────────────────────────────┐   │
│  │ rm -rf node_modules && npm install    │   │
│  └──────────────────────────────────────┘   │
│                                              │
│  ☐ Always allow this tool                    │
│                                              │
│  [ Deny ]                    [ Allow Once ]  │
└─────────────────────────────────────────────┘
```

### Risk Levels

| Level | Badge Color | Tools |
|---|---|---|
| **Read** | Blue | `read_file`, `list_dir`, `grep_search`, `glob` |
| **Write** | Yellow | `edit_file`, `write_file`, `notebook_edit` |
| **Danger** | Red | `bash`, `multi_edit` + any unknown |

### Behavior

- Modal backdrop blocks all interaction
- Keyboard: `Enter` = Allow, `Escape` = Deny
- "Always allow" creates a session-level permission override
- Sends `approve_permission` or `deny_permission` via WebSocket
- Queues multiple requests (shows one at a time)

### Props

```typescript
interface PermissionModalProps {
  requestId: string
  tool: string
  description: string
  riskLevel: 'read' | 'write' | 'danger'
}
```

---

*Back: [README](README.md) | Events: [events.md](events.md)*
