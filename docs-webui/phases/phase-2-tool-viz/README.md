# Phase 2 — Tool Visualization

> **Goal**: When the AI calls tools (edit file, run bash, search code), the user sees exactly what's happening — not just text.

---

## What This Phase Delivers

- Tool calls appear as **collapsible cards** with spinner → result → success/error state
- `edit_file` results show **colored inline diffs** (side-by-side or unified)
- Code blocks have **syntax highlighting**, language badge, copy button
- `bash`/`PowerShell` output renders in an **embedded terminal** with ANSI colors
- Permission requests appear as **modals** with risk badges and allow/deny buttons

## Prerequisites

- Phase 1 complete (chat working, WebSocket connected, streaming text rendering)

## Steps

| # | Task | Detail Doc |
|---|---|---|
| 2.1 | `ToolCallCard.vue` — collapsible card with states | [components.md](components.md) |
| 2.2 | `DiffViewer.vue` — side-by-side diffs | [components.md](components.md) |
| 2.3 | `CodeBlock.vue` — syntax highlight + copy | [components.md](components.md) |
| 2.4 | `TerminalPanel.vue` — xterm.js embed | [components.md](components.md) |
| 2.5 | `PermissionModal.vue` — approve/deny | [components.md](components.md) |
| 2.6 | Backend tool event handlers | [events.md](events.md) |

## New Dependencies

```bash
npm install v-code-diff @xterm/xterm @xterm/addon-fit highlight.js
```

## Acceptance Criteria

- [x] Tool calls render as cards in the chat thread (not raw JSON)
- [x] Cards show spinner while tool runs, green/red border on complete
- [x] `edit_file` card auto-expands a colored diff viewer
- [x] Code blocks have syntax highlighting for 20+ languages
- [x] Code blocks have one-click copy button
- [x] `bash` output shows ANSI colors correctly in xterm widget
- [x] Permission modal blocks interaction until approved/denied
- [x] "Always allow" checkbox remembers preference for the session

## Implementation Notes

- **Build verified**: `vue-tsc --noEmit` (0 errors), `vite build` (84 modules, 497ms)
- **Backend**: `ws/handler.rs` updated with `simulate_turn()` that demonstrates all tool event types based on keywords in user message ("edit", "bash", "read", "permission", etc.)
- **ChatView chunk** is ~585 KB due to xterm + highlight.js; code-splitting deferred to Phase 6
- **highlight.js** configured with 20 languages: JS, TS, Python, Rust, JSON, Bash, CSS, HTML, Markdown, YAML, SQL, Go, Java, C++, C, Ruby, PHP, Swift, Kotlin, Dockerfile

## Files Created

```
src/components/chat/ToolCallCard.vue
src/components/chat/DiffViewer.vue
src/components/chat/CodeBlock.vue
src/components/chat/TerminalPanel.vue
src/components/chat/PermissionModal.vue
```

---

*Detail docs: [components.md](components.md) | [events.md](events.md)*
