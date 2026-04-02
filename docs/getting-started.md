# Getting Started

## Prerequisites

- **Rust** 1.75 or later (`rustup` is the recommended installer)
- An **Anthropic API key**, or one of the supported OpenAI-compatible providers

## Build

```bash
git clone https://github.com/instructkr/claw-code
cd claw-code/rust
cargo build --release
```

The binary is placed at `rust/target/release/claw`.

Optionally, add it to your PATH:

```bash
# Linux / macOS
cp target/release/claw ~/.local/bin/

# Or symlink
ln -s "$(pwd)/target/release/claw" ~/.local/bin/claw
```

## Authentication

### API Key (direct)

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

Set this in your shell profile (`.bashrc`, `.zshrc`, etc.) to make it permanent.

### OAuth Login

```bash
claw login
```

This opens a browser window for Anthropic's OAuth flow using PKCE. Credentials are stored locally. Log out with `claw logout`.

### OpenAI-compatible Proxy

```bash
export OPENAI_API_KEY="..."
export OPENAI_BASE_URL="https://your-proxy.com/v1"
```

Use a model name that matches the provider (e.g. `gpt-4o`, `grok-3`). See [Configuration](configuration.md#provider-selection) for details.

## First Run

### Interactive REPL

```bash
claw
```

You'll see the `claw>` prompt. Type any natural language request. The agent will plan, call tools, and respond.

### One-Shot Prompt

```bash
claw prompt "summarize the project for me"
```

Useful for scripting or CI pipelines. Add `--output-format json` to get structured JSON output.

### With a Specific Model

```bash
claw --model sonnet prompt "review the code in main.rs"
```

Model aliases: `opus`, `sonnet`, `haiku` (see [Configuration](configuration.md#model-aliases)).

## Initialize a Project

Run this in any repo to create a `CLAUDE.md` that the agent reads as project-aware context:

```bash
claw init
```

Or from within the REPL:

```
/init
```

## Verify Your Setup

```bash
claw doctor
```

This checks:
- API key / OAuth credentials
- Shell environment
- MCP server connectivity (if configured)
- Sandbox support

## Permission Modes

All tool calls require a permission level. Three modes are available:

| Mode | Allowed operations |
|---|---|
| `read-only` | Read files, search, web fetch — no writes |
| `workspace-write` | Read + write/edit files within the working directory |
| `danger-full-access` | Full access including bash execution (default) |

Set the mode at launch:

```bash
claw --permission-mode read-only
```

Or switch mid-session in the REPL:

```
/permissions read-only
```

## Session Persistence

Sessions are saved automatically as `.jsonl` files. Resume a previous session:

```bash
claw --resume latest
```

Or inside the REPL:

```
/resume latest
```

List all saved sessions:

```
/session list
```

## Next Steps

- [Tools Reference](tools.md) — what the agent can do
- [Slash Commands](slash-commands.md) — REPL control commands
- [Configuration](configuration.md) — customize behaviour
- [MCP Integration](mcp.md) — extend with external tool servers
