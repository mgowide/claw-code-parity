# Phase 4 — Composer Components

> Slash command autocomplete, file attachment, model selector, and keyboard shortcuts.

---

## SlashCommandMenu.vue

**Where**: Overlay dropdown inside `InputComposer`, triggered when user types `/`.

### Command Registry

```typescript
const COMMANDS = [
  // Session
  { name: '/clear', description: 'Clear conversation display', group: 'session' },
  { name: '/compact', description: 'Compact conversation to reduce tokens', group: 'session' },
  { name: '/cost', description: 'Show token usage and cost', group: 'session' },
  
  // Model
  { name: '/model', description: 'Switch AI model', group: 'model', hasArg: true },
  
  // Tools
  { name: '/allowed-tools', description: 'Show currently allowed tools', group: 'tools' },
  { name: '/permissions', description: 'Show permission settings', group: 'tools' },
  
  // Debug
  { name: '/doctor', description: 'Check system health', group: 'debug' },
  { name: '/config', description: 'Show current configuration', group: 'debug' },
  { name: '/status', description: 'Show session status', group: 'debug' },
  
  // Files
  { name: '/add-dir', description: 'Add directory to context', group: 'files', hasArg: true },
  
  // ... all 25+ commands from commands crate
]
```

### Fuzzy Matching

```typescript
// User types "/mod" → matches:
// - /model (name match)
// - /mode  (name match)
// Score: name match > description match > group match
// Uses simple substring scoring, no external library needed
```

### Visual

```
┌────────────────────────────────────┐
│ Session                            │
│   /clear     Clear conversation    │
│   /compact   Compact to reduce..   │
│   /cost      Show token usage      │
│ Model                              │
│   ▶ /model   Switch AI model       │  ← highlighted
│ Debug                              │
│   /doctor    Check system health   │
└────────────────────────────────────┘
```

---

## FileAttachBar.vue

**Where**: Above `InputComposer` textarea, visible when files are attached.

### Drag-Drop Behavior

1. User drags file onto InputComposer area
2. Drop zone highlights (dashed blue border, "Drop files to attach")
3. File dropped → appears as a pill in `FileAttachBar`
4. Multiple files supported
5. Click X on pill to remove

### File Pill

```
┌──────────────────────────────────────────┐
│ 📎 main.rs ✕  │  📎 config.json ✕       │
└──────────────────────────────────────────┘
```

### How Files Are Sent

Attached files are read client-side (FileReader API) and sent as extra context:

```typescript
// When user sends a message with attachments:
ws.send({
  type: 'send_message',
  session_id: '...',
  text: 'explain this code',
  attachments: [
    { name: 'main.rs', content: '...file content...' }
  ]
})
```

Backend injects attachment content into the user message as context before sending to the API.

---

## ModelSelector.vue

**Where**: Dropdown triggered by `Ctrl+K` or clicking model name in StatusBar.

### Available Models

Fetched from `GET /api/status` which returns available providers and models:

```json
{
  "models": [
    { "id": "claude-sonnet-4-20250514", "provider": "anthropic", "active": true },
    { "id": "claude-opus-4-20250514", "provider": "anthropic" },
    { "id": "grok-3", "provider": "xai" },
    { "id": "gpt-4o", "provider": "openai-compat" }
  ]
}
```

### Visual

```
┌────────────────────────────────────┐
│ Switch Model                       │
│                                    │
│ Anthropic                          │
│   ✓ claude-sonnet-4  (active)      │
│     claude-opus-4                  │
│ xAI                                │
│     grok-3                         │
│ OpenAI Compatible                  │
│     gpt-4o                         │
└────────────────────────────────────┘
```

Selecting sends `{"type": "switch_model", "model": "..."}` via WebSocket.

---

## useKeyboardShortcuts.ts

### Shortcut Map

| Shortcut | Action | Context |
|---|---|---|
| `Ctrl+Enter` | Send message | When InputComposer focused |
| `Escape` | Cancel current turn | When streaming |
| `Ctrl+N` | New session | Global |
| `Ctrl+K` | Open model selector | Global |
| `Ctrl+/` | Focus InputComposer | Global |
| `Ctrl+B` | Toggle sidebar | Global |
| `Ctrl+Shift+E` | Toggle file tree | Global |
| `Ctrl+L` | Clear chat display | Global (doesn't delete messages) |
| `Ctrl+Shift+P` | Command palette | Global (future) |

### Implementation

```typescript
import { onKeyStroke } from '@vueuse/core'

export function useKeyboardShortcuts() {
  const sessionStore = useSessionStore()
  const uiStore = useUiStore()

  // Ctrl+Enter to send
  onKeyStroke('Enter', (e) => {
    if (e.ctrlKey) {
      e.preventDefault()
      sessionStore.sendPendingMessage()
    }
  })

  // Escape to cancel
  onKeyStroke('Escape', () => {
    if (sessionStore.isStreaming) {
      sessionStore.cancelTurn()
    }
  })

  // ... etc
}
```

Activated in `App.vue` so shortcuts work globally.

---

*Back: [README](README.md)*
