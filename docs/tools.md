# Tools Reference

The agent selects and invokes tools autonomously based on your request. You can restrict which tools are active with `--allowedTools`.

## Permission Tiers

| Tier | Grants |
|---|---|
| `read-only` | Read files, search, web fetch |
| `workspace-write` | read-only + write/edit files |
| `danger-full-access` | workspace-write + bash execution |

## Tool Aliases

These short names work in `--allowedTools`:

| Alias | Resolves to |
|---|---|
| `read` | `read_file` |
| `write` | `write_file` |
| `edit` | `edit_file` |
| `glob` | `glob_search` |
| `grep` | `grep_search` |

---

## File System Tools

### `read_file`

**Permission**: `read-only`

Read a text file from the workspace. Supports offset/limit for large files.

```json
{
  "path": "src/main.rs",
  "offset": 0,
  "limit": 100
}
```

| Parameter | Type | Required | Description |
|---|---|---|---|
| `path` | string | ✅ | File path relative to cwd |
| `offset` | integer | | Line offset to start from |
| `limit` | integer | | Max lines to return |

---

### `write_file`

**Permission**: `workspace-write`

Write a complete file. Creates parent directories if they don't exist. Overwrites existing content.

```json
{
  "path": "src/new_module.rs",
  "content": "pub fn hello() {}\n"
}
```

| Parameter | Type | Required | Description |
|---|---|---|---|
| `path` | string | ✅ | Destination path |
| `content` | string | ✅ | Full file content |

---

### `edit_file`

**Permission**: `workspace-write`

Replace an exact string in a file. Safer than `write_file` for targeted edits — fails if `old_string` is not found.

```json
{
  "path": "src/lib.rs",
  "old_string": "fn foo() -> i32 { 0 }",
  "new_string": "fn foo() -> i32 { 42 }",
  "replace_all": false
}
```

| Parameter | Type | Required | Description |
|---|---|---|---|
| `path` | string | ✅ | Target file path |
| `old_string` | string | ✅ | Exact text to replace |
| `new_string` | string | ✅ | Replacement text |
| `replace_all` | boolean | | Replace all occurrences (default: false) |

---

### `glob_search`

**Permission**: `read-only`

Find files matching a glob pattern.

```json
{
  "pattern": "**/*.rs",
  "path": "rust/crates/runtime"
}
```

| Parameter | Type | Required | Description |
|---|---|---|---|
| `pattern` | string | ✅ | Glob pattern (e.g. `**/*.rs`) |
| `path` | string | | Root directory to search from |

---

### `grep_search`

**Permission**: `read-only`

Search file contents using a regex pattern. Supports context lines, case-insensitive matching, and multiline mode.

```json
{
  "pattern": "fn execute",
  "path": "rust/crates",
  "glob": "*.rs",
  "-A": 3,
  "-B": 1,
  "-i": false
}
```

| Parameter | Type | Description |
|---|---|---|
| `pattern` | string | Regex pattern to search for |
| `path` | string | Directory to search in |
| `glob` | string | File glob filter (e.g. `*.rs`) |
| `-A` | integer | Lines of context after each match |
| `-B` | integer | Lines of context before each match |
| `-C` | integer | Lines of context on both sides |
| `context` | integer | Alias for `-C` |
| `-n` | boolean | Show line numbers |
| `-i` | boolean | Case-insensitive matching |
| `multiline` | boolean | Enable multiline mode |
| `head_limit` | integer | Max matches to return |
| `offset` | integer | Skip first N matches |

---

## Shell Execution

### `bash`

**Permission**: `danger-full-access`

Execute a shell command. Runs in the current working directory unless overridden. Supports timeout, background execution, and optional sandbox isolation.

```json
{
  "command": "cargo test --workspace",
  "timeout": 120,
  "description": "Run all workspace tests"
}
```

| Parameter | Type | Required | Description |
|---|---|---|---|
| `command` | string | ✅ | Shell command to execute |
| `timeout` | integer | | Timeout in seconds |
| `description` | string | | Human-readable description for display |
| `run_in_background` | boolean | | Run without waiting for output |
| `dangerouslyDisableSandbox` | boolean | | Skip sandbox even if globally enabled |
| `namespaceRestrictions` | boolean | | Enable Linux namespace isolation |
| `isolateNetwork` | boolean | | Block network access during execution |
| `filesystemMode` | string | | `off` / `workspace-only` / `allow-list` |
| `allowedMounts` | array | | Allowed filesystem paths (for `allow-list` mode) |

