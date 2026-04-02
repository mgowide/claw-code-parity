# Phase 3 — Session Views

> Frontend components for session listing, switching, search, and export.

---

## SessionsView.vue

**Route**: `/sessions` — full-page session browser.

### Layout

```
┌─────────────────────────────────────────────────────┐
│  Sessions                              🔍 Search... │
│                                                      │
│  ┌────────────────────────────────────┐             │
│  │ 📝 Refactor auth module            │ Apr 2, '26  │
│  │ claude-sonnet-4 │ 24 msgs │ $0.04  │ [Export][🗑] │
│  ├────────────────────────────────────┤             │
│  │ 📝 Setup database migrations       │ Apr 1, '26  │
│  │ claude-sonnet-4 │ 18 msgs │ $0.03  │ [Export][🗑] │
│  ├────────────────────────────────────┤             │
│  │ 📝 Debug WebSocket reconnect       │ Mar 31, '26 │
│  │ claude-opus-4   │ 42 msgs │ $0.12  │ [Export][🗑] │
│  └────────────────────────────────────┘             │
│                                                      │
│  Sort by: [Date ▼] [Cost] [Messages] [Model]        │
└─────────────────────────────────────────────────────┘
```

### Features

- **Session cards**: name, model, message count, cost, date
- **Search**: filters by content (searches messages), debounced 300ms
- **Sort**: by date (default), cost, message count, model
- **Actions per session**:
  - Click card → resume (navigates to `/` with session loaded)
  - Export button → downloads Markdown file
  - Delete button → confirmation dialog → `DELETE /api/sessions/:id`

---

## SessionList.vue

**Where**: Sidebar component — condensed version of SessionsView.

### Layout

```
┌──────────────┐
│ + New Chat    │
├──────────────┤
│ 🔍 Search... │
├──────────────┤
│ · Refactor.. │  ← current session (highlighted)
│ · Setup DB.. │
│ · Debug WS.. │
│ · Fix tests. │
│ · Add API..  │
│              │
│ View all →   │
└──────────────┘
```

### Features

- Shows last 10 sessions (truncated names)
- Active session highlighted
- Click to switch (loads session via WebSocket `resume_session` command)
- "New Chat" button creates fresh session
- "View all →" links to `/sessions`
- Search filters inline

---

## Session Resume Flow

```
1. User clicks session in SessionList
2. Frontend sends: {"type": "resume_session", "session_id": "abc-123"}
3. Backend loads session from disk → restores ConversationRuntime state
4. Backend sends: {"type": "connected", "session_id": "abc-123", "model": "..."}
5. Backend sends transcript replay (all previous messages) as a batch
6. Frontend populates sessionStore.messages from replay
7. ChatThread renders full conversation history
8. User can continue the conversation
```

---

## Session Name Generation

Sessions are auto-named from the first user message:
- Take first 50 characters of first user message
- Truncate at last word boundary
- User can rename inline (click session name → editable field)

---

*Back: [README](README.md) | API: [api.md](api.md)*
