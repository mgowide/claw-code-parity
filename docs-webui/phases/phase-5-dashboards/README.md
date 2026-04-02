# Phase 5 — Panels & Dashboards

> **Goal**: Full visibility into everything the runtime exposes — files, todos, cost, tokens, config, MCP.

---

## What This Phase Delivers

- **Right panel** with live token counter, cost tracker, and todo board
- **File browser** with Monaco-powered code viewer
- **Tools dashboard** showing all tools with usage stats
- **Settings page** for visual `.claude.json` editing
- **Cost analytics** with charts and export
- **MCP manager** for server management

## Prerequisites

- Phase 1 (WebSocket + chat)
- Phase 2 (tool events for tool dashboard)
- Phase 3 (sessions for cost tracking)

## Steps

| # | Task | Detail Doc |
|---|---|---|
| 5.1 | RightPanel + TokenCounter + CostTracker | [panels.md](panels.md) |
| 5.2 | TodoBoard kanban | [panels.md](panels.md) |
| 5.3 | FilesView + Monaco viewer | [views.md](views.md) |
| 5.4 | ToolsView dashboard | [views.md](views.md) |
| 5.5 | SettingsView config editor | [views.md](views.md) |
| 5.6 | CostView analytics + charts | [views.md](views.md) |
| 5.7 | MCP manager page | [views.md](views.md) |
| 5.8 | Backend routes (files/config/tools/mcp) | [api.md](api.md) |

## New Dependencies

```bash
npm install monaco-editor chart.js vue-chartjs d3
```

## Acceptance Criteria

- [ ] Right panel shows live input/output/cache token counts
- [ ] Right panel shows running cost with per-turn breakdown
- [ ] Todo board shows items from `TodoWrite` tool (Pending/In Progress/Completed)
- [ ] File browser renders workspace tree, click opens Monaco viewer
- [ ] Tools page shows all 20+ tools with usage count and permission level
- [ ] Settings page reads, edits, and saves `.claude.json`
- [ ] Cost page shows per-session chart, per-tool breakdown, daily graph
- [ ] MCP page lists servers, shows health, supports add/remove

## Files Created

```
# Components
src/components/panels/RightPanel.vue
src/components/panels/TokenCounter.vue
src/components/panels/CostTracker.vue
src/components/panels/TodoBoard.vue

# Views
src/views/FilesView.vue
src/views/ToolsView.vue
src/views/SettingsView.vue
src/views/CostView.vue
src/views/MCPView.vue

# Backend
src/routes/files.rs
src/routes/config.rs
src/routes/tools.rs
src/routes/mcp.rs
```

---

*Detail docs: [panels.md](panels.md) | [views.md](views.md) | [api.md](api.md)*
