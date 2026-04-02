# REST API Reference

> Complete HTTP API surface served by the Axum backend.

---

## Base URL

```
http://localhost:3100/api
```

---

## Health

### `GET /api/status`

Server health and info.

**Response** `200`:
```json
{
  "status": "ok",
  "version": "0.1.0",
  "model": "claude-sonnet-4-20250514",
  "uptime_seconds": 3600,
  "active_sessions": 2,
  "models": [
    { "id": "claude-sonnet-4-20250514", "provider": "anthropic", "active": true },
    { "id": "claude-opus-4-20250514", "provider": "anthropic" }
  ]
}
```

---

## Auth

### `POST /api/auth/login`

Initiate OAuth PKCE flow.

**Response** `200`:
```json
{ "auth_url": "https://console.anthropic.com/oauth/authorize?..." }
```

### `GET /api/auth/callback?code=...&state=...`

OAuth callback. Exchanges code for token and stores it.

**Response** `302` → redirects to `/`

---

## Sessions

### `POST /api/sessions`

Create a new session.

**Body** (optional):
```json
{ "model": "claude-opus-4-20250514", "system_prompt": "You are a helpful assistant." }
```

**Response** `201`:
```json
{
  "session_id": "abc-123",
  "model": "claude-opus-4-20250514",
  "created_at": "2026-04-02T10:30:00Z"
}
```

### `GET /api/sessions`

List all sessions.

**Query params**:
- `search` — full-text search across transcripts
- `sort` — `date` (default), `cost`, `messages`
- `order` — `desc` (default), `asc`

**Response** `200`:
```json
{
  "sessions": [
    {
      "id": "abc-123",
      "name": "Refactor auth module",
      "created_at": "2026-04-02T10:30:00Z",
      "updated_at": "2026-04-02T11:15:00Z",
      "message_count": 24,
      "input_tokens": 8400,
      "output_tokens": 3200,
      "cost": 0.042,
      "model": "claude-sonnet-4-20250514"
    }
  ]
}
```

### `GET /api/sessions/:id`

Get full session with transcript.

**Response** `200`: Full session JSON (see Phase 3 API doc).

### `DELETE /api/sessions/:id`

Delete a session.

**Response** `204` No Content.

### `GET /api/sessions/:id/export`

Export session transcript.

**Query params**:
- `format` — `json` (default), `md`

**Response** `200` with `Content-Disposition: attachment` header.

---

## Files

### `GET /api/files?path=.`

List directory contents.

**Query params**:
- `path` — relative path from workspace root (default: `.`)
- `recursive` — `true` for full tree (default: `false`)

**Response** `200`:
```json
{
  "path": "src",
  "entries": [
    { "name": "main.rs", "type": "file", "size": 1240, "modified": "2026-04-02T10:00:00Z" },
    { "name": "routes", "type": "directory", "children_count": 3 }
  ]
}
```

**Errors**:
- `400` — path traversal attempt (`..`)
- `404` — path doesn't exist

### `GET /api/files/read?path=src/main.rs`

Read file content.

**Query params**:
- `path` — relative path (required)
- `start` — start line, 1-based (optional)
- `end` — end line (optional)

**Response** `200`:
```json
{
  "path": "src/main.rs",
  "content": "fn main() {\n    ...\n}",
  "language": "rust",
  "total_lines": 145,
  "start_line": 1,
  "end_line": 145
}
```

**Errors**:
- `400` — path traversal
- `404` — file not found
- `413` — file too large (>1MB)
- `415` — binary file

---

## Config

### `GET /api/config`

Read current `.claude.json`.

**Response** `200`: Config object.

### `PUT /api/config`

Write `.claude.json`.

**Body**: Full config object.

**Response** `200`: Saved config.

**Errors**:
- `400` — invalid JSON or schema validation failure (with details)

---

## Tools

### `GET /api/tools`

List all available tools.

**Response** `200`:
```json
{
  "tools": [...],
  "total": 23,
  "builtin": 20,
  "plugin": 3
}
```

---

## MCP

### `GET /api/mcp/servers`

List connected MCP servers.

**Response** `200`:
```json
{
  "servers": [
    {
      "id": "fs-1",
      "name": "filesystem-server",
      "transport": "stdio",
      "status": "connected",
      "tools": [...]
    }
  ]
}
```

### `POST /api/mcp/servers`

Add MCP server.

**Body**:
```json
{
  "name": "my-server",
  "transport": "stdio",
  "command": "npx @example/mcp-server"
}
```

**Response** `201`: Server object.

### `DELETE /api/mcp/servers/:id`

Remove MCP server.

**Response** `204` No Content.

---

*Back: [docs-webui/README.md](../README.md)*
