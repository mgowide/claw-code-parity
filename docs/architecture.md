# Architecture

## Overview

Claw Code is structured as a Cargo workspace inside `rust/`. The main binary is `claw`, built from the `rusty-claude-cli` crate. All other crates are libraries consumed by it.

```
rust/crates/
├── rusty-claude-cli/   # Binary entry point
├── runtime/            # Core agent runtime
├── api/                # HTTP client layer
├── tools/              # Built-in tool implementations
├── commands/           # Slash command registry
├── plugins/            # Plugin manager
└── compat-harness/     # TypeScript parity extractor
```

---

## Crate Responsibilities

### `rusty-claude-cli`

The main binary (`claw`). Owns:

- Argument parsing and subcommand dispatch
- The interactive REPL loop (`LiveCli` struct in `main.rs`)
- Markdown rendering pipeline (`render.rs`) — heading, table, code block, syntax highlighting via `syntect`
- Readline input with tab completion (`input.rs`) — slash commands, model aliases, session IDs
- Slash command handlers (all `/...` commands)
- Streaming output display — SSE events rendered character-by-character with braille spinner
- Permission prompting — interactive Y/N approval dialogs
- Session management UI — list, switch, resume

Key dependencies: `crossterm`, `rustyline`, `pulldown-cmark`, `syntect`, `tokio`

### `runtime`

The agent's brain. Contains:

- **`conversation.rs`** — `ConversationRuntime`: the core agentic loop
- **`config.rs`** — `ConfigLoader`: merges user/project/local `.claude.json` files; parses hooks, MCP servers, permission rules, plugins
- **`prompt.rs`** — `SystemPromptBuilder`: assembles the system prompt from `CLAUDE.md` files, git context, OS info, tooling directives
- **`session.rs`** — `Session`: in-memory and persisted JSONL conversation history
- **`compact.rs`** — Auto-compaction: summarises old history when input tokens exceed ~100K
- **`permissions.rs`** — `PermissionPolicy`, `PermissionContext`: evaluates per-tool access against the current mode and configured allow/deny rules
- **`hooks.rs`** — Hook config structures (`PreToolUse`, `PostToolUse`) — loaded but not yet executed
- **`mcp.rs`**, **`mcp_client.rs`**, **`mcp_stdio.rs`** — MCP server lifecycle management, JSON-RPC tool calls over stdio/SSE/HTTP/WebSocket
- **`oauth.rs`** — PKCE OAuth 2.0 flow, credential persistence
- **`sandbox.rs`** — Linux namespace / network / filesystem isolation detection and command construction
- **`usage.rs`** — Token usage tracking, cost estimation, per-model pricing

### `api`

Thin async HTTP client layer:

- **`providers/anthropic.rs`** — Anthropic Messages API (non-stream + SSE stream)
- **`providers/openai_compat.rs`** — OpenAI-compatible API (xAI, OpenAI, any proxy)
- **`client.rs`** — `ProviderClient` enum that dispatches to the correct provider based on the model name
- **`sse.rs`** — Incremental SSE parser for streaming responses
- **`prompt_cache.rs`** — Prompt cache hit/miss tracking and `cache_control` injection

Provider detection is automatic from the model name:
- `claude-*` → Anthropic
- `grok-*` → xAI
- `gpt-*` → OpenAI

### `tools`

A `GlobalToolRegistry` that holds all built-in tool `ToolSpec`s plus dynamically-loaded plugin tools. Responsible for:

- Returning `ToolDefinition` JSON schemas sent to the API
- Returning `(tool_name, PermissionMode)` pairs for the permission system
- Dispatching `execute(name, input_json)` calls to the correct implementation
- Tool name normalization (e.g. `read` → `read_file`, `bash-exec` → `bash`)

All 20 built-in tools are defined in `lib.rs` with their JSON schemas and permission tiers. Implementations delegate to `runtime` file ops and bash execution via the `runtime` crate.

### `commands`

Purely a data/metadata crate. Defines `SlashCommandSpec` structs for all `/commands` and provides helpers for:

- Rendering the `/help` table
- Validating slash command input tokens
- Checking if a command supports resume (safe to run in a resumed session without a new model call)
- Dispatching `/agents`, `/plugins`, `/skills` commands that require logic beyond a simple spec

