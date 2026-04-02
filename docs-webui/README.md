# Claw Code Web UI — Documentation Index

> Complete documentation for building the Claw Code browser-based agent interface.

---

## 📁 Folder Structure

```
docs-webui/
├── README.md                    ← You are here
├── PROJECT-OVERVIEW.md          # What, why, and how — full project summary
├── PLAN.md                      # Technical plan — components, protocols, APIs
├── STACK-REPORT.md              # Dependency audit — every crate assessed
│
├── phases/                      # Phase-by-phase implementation guides
│   ├── README.md                # Phase roadmap — timeline & dependencies
│   ├── phase-1-core-chat/
│   │   ├── README.md            # Phase 1 overview — goals, deliverables, checklist
│   │   ├── backend.md           # Axum server, WebSocket, session handler
│   │   └── frontend.md          # Vue 3 shell, chat components, composables
│   ├── phase-2-tool-viz/
│   │   ├── README.md            # Phase 2 overview
│   │   ├── components.md        # ToolCallCard, DiffViewer, CodeBlock, Terminal
│   │   └── events.md            # WebSocket event types for tools
│   ├── phase-3-sessions/
│   │   ├── README.md            # Phase 3 overview
│   │   ├── api.md               # Session CRUD routes
│   │   └── views.md             # SessionsView, SessionList, search, export
│   ├── phase-4-composer/
│   │   ├── README.md            # Phase 4 overview
│   │   └── components.md        # SlashMenu, FileAttach, ModelSelector, shortcuts
│   ├── phase-5-dashboards/
│   │   ├── README.md            # Phase 5 overview
│   │   ├── panels.md            # RightPanel, TokenCounter, CostTracker, TodoBoard
│   │   ├── views.md             # Files, Tools, Settings, Cost, MCP views
│   │   └── api.md               # Backend routes for files, config, tools, MCP
│   └── phase-6-polish/
│       ├── README.md            # Phase 6 overview
│       ├── theming.md           # Theme system, CSS variables, dark/light
│       ├── animations.md        # @vueuse/motion, Lottie, Three.js strategy
│       └── pwa.md               # Service worker, offline, responsive, a11y
│
└── reference/
    ├── ws-protocol.md           # Full WebSocket protocol spec
    ├── rest-api.md              # Complete REST API surface
    └── component-tree.md        # Vue component hierarchy map
```

---

## Quick Links

| Document | Description |
|---|---|
| [Project Overview](PROJECT-OVERVIEW.md) | What we're building, why, architecture, design principles |
| [Technical Plan](PLAN.md) | Component tree, WebSocket protocol, API routes, npm deps |
| [Stack Report](STACK-REPORT.md) | Every Rust dependency assessed for risk and reliability |
| **Phase Guides** | |
| [Phase Roadmap](phases/README.md) | Timeline, dependency graph, milestone checklist |
| [Phase 1 — Core Chat](phases/phase-1-core-chat/README.md) | Axum + Vue 3 + streaming chat end-to-end |
| [Phase 2 — Tool Viz](phases/phase-2-tool-viz/README.md) | ToolCallCard, DiffViewer, Terminal, Permissions |
| [Phase 3 — Sessions](phases/phase-3-sessions/README.md) | Multi-session CRUD, resume, search, export |
| [Phase 4 — Composer](phases/phase-4-composer/README.md) | Slash commands, file attach, model selector, hotkeys |
| [Phase 5 — Dashboards](phases/phase-5-dashboards/README.md) | Panels, file browser, settings, cost analytics, MCP |
| [Phase 6 — Polish](phases/phase-6-polish/README.md) | Themes, animations, Three.js, PWA, responsive |
| **Reference** | |
| [WebSocket Protocol](reference/ws-protocol.md) | Server→Client events, Client→Server commands |
| [REST API](reference/rest-api.md) | All HTTP endpoints with request/response schemas |
| [Component Tree](reference/component-tree.md) | Full Vue component hierarchy |

---

## Reading Order

1. **[PROJECT-OVERVIEW.md](PROJECT-OVERVIEW.md)** — Start here. Understand what and why.
2. **[STACK-REPORT.md](STACK-REPORT.md)** — Know what we depend on.
3. **[phases/README.md](phases/README.md)** — See the full roadmap.
4. **Phase 1 docs** — Then follow phases in order as we build.

---

*Last updated: April 2, 2026*
