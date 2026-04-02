# Claw Code — Documentation

Welcome to the Claw Code docs. Claw Code is a high-performance, open-source reimplementation of the Claude Code CLI agent harness, written in Rust.

## Contents

| Document | Description |
|---|---|
| [Getting Started](getting-started.md) | Build, install, and run your first session |
| [Examples & Use Cases](examples.md) | Practical examples, recipes, and what you can build |
| [Architecture](architecture.md) | Crate layout, conversation loop, and design decisions |
| [Tools Reference](tools.md) | All built-in tools the agent can call |
| [Slash Commands](slash-commands.md) | All `/commands` available in the REPL |
| [Configuration](configuration.md) | `.claude.json`, `CLAUDE.md`, permission modes |
| [MCP Integration](mcp.md) | Connecting external MCP servers |
| **UI Implementation Guides** | |
| [TUI Guide](guide-tui.md) | Full-screen terminal UI with ratatui |
| [Web UI Guide](guide-web-ui.md) | Browser-based UI with Axum + SSE |
| [Tauri Desktop Guide](guide-tauri-desktop.md) | Native desktop app with Tauri |

## Quick Reference

```bash
# Build
cd rust && cargo build --release

# Interactive REPL
./target/release/claw

# One-shot prompt
./target/release/claw prompt "explain this codebase"

# Set API key
export ANTHROPIC_API_KEY="sk-ant-..."

# OAuth login
claw login
```

## Project Layout

```
.
├── rust/               # Active Rust workspace (primary implementation)
│   └── crates/
│       ├── rusty-claude-cli/   # Main binary
│       ├── runtime/            # Session, config, permissions, MCP
│       ├── api/                # Anthropic + OpenAI-compat HTTP client
│       ├── tools/              # 20 built-in tools
│       ├── commands/           # Slash command registry
│       ├── plugins/            # Plugin manager (in progress)
│       └── compat-harness/     # TypeScript parity extraction
├── src/                # Python parity audit workspace
├── tests/              # Python verification tests
└── docs/               # This documentation
```