---

## Web Tools

### `WebFetch`

**Permission**: `read-only`

Fetch a URL, extract readable text, and answer a prompt about the content.

```json
{
  "url": "https://docs.rs/tokio/latest/tokio/",
  "prompt": "What async runtimes does tokio provide?"
}
```

| Parameter | Type | Required | Description |
|---|---|---|---|
| `url` | string (URI) | ✅ | URL to fetch |
| `prompt` | string | ✅ | Question to answer about the fetched content |

---

### `WebSearch`

**Permission**: `read-only`

Search the web and return cited results.

```json
{
  "query": "Rust async trait object 2025",
  "allowed_domains": ["doc.rust-lang.org", "blog.rust-lang.org"],
  "blocked_domains": ["w3schools.com"]
}
```

| Parameter | Type | Required | Description |
|---|---|---|---|
| `query` | string | ✅ | Search query (min 2 chars) |
| `allowed_domains` | array | | Restrict results to these domains |
| `blocked_domains` | array | | Exclude results from these domains |

---

## Task Management

### `TodoWrite`

**Permission**: `workspace-write`

Update the structured task list for the current session. The agent uses this to track multi-step work.

```json
{
  "todos": [
    { "content": "Fix the failing test", "activeForm": "Fix failing test", "status": "in_progress" },
    { "content": "Update docs", "activeForm": "Update docs", "status": "pending" }
  ]
}
```

| Parameter | Type | Required | Description |
|---|---|---|---|
| `todos` | array | ✅ | Complete todo list (replaces previous) |
| `todos[].content` | string | ✅ | Full task description |
| `todos[].activeForm` | string | ✅ | Short display label |
| `todos[].status` | string | ✅ | `pending` / `in_progress` / `completed` |

---

## Agent & Orchestration Tools

### `Agent`

**Permission**: inherits from parent

Spawn a sub-agent with its own conversation context. Used for parallel or isolated sub-tasks.

```json
{
  "prompt": "Audit all TODO comments in the codebase",
  "tools": ["read_file", "glob_search", "grep_search"]
}
```

| Parameter | Type | Description |
|---|---|---|
| `prompt` | string | Task for the sub-agent |
| `tools` | array | Optional restricted tool list for the sub-agent |

---

### `REPL`

**Permission**: `danger-full-access`

Start an interactive Node.js-style stateful REPL for multi-step evaluation.

---

### `PowerShell`

**Permission**: `danger-full-access`

Execute a PowerShell command (Windows only).

---

## Configuration & Metadata Tools

### `Config`

**Permission**: `read-only` (read) / `workspace-write` (write)

Read or modify `.claude.json` configuration values.

```json
{
  "action": "get",
  "key": "model"
}
```

---

### `StructuredOutput`

**Permission**: `read-only`

Emit typed JSON output, used when `--output-format json` is active.

---

### `Sleep`

**Permission**: `read-only`

Pause execution for a given number of milliseconds.

---

### `SendUserMessage`

**Permission**: `read-only`

Send a plain-text or rich message to the user's terminal.

---

## Knowledge & Discovery Tools

### `Skill`

**Permission**: `read-only`

Load a local `SKILL.md` file from the workspace and inject its instructions into the agent context.

```json
{
  "path": ".claude/skills/testing.md"
}
```

---

### `ToolSearch`

**Permission**: `read-only`

Search the available tool catalog by keyword.

---

### `NotebookEdit`

**Permission**: `workspace-write`

Edit Jupyter notebook cells (`.ipynb` files).

---

## Restricting Tools

Pass a comma-separated list or multiple flags to `--allowedTools`:

```bash
# Allow only read and search tools
claw --allowedTools "read_file,glob_search,grep_search,WebSearch"

# Single flag per tool
claw --allowedTools read_file --allowedTools bash
```

Names are normalized: case-insensitive, hyphens and underscores are interchangeable.

---

## Plugin Tools

Plugins can contribute additional tools. Plugin tools follow the same permission tier system. A plugin tool name must not conflict with any built-in tool name.

See [Configuration — Plugins](configuration.md#plugins) for setup.
