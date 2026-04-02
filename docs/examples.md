# Usage Examples & Use Cases

## What It Looks Like

Claw Code is a **terminal CLI agent**. You type plain English; the AI autonomously runs tools (bash, file reads/writes, web search) to complete your request:

```
claw> fix all the clippy warnings in the project

🦀 Thinking...

╭─ bash ──────────────────────────────────╮
│ cargo clippy --workspace 2>&1           │
╰─────────────────────────────────────────╯
✓ bash (exit 0)

╭─ read_file ─────────────────────────────╮
│ rust/crates/runtime/src/compact.rs      │
╰─────────────────────────────────────────╯
✓ read_file

╭─ edit_file ─────────────────────────────╮
│ rust/crates/runtime/src/compact.rs      │
╰─────────────────────────────────────────╯
✓ edit_file

Done. Fixed 3 unused variable warnings and 1 clippy::needless_pass_by_ref.
```

---

## Practical Examples

### 1. Coding Assistant

```bash
claw
claw> refactor the permission system to add a new "sandboxed" tier between read-only and workspace-write
```

The agent reads the relevant files, plans the changes, edits them, and runs tests — all autonomously.

---

### 2. One-Shot Automation (CI / Scripts)

```bash
# Explain a codebase
claw prompt "give me a high-level summary of what each crate does"

# Auto-fix failing tests
claw prompt "run cargo test and fix any failures"

# Generate a commit message from staged changes
claw prompt "look at git diff --staged and write a conventional commit message"
```

---

### 3. Code Review

```bash
# Read-only mode — cannot touch files, only analyzes
claw --permission-mode read-only prompt "review main.rs for security issues and anti-patterns"
```

---

### 4. Bulk File Operations

```bash
claw prompt "find every .rs file that imports std::fs directly and refactor them to use tokio::fs instead"
```

The agent uses `glob_search` + `grep_search` to find files, then `edit_file` on each one.

---

### 5. Restrict to Safe Tools Only

```bash
# Only let the agent read and search — no writes, no bash
claw --allowedTools "read_file,glob_search,grep_search,WebSearch" \
     prompt "how does session compaction work in this codebase?"
```

---

### 6. Use a Cheaper / Faster Model

```bash
claw --model haiku prompt "rename the function run_turn to execute_turn everywhere"
```

---

### 7. Resume a Long Session

```bash
# Start a big refactoring task, close your laptop, come back later
claw --resume latest

# Or from inside the REPL
claw> /resume latest
```

---

### 8. Extend with External Tools via MCP

Add to `.claude.json` in your project:

```json
{
  "mcp": {
    "servers": {
      "github": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-github"],
        "env": { "GITHUB_PERSONAL_ACCESS_TOKEN": "ghp_..." }
      }
    }
  }
}
```

Now the agent can read GitHub issues, create PRs, etc.:

```
claw> look at the open issues tagged "bug" and fix the first one
```

---

### 9. JSON Output for Pipelines

```bash
claw --output-format json prompt "list all public functions in tools/src/lib.rs" | jq '.result'
```

---

### 10. Project Memory via CLAUDE.md

Create `CLAUDE.md` at the root of your repo:

```markdown
## Stack
- Rust, Tokio, async everywhere
- Use `anyhow::Result` not `Box<dyn Error>`

## Test command
cargo test --workspace

## Never touch
- rust/crates/compat-harness/  (read-only reference code)
```

Every `claw` session in that directory will know your conventions automatically.

---

## Use Cases at a Glance

| Use Case | Command |
|---|---|
| Autonomous code refactoring | `claw prompt "migrate all sync file I/O to async tokio::fs"` |
| Bug hunting | `/bughunter` — agent scans for panics, logic errors, security issues |
| Writing tests | `claw prompt "write unit tests for compact.rs"` |
| Adding documentation | `claw prompt "add doc comments to all public functions in the api crate"` |
| DevOps scripting | `claw prompt "write a deploy.sh that builds release, runs tests, and rsyncs to server"` |
| Research | `claw --allowedTools WebSearch,WebFetch prompt "summarize the latest Rust async patterns"` |
| Git workflow | `/commit`, `/pr`, `/diff` — AI-assisted git operations |
| Deep planning | `/ultraplan build a REST API layer on top of this CLI` |
| Code review | `claw --permission-mode read-only prompt "review this PR for issues"` |
| Multi-session long projects | `--resume latest` — pick up exactly where you left off |

---

## Can You Make Your Own Model From It?

**No — and that is not what this does.**

Claw Code is a **harness/runtime**, not a model. It sends your prompts to an existing LLM (Claude, GPT-4o, Grok) via API and handles the agent loop: tool execution, sessions, permissions, and streaming output.

What you **can** do:

| Goal | How |
|---|---|
| Use a different cloud LLM | Set `OPENAI_API_KEY` and use a GPT or Grok model name |
| Use a local model | Point `OPENAI_BASE_URL` to Ollama, LM Studio, or any OpenAI-compatible server |
| Add custom tools | Write a Rust plugin (`PluginTool` trait) or connect an MCP server |
| Add domain knowledge | Edit `CLAUDE.md` — inject coding standards, architecture notes, project facts |
| Customize agent behavior | MIT-licensed Rust — modify the system prompt in `runtime/src/prompt.rs`, add slash commands in `commands/src/lib.rs`, add tools in `tools/src/lib.rs` |
| Use a fine-tuned model | If you host an OpenAI-compatible fine-tune, just set `OPENAI_BASE_URL` |

### Example: Local Ollama Model

```bash
export OPENAI_BASE_URL="http://localhost:11434/v1"
export OPENAI_API_KEY="ollama"      # placeholder value required
claw --model qwen2.5-coder:32b prompt "review this code"
```

### Example: Any OpenAI-Compatible API

```bash
export OPENAI_BASE_URL="https://my-company-llm-proxy.internal/v1"
export OPENAI_API_KEY="..."
claw --model my-internal-model prompt "fix the bugs in src/"
```

---

## Full Custom Stack

You control everything except the model weights:

```
Your model (cloud, local, fine-tuned, any OpenAI-compatible)
        ↑  API calls
  Claw Code (this repo) — tools, sessions, permissions, streaming
        ↑  spawns
  MCP servers — extend the tool set with anything you need
        ↑
  CLAUDE.md — inject your domain knowledge into every session
```
