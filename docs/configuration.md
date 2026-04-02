# Configuration

Claw Code has a three-level configuration hierarchy. Settings are merged in order from lowest to highest priority:

```
User config   (~/.claude.json)
   ↓ merged with
Project config  (<repo>/.claude.json)
   ↓ merged with
Local config    (<repo>/.claude/settings.local.json)
```

Local config always wins. This lets machine-specific overrides stay out of source control.

---

## Config File Format

`.claude.json` is a JSON file. The canonical schema key is `SettingsSchema`.

Example:

```json
{
  "model": "sonnet",
  "permissionMode": "workspace-write",
  "hooks": {
    "preToolUse": ["./scripts/pre-tool.sh"],
    "postToolUse": ["./scripts/post-tool.sh"]
  },
  "mcp": {
    "servers": {
      "filesystem": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-filesystem", "/home/user/projects"]
      }
    }
  },
  "plugins": {
    "enabled": {
      "my-plugin": true
    },
    "externalDirectories": ["/home/user/.claw/plugins"]
  },
  "permissions": {
    "allow": ["bash(git *)", "read_file"],
    "deny": ["bash(rm -rf *)"],
    "ask": ["write_file"]
  }
}
```

---

## Model Aliases

Set a default model in config or pass `--model` at launch:

```json
{ "model": "sonnet" }
```

| Alias | Full model name |
|---|---|
| `opus` | `claude-opus-4-6` |
| `sonnet` | `claude-sonnet-4-6` |
| `haiku` | `claude-haiku-4-5-20251213` |

Full model names are also accepted. Default is `claude-opus-4-6`.

Max tokens are model-dependent: opus models default to `32,000`; all others default to `64,000`.

---

## Provider Selection

The provider is detected automatically from the model name. Override API credentials via environment variables:

### Anthropic

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
# Optional: point to a proxy
export ANTHROPIC_BASE_URL="https://your-proxy.com"
```

### OpenAI-compatible

```bash
export OPENAI_API_KEY="..."
export OPENAI_BASE_URL="https://api.openai.com/v1"
```

Use an OpenAI-compatible model name (e.g. `gpt-4o`, `gpt-4o-mini`).

### xAI (Grok)

```bash
export XAI_API_KEY="..."
```

Use a Grok model name (e.g. `grok-3`).

---

## Permission Mode

Set a default permission mode:

```json
{ "permissionMode": "workspace-write" }
```

Or pass at launch:

```bash
claw --permission-mode read-only
```

Or skip all checks entirely (use with caution):

```bash
claw --dangerously-skip-permissions
```

### Fine-Grained Permission Rules

Control individual tools with allow/deny/ask lists. Patterns support glob matching on tool names and command arguments for bash:

```json
{
  "permissions": {
    "allow": [
      "read_file",
      "glob_search",
      "bash(git *)",
      "bash(cargo *)"
    ],
    "deny": [
      "bash(rm *)",
      "bash(curl *)"
    ],
    "ask": [
      "write_file",
      "edit_file"
    ]
  }
}
```

- **allow** — always permitted without prompting
- **deny** — always blocked, tool returns an error
- **ask** — prompts for interactive Y/N approval each time

---

## Hooks

Hooks let you run scripts before or after tool calls. Currently the config is loaded and visible in `/config hooks`, but execution of hook scripts is not yet implemented.

```json
{
  "hooks": {
    "preToolUse": ["./scripts/pre-tool.sh"],
    "postToolUse": ["./scripts/post-tool.sh"],
    "postToolUseFailure": ["./scripts/on-failure.sh"]
  }
}
```

Each entry is a path or command string. Hooks will be called with tool name and input as arguments when the execution pipeline is implemented.

---

## CLAUDE.md — Project Memory

`CLAUDE.md` files are plain Markdown files read at startup. They inject persistent project context into every conversation.

The runtime discovers them by walking up from the current working directory. Files are capped at 4,000 characters each, with a total limit of 12,000 characters across all files.

Standard locations searched:

- `<cwd>/CLAUDE.md`
- `<cwd>/.claude/CLAUDE.md`
- `<parent dirs up to repo root>/CLAUDE.md`

Create one with:

```bash
claw init
# or inside REPL:
/init
```

Example `CLAUDE.md`:

```markdown
# Project: My App

## Stack
- Rust, Tokio, Axum
- PostgreSQL via sqlx

## Conventions
- Use `anyhow::Result` for error handling
- All public functions need doc comments
- Run `cargo fmt` and `cargo clippy` before committing

## Commands
- `cargo test --workspace` — run all tests
- `cargo build --release` — release build
```

Inspect loaded files in the REPL:

```
/memory
```

---

## Sandbox

Linux sandbox isolation can be configured:

```json
{
  "sandbox": {
    "enabled": true,
    "namespaceRestrictions": true,
    "networkIsolation": false,
    "filesystemMode": "workspace-only",
    "allowedMounts": []
  }
}
```

| Setting | Default | Description |
|---|---|---|
| `enabled` | `true` | Enable sandbox when supported |
| `namespaceRestrictions` | `true` | Use Linux user namespaces |
| `networkIsolation` | `false` | Block network in bash tool |
| `filesystemMode` | `workspace-only` | `off` / `workspace-only` / `allow-list` |
| `allowedMounts` | `[]` | Extra paths allowed in `allow-list` mode |

Check sandbox status in the REPL:

```
/sandbox
```

---

## Session Storage

Sessions are stored as `.jsonl` files in `~/.claude/sessions/` by default.

The `PRIMARY_SESSION_EXTENSION` is `.jsonl`. Legacy `.json` sessions are also readable.

Special references:

| Reference | Meaning |
|---|---|
| `latest` | Most recently written session |
| `last` | Alias for `latest` |
| `recent` | Alias for `latest` |

---

## Environment Variables

| Variable | Description |
|---|---|
| `ANTHROPIC_API_KEY` | Anthropic API key |
| `ANTHROPIC_BASE_URL` | Anthropic API base URL override |
| `OPENAI_API_KEY` | OpenAI / compatible provider key |
| `OPENAI_BASE_URL` | OpenAI base URL |
| `XAI_API_KEY` | xAI Grok API key |
| `CLAUDE_CODE_UPSTREAM` | Path to upstream TypeScript source for parity tooling |
| `CLAUDE_CODE_AUTO_COMPACT_INPUT_TOKENS` | Token threshold for auto-compaction (default: 100000) |

---

## Plugins

Plugin configuration:

```json
{
  "plugins": {
    "enabled": {
      "my-plugin-name": true
    },
    "externalDirectories": ["/path/to/plugin/dir"],
    "installRoot": "/path/to/install/root",
    "registryPath": "/path/to/registry.json",
    "bundledRoot": "/path/to/bundled/plugins"
  }
}
```

Full plugin install/lifecycle management (the `claw plugin install/update` subcommands) is planned but not yet implemented.