### `plugins`

Plugin manager responsible for loading external tool providers. Currently handles:
- Plugin config from `.claude.json`
- `PluginTool` trait for tools contributed by plugins
- Conflict detection (plugin tool name must not shadow a built-in)

Full plugin install/lifecycle management is planned and not yet implemented.

### `compat-harness`

A development utility crate. When the original TypeScript source is available alongside the repo (detected via `CLAUDE_CODE_UPSTREAM` env var or sibling directory search), it can:

- Parse `commands.ts` to extract the upstream command manifest
- Parse `tools.ts` to extract the upstream tool manifest
- Parse `cli.tsx` to reconstruct the bootstrap plan

This powers the `claw --dump-manifests` and `claw --bootstrap-plan` debug flags, and feeds the `PARITY.md` gap analysis.

---

## Conversation Loop

The core loop in `runtime/src/conversation.rs`:

```
┌──────────────────────────────────────────────────────────────┐
│  ConversationRuntime::run_turn(user_message)                  │
│                                                               │
│  1. Append user message to session                            │
│  2. Build ApiRequest (system prompt + messages)               │
│  3. Stream API response                                       │
│     ├── TextDelta  → emit to caller                           │
│     ├── ToolUse    → validate + execute (steps 4-6)           │
│     ├── Usage      → update UsageTracker                      │
│     └── MessageStop → break                                   │
│  4. Check PermissionPolicy for tool                           │
│     ├── Allowed    → proceed                                  │
│     ├── Ask        → prompt user via PermissionPrompter       │
│     └── Denied     → return ToolError                         │
│  5. Execute tool via ToolExecutor                             │
│  6. Append tool_result to session                             │
│  7. Auto-compact if token threshold exceeded                  │
│  8. Loop back to step 2 (tool_use triggers a new model call)  │
│                                                               │
│  Iteration limit: usize::MAX (effectively unlimited)          │
└──────────────────────────────────────────────────────────────┘
```

---

## System Prompt Assembly

`SystemPromptBuilder` in `runtime/src/prompt.rs` composes the system prompt from:

1. Core role and capability description
2. OS name and version
3. Current date
4. Git repository status and diff (when available)
5. `CLAUDE.md` files discovered by walking up from `cwd` (capped at 4,000 chars each, 12,000 total)
6. Hook configuration visibility (what the user has configured)
7. Permission mode directives
8. Configured tool restrictions
9. Output style overrides

The boundary marker `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__` splits static context (cached) from dynamic context (not cached) to maximise Anthropic prompt cache hits.

---

## Session Compaction

When `estimate_session_tokens(session) >= 100_000` input tokens (configurable via `CLAUDE_CODE_AUTO_COMPACT_INPUT_TOKENS` env var), the runtime automatically:

1. Asks the model to produce an XML `<summary>` of the conversation so far
2. Replaces older messages with the formatted summary
3. Preserves the most recent 4 messages verbatim
4. Continues the conversation transparently

Manual compaction: `/compact`

---

## Permission System

Three permission tiers, ordered from most to least restrictive:

```
ReadOnly < WorkspaceWrite < DangerFullAccess
```

Each tool declares a `required_permission`. A tool call is allowed if:

```
current_mode >= tool.required_permission
AND NOT in deny list
AND (if in ask list) user approved interactively
```

Default mode is `danger-full-access`. Override with `--permission-mode` or `/permissions`.

---

## MCP Integration

MCP (Model Context Protocol) servers extend the tool catalog at runtime. Supported transports:

| Transport | Use case |
|---|---|
| `stdio` | Local process (most common) |
| `sse` | Remote HTTP server-sent events |
| `http` | Remote HTTP streamable |
| `websocket` | Remote WebSocket |
| `sdk` | Managed SDK proxy |
| `managed-proxy` | Anthropic-managed CCR proxy |

When any MCP server is configured, the runtime spawns/connects it on startup, lists its tools (JSON-RPC `tools/list`), and exposes them to the model with a `mcp__<server>__<tool>` name prefix. Tool calls are forwarded via `tools/call`.

---

## Telemetry

The `telemetry` crate provides a `SessionTracer` interface used by `ConversationRuntime` for structured event tracing. By default this is a no-op. Implementations can be injected via the `ApiClient` trait boundary.
