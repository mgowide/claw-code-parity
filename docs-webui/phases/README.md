# Phase Roadmap

> 6 phases, 42 steps — from empty folder to outstanding Web UI.

---

## Dependency Graph

```
Phase 1 (Core Chat)
    └──► Phase 2 (Tool Viz)        ← depends on WebSocket + chat working
    └──► Phase 3 (Sessions)        ← depends on session handler
             └──► Phase 5 (Dashboards)  ← depends on sessions + tool events
    └──► Phase 4 (Composer)        ← depends on InputComposer existing
                                       └──► Phase 6 (Polish)  ← depends on all UI existing
```

Phase 1 is the foundation. Phases 2, 3, and 4 can partially overlap after Phase 1 is done. Phase 5 needs sessions and tools working. Phase 6 is the final pass over everything.

---

## Phase Summary

| Phase | Name | Steps | Backend Files | Frontend Files | Key Deliverable |
|---|---|---|---|---|---|
| **1** | Core Chat | 15 | ~5 | ~10 | Send message → streaming response in browser |
| **2** | Tool Viz | 6 | ~2 | ~6 | Tool calls, diffs, terminal, permissions visible |
| **3** | Sessions | 3 | ~3 | ~3 | Multi-session with resume, search, export |
| **4** | Composer | 4 | 0 | ~5 | Slash commands, file attach, model switch, hotkeys |
| **5** | Dashboards | 8 | ~5 | ~10 | Tokens, cost, files, tools, settings, MCP views |
| **6** | Polish | 6 | ~1 | ~8 | Themes, animations, Three.js, PWA, responsive |
| | **Total** | **42** | **~16** | **~42** | |

---

## Milestone Checklist

### Phase 1 — Core Chat That Works ✅
- [x] `rust/crates/web-server/` crate exists and compiles
- [x] Axum serves on `localhost:3100`
- [x] `/api/status` returns health JSON
- [x] `/ws` accepts WebSocket connections
- [x] `POST /api/sessions` creates a session
- [x] `send_message` → `text_delta` events stream to browser
- [x] `cancel_turn` stops generation
- [x] Vue 3 app loads at `localhost:5173` (dev) or via Axum (prod)
- [x] User can type, send, and see streaming AI response
- [x] Status bar shows model and connection state

### Phase 2 — Tool Visualization
- [ ] Tool calls appear as collapsible cards with spinner → result
- [ ] `edit_file` shows colored inline diff
- [ ] Code blocks have syntax highlighting + copy button
- [ ] `bash` output renders in xterm.js with ANSI colors
- [ ] Permission requests show as modals with risk badges
- [ ] All tool events (start/result/diff/permission) flow through WebSocket

### Phase 3 — Session Management
- [ ] Session list in sidebar shows all past sessions
- [ ] Click session to resume
- [ ] Full-text search across all transcripts
- [ ] Export session as Markdown or JSON
- [ ] Delete session with confirmation

### Phase 4 — Composer & Shortcuts
- [ ] Type `/` → fuzzy autocomplete dropdown appears
- [ ] Drag files onto input → file pills appear
- [ ] `Ctrl+K` → model selector dropdown
- [ ] All keyboard shortcuts active (Ctrl+Enter, Ctrl+N, Escape, etc.)

### Phase 5 — Panels & Dashboards
- [ ] Right panel shows live tokens + cost + todos
- [ ] File browser with Monaco code viewer
- [ ] Tools dashboard with usage stats
- [ ] Settings page edits `.claude.json` visually
- [ ] Cost page with charts (per-session, per-tool, daily)
- [ ] MCP manager: list servers, add, remove, see tools

### Phase 6 — Polish
- [ ] 4 themes working (dark, light, solarized, high-contrast)
- [ ] Animations on message, tool card, page transitions
- [ ] Three.js splash screen + ambient visuals
- [ ] Panels resizable via drag
- [ ] Responsive layout (sidebar collapses on mobile)
- [ ] PWA installable + offline session reading
- [ ] WebSocket auto-reconnect with visual indicator
- [ ] Toast notifications for errors and events

---

## Phase Links

| Phase | Overview | Detail Docs |
|---|---|---|
| 1 | [README](phase-1-core-chat/README.md) | [backend](phase-1-core-chat/backend.md), [frontend](phase-1-core-chat/frontend.md) |
| 2 | [README](phase-2-tool-viz/README.md) | [components](phase-2-tool-viz/components.md), [events](phase-2-tool-viz/events.md) |
| 3 | [README](phase-3-sessions/README.md) | [api](phase-3-sessions/api.md), [views](phase-3-sessions/views.md) |
| 4 | [README](phase-4-composer/README.md) | [components](phase-4-composer/components.md) |
| 5 | [README](phase-5-dashboards/README.md) | [panels](phase-5-dashboards/panels.md), [views](phase-5-dashboards/views.md), [api](phase-5-dashboards/api.md) |
| 6 | [README](phase-6-polish/README.md) | [theming](phase-6-polish/theming.md), [animations](phase-6-polish/animations.md), [pwa](phase-6-polish/pwa.md) |

---

*Last updated: April 2, 2026*
