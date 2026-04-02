# Phase 4 — Composer & Shortcuts

> **Goal**: Power-user input experience — slash commands, file attach, keyboard-driven workflow.

---

## What This Phase Delivers

- Type `/` → fuzzy autocomplete dropdown with all 25+ commands
- Drag files onto input → file pills appear as context attachments
- `Ctrl+K` → model selector dropdown for mid-conversation switching
- Full keyboard shortcut system for all major actions

## Prerequisites

- Phase 1 complete (InputComposer exists)
- Phase 3 recommended (session switching via `Ctrl+N`)

## Steps

| # | Task | Detail Doc |
|---|---|---|
| 4.1 | `SlashCommandMenu.vue` — autocomplete | [components.md](components.md) |
| 4.2 | `FileAttachBar.vue` — drag-drop | [components.md](components.md) |
| 4.3 | `ModelSelector.vue` — dropdown | [components.md](components.md) |
| 4.4 | `useKeyboardShortcuts.ts` | [components.md](components.md) |

## Acceptance Criteria

- [ ] Type `/` in InputComposer → dropdown appears with all commands
- [ ] Fuzzy search: `/mod` matches `/model`, `/mode`
- [ ] Arrow keys navigate, Enter selects, Escape dismisses
- [ ] Drag file onto input → file pill appears, X to remove
- [ ] `Ctrl+K` opens model selector with available models
- [ ] All shortcuts work: Ctrl+Enter, Ctrl+N, Ctrl+B, Escape, etc.

## Files Created

```
src/components/composer/SlashCommandMenu.vue
src/components/composer/FileAttachBar.vue
src/components/composer/ModelSelector.vue
src/composables/useKeyboardShortcuts.ts
```

---

*Detail docs: [components.md](components.md)*
