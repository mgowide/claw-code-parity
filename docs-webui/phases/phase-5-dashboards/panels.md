# Phase 5 — Panel Components

> Right panel, token counter, cost tracker, and todo board.

---

## RightPanel.vue

**Where**: Right side of AppShell, collapsible. Contains tabbed sub-panels.

### Tabs

| Tab | Component | Data Source |
|---|---|---|
| **Context** | File list + token count per file | Session context tracking |
| **Todos** | TodoBoard kanban | `TodoWrite` tool events |
| **Tokens** | TokenCounter gauge | `usage` WebSocket events |
| **Cost** | CostTracker summary | Accumulated usage data |

### Layout

```
┌──────────────┐
│ [Context][Todos][Tokens][Cost] │  ← tab bar
├──────────────┤
│              │
│  (active     │
│   tab        │
│   content)   │
│              │
└──────────────┘
```

- Drag left edge to resize width (min 200px, max 500px)
- `Ctrl+Shift+R` toggles visibility
- Width persisted via `useStorage`

---

## TokenCounter.vue

**Where**: Inside RightPanel "Tokens" tab and condensed in StatusBar.

### Display

```
┌──────────────────────────────┐
│  Token Usage                 │
│                              │
│  Input    ████████░░  8,400  │  ← blue bar
│  Output   ███░░░░░░░  3,200  │  ← green bar
│  Cache    ██████░░░░  5,600  │  ← yellow bar
│                              │
│  Context: 17,200 / 200,000   │  ← usage percentage
│  ██████████░░░░░░░░░░  8.6%  │
│                              │
│  This Turn: +1,200 in / +340 │
└──────────────────────────────┘
```

### Props & Data

```typescript
interface TokenCounterProps {
  inputTokens: number
  outputTokens: number
  cacheHits: number
  contextLimit: number  // e.g., 200000 for Claude
  turnInputDelta: number
  turnOutputDelta: number
}
```

Updates live as `usage` events arrive over WebSocket.

---

## CostTracker.vue

**Where**: Inside RightPanel "Cost" tab and condensed in StatusBar.

### Display

```
┌──────────────────────────────┐
│  Session Cost                │
│                              │
│  Total: $0.042               │
│                              │
│  Turn 1:  $0.008             │
│  Turn 2:  $0.012             │
│  Turn 3:  $0.015   (current) │
│  Turn 4:  $0.007             │
│                              │
│  Input:   $0.025             │
│  Output:  $0.017             │
│  Cache savings: -$0.009      │
└──────────────────────────────┘
```

Tracks cost per turn, breaks down by input/output, shows cache savings.

---

## TodoBoard.vue

**Where**: Inside RightPanel "Todos" tab and as standalone page at `/todos`.

### Kanban Layout

```
┌──────────────────────────────────────────┐
│  Pending      In Progress    Completed   │
│  ┌────────┐   ┌────────┐   ┌────────┐  │
│  │ Task 1 │   │ Task 3 │   │ Task 2 │  │
│  │        │   │        │   │        │  │
│  └────────┘   └────────┘   └────────┘  │
│  ┌────────┐                 ┌────────┐  │
│  │ Task 4 │                 │ Task 5 │  │
│  └────────┘                 └────────┘  │
└──────────────────────────────────────────┘
```

### Data Source

The `TodoWrite` tool emits todo items via tool results. The backend sends these as part of `tool_result`:

```json
{
  "type": "tool_result",
  "id": "t5",
  "output": "{\"todos\": [{\"id\": 1, \"title\": \"Fix auth\", \"status\": \"in-progress\"}]}",
  "is_error": false
}
```

Frontend parses tool output when `name === "TodoWrite"` and updates todo state.

### Todo Item

```typescript
interface TodoItem {
  id: number
  title: string
  status: 'pending' | 'in_progress' | 'completed'
}
```

---

*Back: [README](README.md) | Views: [views.md](views.md) | API: [api.md](api.md)*
