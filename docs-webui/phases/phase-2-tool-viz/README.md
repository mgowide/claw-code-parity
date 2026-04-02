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

- [ ] Tool calls render as cards in the chat thread (not raw JSON)
- [ ] Cards show spinner while tool runs, green/red border on complete
- [ ] `edit_file` card auto-expands a colored diff viewer
- [ ] Code blocks have syntax highlighting for 20+ languages
- [ ] Code blocks have one-click copy button
- [ ] `bash` output shows ANSI colors correctly in xterm widget
- [ ] Permission modal blocks interaction until approved/denied
- [ ] "Always allow" checkbox remembers preference for the session

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
