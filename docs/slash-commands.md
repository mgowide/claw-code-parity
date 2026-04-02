# Slash Commands

Slash commands are typed directly in the REPL prompt (e.g. `/status`). Tab-completion works for command names and many arguments.

---

## Session

### `/status`

Show the current session summary: model, permission mode, session ID, message count, token usage.

```
/status
```

---

### `/cost`

Show cumulative token usage and estimated cost for the current session.

```
/cost
```

---

### `/compact`

Manually compact the session history. Summarises old messages to free up context window space while preserving recent messages. Happens automatically when input tokens exceed ~100K.

```
/compact
```

---

### `/clear [--confirm]`

Start a fresh session, discarding the current conversation. Prompts for confirmation unless `--confirm` is passed.

```
/clear
/clear --confirm
```

---

### `/resume <session-path>`

Load a previously saved session file into the current REPL.

```
/resume latest
/resume ~/.claude/sessions/abc123.jsonl
```

The alias `latest` (or `last`, `recent`) loads the most recently written session.

---

### `/export`

Export the current conversation to a file.

```
/export
/export /tmp/my-session.md
```

---

## Model & Permissions

### `/model [model]`

Show the active model, or switch to a new one.

```
/model
/model sonnet
/model claude-opus-4-6
```

Model aliases: `opus` → `claude-opus-4-6`, `sonnet` → `claude-sonnet-4-6`, `haiku` → `claude-haiku-4-5-20251213`

---

### `/permissions [mode]`

Show the current permission mode, or switch to a new one.

```
/permissions
/permissions read-only
/permissions workspace-write
/permissions danger-full-access
```

---

### `/sandbox`

Show the current sandbox isolation status — whether Linux namespace, network, and filesystem isolation are active.

```
/sandbox
```

---

## Configuration & Memory

### `/config [section]`

Inspect the merged `.claude.json` configuration. Optionally filter to a specific section.

```
/config
/config env
/config hooks
/config model
/config plugins
```

---

### `/memory`

Show all `CLAUDE.md` instruction files loaded into the current session's context, and their content.

```
/memory
```

---

### `/init`

Create a starter `CLAUDE.md` file in the current directory, pre-populated with project detection hints.

```
/init
```

---

## Git Workflow

### `/diff`

Show the `git diff` output for the current workspace, including staged and unstaged changes.

```
/diff
```

---

### `/commit`

Ask the agent to inspect staged changes, generate a commit message, and create a commit.

```
/commit
```

---

### `/pr [context]`

Draft or create a pull request from the current conversation and changes.

```
/pr
/pr "this fixes the auth bug from issue #42"
```

---

### `/issue [context]`

Draft or create a GitHub issue from the conversation context.

```
/issue
/issue "search is broken when query contains spaces"
```

---

## AI Workflow

### `/ultraplan [task]`

Run a deep multi-step planning prompt. The agent reasons through the task and produces a structured execution plan before taking any action.

```
/ultraplan
/ultraplan refactor the database layer to use connection pooling
```

---

### `/bughunter [scope]`

Ask the agent to inspect the codebase for likely bugs, anti-patterns, and security issues.

```
/bughunter
/bughunter rust/crates/runtime/src/conversation.rs
```

---

### `/teleport <symbol-or-path>`

Jump to a file or symbol by searching the workspace and opening the result.

```
/teleport ConversationRuntime
/teleport src/main.rs
```

---

### `/agents`

Manage and list sub-agents spawned in the current session.

```
/agents
```

---

### `/skills`

List and manage loaded skills.

```
/skills
```

---

### `/plugins`

List installed plugins and their contributed tools.

```
/plugins
```

---

## Debug & Info

### `/version`

Show the CLI version, build target, and git SHA.

```
/version
```

---

### `/help`

List all available slash commands with their summaries and argument hints.

```
/help
```

---

### `/debug-tool-call`

Replay the last tool call with full debug output: raw input JSON, permission check result, raw output.

```
/debug-tool-call
```

---

## Resume Support

Commands marked as "resume-supported" are safe to run immediately after loading a session with `/resume`. Commands that trigger new model calls (like `/model` or `/permissions`) are not resume-safe and will be blocked when appropriate.

| Command | Resume Safe |
|---|---|
| `/status` | ✅ |
| `/cost` | ✅ |
| `/compact` | ✅ |
| `/config` | ✅ |
| `/memory` | ✅ |
| `/diff` | ✅ |
| `/version` | ✅ |
| `/help` | ✅ |
| `/sandbox` | ✅ |
| `/model` | ❌ |
| `/permissions` | ❌ |
| `/bughunter` | ❌ |
| `/commit` | ❌ |
| `/pr` | ❌ |
| `/issue` | ❌ |
| `/ultraplan` | ❌ |
| `/teleport` | ❌ |
| `/debug-tool-call` | ❌ |
