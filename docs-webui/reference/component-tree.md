# Vue Component Tree Reference

> Complete hierarchy of all Vue components across all phases.

---

## Full Tree

```
src/
├── App.vue                              # Root: NConfigProvider + theme + router
├── main.ts                              # Entry: createApp, Pinia, Router, global styles
│
├── router/
│   └── index.ts                         # Route definitions (lazy-loaded views)
│
├── views/                               # One per route
│   ├── ChatView.vue                     # /        (Phase 1)
│   ├── SessionsView.vue                 # /sessions (Phase 3)
│   ├── FilesView.vue                    # /files    (Phase 5)
│   ├── ToolsView.vue                    # /tools    (Phase 5)
│   ├── TodosView.vue                    # /todos    (Phase 5)
│   ├── SettingsView.vue                 # /settings (Phase 5)
│   ├── CostView.vue                     # /cost     (Phase 5)
│   └── MCPView.vue                      # /mcp      (Phase 5)
│
├── components/
│   ├── layout/                          # App structure
│   │   ├── AppShell.vue                 # Sidebar + main + right panel grid  (Phase 1)
│   │   └── StatusBar.vue                # Bottom bar: model, tokens, cost    (Phase 1)
│   │
│   ├── chat/                            # Conversation UI
│   │   ├── ChatThread.vue               # Scrollable message list            (Phase 1)
│   │   ├── MessageBubble.vue            # Single message (user/assistant)    (Phase 1)
│   │   ├── StreamingText.vue            # Token-by-token text + cursor       (Phase 1)
│   │   ├── ToolCallCard.vue             # Collapsible tool execution card    (Phase 2)
│   │   ├── DiffViewer.vue               # Inline diff (v-code-diff)          (Phase 2)
│   │   ├── CodeBlock.vue                # Syntax-highlighted code            (Phase 2)
│   │   ├── TerminalPanel.vue            # xterm.js for bash output           (Phase 2)
│   │   ├── PermissionModal.vue          # Approve/deny permission            (Phase 2)
│   │   └── ThinkingIndicator.vue        # Animated thinking state            (Phase 2)
│   │
│   ├── composer/                        # Input area
│   │   ├── InputComposer.vue            # Multi-line textarea + send         (Phase 1)
│   │   ├── SlashCommandMenu.vue         # /command autocomplete dropdown     (Phase 4)
│   │   ├── FileAttachBar.vue            # Drag+drop file attachment          (Phase 4)
│   │   └── ModelSelector.vue            # Quick model switch                 (Phase 4)
│   │
│   ├── sidebar/                         # Left sidebar
│   │   ├── AppSidebar.vue               # Navigation + session list          (Phase 1 stub, Phase 3 full)
│   │   ├── SessionList.vue              # Recent sessions                    (Phase 3)
│   │   ├── FileTree.vue                 # Workspace file explorer            (Phase 5)
│   │   └── ToolList.vue                 # Available tools quick view         (Phase 5)
│   │
│   ├── panels/                          # Right panel
│   │   ├── RightPanel.vue               # Tabbed container                   (Phase 5)
│   │   ├── TodoBoard.vue                # Kanban from TodoWrite              (Phase 5)
│   │   ├── TokenCounter.vue             # Live token usage gauge             (Phase 5)
│   │   └── CostTracker.vue              # Running cost + per-turn            (Phase 5)
│   │
│   ├── transitions/                     # Animation wrappers
│   │   ├── PageTransition.vue           # Router view transitions            (Phase 6)
│   │   └── ListTransition.vue           # List item enter/leave              (Phase 6)
│   │
│   ├── visuals/                         # Three.js + decorative
│   │   ├── SplashScreen.vue             # 3D particle intro                  (Phase 6)
│   │   ├── AmbientParticles.vue         # Background particle field          (Phase 6)
│   │   └── TokenFlow.vue                # Token flow visualization           (Phase 6)
│   │
│   ├── ErrorBoundary.vue                # Component-level error catch        (Phase 6)
│   └── ToastContainer.vue               # Toast notification stack           (Phase 6)
│
├── composables/                         # Reusable logic
│   ├── useWebSocket.ts                  # WS connection + auto-reconnect     (Phase 1)
│   ├── useStream.ts                     # Process ServerEvents → store       (Phase 1)
│   ├── useSession.ts                    # Active session management          (Phase 3)
│   ├── useKeyboardShortcuts.ts          # Global hotkeys                     (Phase 4)
│   ├── useTheme.ts                      # Theme switching + persistence      (Phase 6)
│   ├── useToast.ts                      # Toast notifications                (Phase 6)
│   └── useServiceWorker.ts              # PWA registration                   (Phase 6)
│
├── stores/                              # Pinia state
│   ├── sessionStore.ts                  # Messages, tool calls, tokens       (Phase 1)
│   ├── uiStore.ts                       # Panel visibility, theme, layout    (Phase 5)
│   └── configStore.ts                   # Runtime config mirror              (Phase 5)
│
├── lib/                                 # Pure utilities
│   ├── ws.ts                            # WebSocket protocol helpers         (Phase 1)
│   ├── api.ts                           # REST API client (typed fetch)      (Phase 3)
│   ├── markdown.ts                      # Markdown → HTML pipeline           (Phase 2)
│   └── diff.ts                          # Diff parsing utilities             (Phase 2)
│
├── types/                               # TypeScript definitions
│   ├── events.ts                        # ServerEvent + ClientCommand        (Phase 1)
│   ├── session.ts                       # Session, Message, ToolCall         (Phase 1)
│   └── config.ts                        # Config types                       (Phase 5)
│
└── styles/
    ├── main.css                         # Tailwind import + base styles      (Phase 1)
    └── themes/
        ├── dark.css                     # CSS custom properties              (Phase 6)
        ├── light.css                    #                                    (Phase 6)
        ├── solarized.css                #                                    (Phase 6)
        └── high-contrast.css            #                                    (Phase 6)
```

---

## Component Count by Phase

| Phase | Components | Composables | Stores | Views | Total |
|---|---|---|---|---|---|
| 1 — Core Chat | 6 | 2 | 1 | 1 | 10 |
| 2 — Tool Viz | 5 | 0 | 0 | 0 | 5 |
| 3 — Sessions | 2 | 1 | 0 | 1 | 4 |
| 4 — Composer | 3 | 1 | 0 | 0 | 4 |
| 5 — Dashboards | 6 | 0 | 2 | 5 | 13 |
| 6 — Polish | 7 | 3 | 0 | 0 | 10 |
| **Total** | **29** | **7** | **3** | **7** | **46** |

---

*Back: [docs-webui/README.md](../README.md)*
