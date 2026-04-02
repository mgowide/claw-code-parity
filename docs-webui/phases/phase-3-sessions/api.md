# Phase 3 — Session API Routes

> Backend routes for session CRUD, persistence, and export.

---

## Routes

### `GET /api/sessions`

List all sessions.

**Response**:
```json
{
  "sessions": [
    {
      "id": "abc-123",
      "name": "Refactor auth module",
      "created_at": "2026-04-02T10:30:00Z",
      "updated_at": "2026-04-02T11:15:00Z",
      "message_count": 24,
      "input_tokens": 8400,
      "output_tokens": 3200,
      "cost": 0.042,
      "model": "claude-sonnet-4-20250514",
      "is_active": false
    }
  ]
}
```

### `GET /api/sessions/:id`

Get full session transcript.

**Response**:
```json
{
  "id": "abc-123",
  "name": "Refactor auth module",
  "model": "claude-sonnet-4-20250514",
  "created_at": "2026-04-02T10:30:00Z",
  "messages": [
    {
      "id": "m1",
      "role": "user",
      "content": "refactor the auth module to use JWT",
      "timestamp": "2026-04-02T10:30:05Z"
    },
    {
      "id": "m2",
      "role": "assistant",
      "content": "I'll refactor the auth module...",
      "timestamp": "2026-04-02T10:30:08Z",
      "tool_calls": [
        { "id": "t1", "name": "read_file", "input": {"path": "src/auth.rs"}, "output": "...", "is_error": false }
      ]
    }
  ],
  "usage": {
    "input_tokens": 8400,
    "output_tokens": 3200,
    "cache_hits": 5600,
    "cost": 0.042
  }
}
```

### `DELETE /api/sessions/:id`

Delete session from disk. Returns `204 No Content`.

### `GET /api/sessions/:id/export`

Export session transcript.

**Query params**:
- `format=md` — Markdown
- `format=json` — JSON (same as GET but downloadable)

**Markdown output example**:
```markdown
# Session: Refactor auth module
**Model**: claude-sonnet-4-20250514 | **Date**: April 2, 2026 | **Cost**: $0.042

---

## User
refactor the auth module to use JWT

## Assistant
I'll refactor the auth module...

> 🔧 **read_file** `src/auth.rs`
> (file content)
```

---

## Session Persistence

### Storage Format

Sessions saved as JSON files in `~/.claw/sessions/`:

```
~/.claw/sessions/
├── abc-123.json
├── def-456.json
└── index.json    # lightweight index for fast listing
```

### `index.json`

```json
{
  "sessions": [
    { "id": "abc-123", "name": "...", "created_at": "...", "updated_at": "...", "message_count": 24, "cost": 0.042 }
  ]
}
```

Full transcript is only loaded when `GET /api/sessions/:id` is called.

### Auto-save

Sessions auto-save to disk:
- After each turn completes
- On WebSocket disconnect
- On server shutdown (graceful)

---

*Back: [README](README.md) | Views: [views.md](views.md)*
