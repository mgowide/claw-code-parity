# Phase 3 — Session Management

> **Goal**: Multiple sessions, history, resume, search, export. Never lose a conversation.

---

## What This Phase Delivers

- Session list in the sidebar showing all past conversations
- Click any session to resume exactly where you left off
- Full-text search across all transcripts
- Export sessions as Markdown or JSON
- Delete sessions with confirmation

## Prerequisites

- Phase 1 complete (single session chat working)

## Steps

| # | Task | Detail Doc |
|---|---|---|
| 3.1 | Backend session CRUD routes | [api.md](api.md) |
| 3.2 | `SessionsView.vue` + `SessionList.vue` | [views.md](views.md) |
| 3.3 | Resume, full-text search, export | [views.md](views.md) |

## Acceptance Criteria

- [ ] `GET /api/sessions` returns list of all sessions with metadata
- [ ] `GET /api/sessions/:id` returns full transcript
- [ ] `DELETE /api/sessions/:id` removes session from disk
- [ ] `GET /api/sessions/:id/export?format=md` returns Markdown
- [ ] Sidebar shows recent sessions with name, date, token count
- [ ] Clicking a session resumes it (loads messages, reconnects runtime)
- [ ] Search box filters sessions by content
- [ ] Export downloads a file
- [ ] Delete shows confirmation dialog

## Files Created

```
# Backend
src/routes/session.rs        # extended with GET, DELETE, export
src/persistence.rs           # session save/load from disk

# Frontend
src/views/SessionsView.vue
src/components/sidebar/SessionList.vue
```

---

*Detail docs: [api.md](api.md) | [views.md](views.md)*
