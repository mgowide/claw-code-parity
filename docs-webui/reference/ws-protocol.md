# WebSocket Protocol Reference

> Complete spec for the bidirectional WebSocket at `/ws`.

---

## Connection

```
URL: ws://localhost:3100/ws
Protocol: JSON text frames (no binary)
Heartbeat: Client sends ping every 30s, server responds with pong
Reconnect: Client side, exponential backoff (1s → 30s max)
```

---

## Server → Client Events

All events are JSON objects with a `type` discriminator field.

### `text_delta`
Partial text from the AI response.
```json
{ "type": "text_delta", "text": "Hello, " }
```

### `thinking_start`
The AI has started an extended thinking block.
```json
{ "type": "thinking_start" }
```

### `thinking_end`
Extended thinking complete.
```json
{ "type": "thinking_end" }
```

### `tool_use_start`
The AI wants to call a tool.
```json
{
  "type": "tool_use_start",
  "id": "toolu_01abc",
  "name": "read_file",
  "input": { "path": "src/main.rs", "start_line": 1, "end_line": 50 }
}
```

### `tool_result`
Tool execution completed.
```json
{
  "type": "tool_result",
  "id": "toolu_01abc",
  "output": "fn main() {\n    ...\n}",
  "is_error": false
}
```

### `diff`
File diff from `edit_file` or `write_file`.
```json
{
  "type": "diff",
  "path": "src/main.rs",
  "old_content": "fn main() {\n    println!(\"hello\");\n}",
  "new_content": "fn main() {\n    println!(\"world\");\n}"
}
```

### `usage`
Token usage update for current turn.
```json
{
  "type": "usage",
  "input_tokens": 1200,
  "output_tokens": 340,
  "cache_hits": 800,
  "cost": 0.0043
}
```

### `turn_complete`
AI has finished responding for this turn.
```json
{ "type": "turn_complete", "turn_index": 3 }
```

### `permission_request`
A tool requires user approval before execution.
```json
{
  "type": "permission_request",
  "id": "perm_01xyz",
  "tool": "bash",
  "description": "Run: rm -rf node_modules && npm install"
}
```

### `session_compacted`
Session was compacted to reduce token count.
```json
{ "type": "session_compacted", "removed_messages": 12 }
```

### `error`
An error occurred.
```json
{ "type": "error", "message": "API rate limit exceeded" }
```

### `connected`
Sent immediately after WebSocket handshake.
```json
{ "type": "connected", "session_id": "abc-123", "model": "claude-sonnet-4-20250514" }
```

---

## Client → Server Commands

### `send_message`
Send a user message to the active session.
```json
{
  "type": "send_message",
  "session_id": "abc-123",
  "text": "explain this code",
  "attachments": [
    { "name": "main.rs", "content": "fn main() { ... }" }
  ]
}
```
`attachments` is optional.

### `cancel_turn`
Stop the current AI response.
```json
{ "type": "cancel_turn", "session_id": "abc-123" }
```

### `approve_permission`
Approve a pending permission request.
```json
{ "type": "approve_permission", "request_id": "perm_01xyz" }
```

### `deny_permission`
Deny a pending permission request.
```json
{ "type": "deny_permission", "request_id": "perm_01xyz" }
```

### `compact`
Manually trigger session compaction.
```json
{ "type": "compact", "session_id": "abc-123" }
```

### `switch_model`
Change the AI model mid-conversation.
```json
{ "type": "switch_model", "model": "claude-opus-4-20250514" }
```

### `resume_session`
Load and resume a previous session.
```json
{ "type": "resume_session", "session_id": "def-456" }
```

---

## Event Sequence Diagram

### Normal Turn

```
Client                              Server
  |  {"type":"send_message",...}      |
  |  ─────────────────────────────►   |
  |                                   |  (calls API, streams response)
  |  {"type":"text_delta","text":"H"} |
  |  ◄─────────────────────────────   |
  |  {"type":"text_delta","text":"i"} |
  |  ◄─────────────────────────────   |
  |  {"type":"usage",...}             |
  |  ◄─────────────────────────────   |
  |  {"type":"turn_complete",...}     |
  |  ◄─────────────────────────────   |
```

### Turn With Tool Call

```
Client                              Server
  |  {"type":"send_message",...}      |
  |  ─────────────────────────────►   |
  |  {"type":"text_delta",...}        |
  |  ◄─────────────────────────────   |
  |  {"type":"tool_use_start",...}    |
  |  ◄─────────────────────────────   |  (executes tool)
  |  {"type":"diff",...}              |  (if edit_file)
  |  ◄─────────────────────────────   |
  |  {"type":"tool_result",...}       |
  |  ◄─────────────────────────────   |
  |  {"type":"text_delta",...}        |  (AI continues after tool)
  |  ◄─────────────────────────────   |
  |  {"type":"turn_complete",...}     |
  |  ◄─────────────────────────────   |
```

### Turn With Permission Gate

```
Client                              Server
  |  {"type":"send_message",...}      |
  |  ─────────────────────────────►   |
  |  {"type":"permission_request",...}|
  |  ◄─────────────────────────────   |  (blocks, waiting)
  |                                   |
  |  (user clicks Allow)             |
  |  {"type":"approve_permission",..} |
  |  ─────────────────────────────►   |  (resumes execution)
  |  {"type":"tool_use_start",...}    |
  |  ◄─────────────────────────────   |
  |  ...                              |
```

---

*Back: [docs-webui/README.md](../README.md)*
