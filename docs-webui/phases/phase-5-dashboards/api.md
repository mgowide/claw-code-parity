# Phase 5 — Backend API Routes

> New REST endpoints for files, config, tools, and MCP management.

---

## File Routes

### `GET /api/files?path=.`

List directory contents.

**Response**:
```json
{
  "path": ".",
  "entries": [
    { "name": "src", "type": "directory", "children_count": 8 },
    { "name": "Cargo.toml", "type": "file", "size": 1240, "modified": "2026-04-02T10:00:00Z" },
    { "name": "README.md", "type": "file", "size": 3400, "modified": "2026-04-01T09:00:00Z" }
  ]
}
```

**Security**: Only serves files within the workspace root. Path traversal (`../`) is rejected with 400.

### `GET /api/files/read?path=src/main.rs&start=1&end=100`

Read file content with optional line range.

**Response**:
```json
{
  "path": "src/main.rs",
  "content": "fn main() {\n    ...\n}",
  "language": "rust",
  "total_lines": 145,
  "start_line": 1,
  "end_line": 100
}
```

**Security**: 
- Max file size: 1MB (reject larger with 413)
- Binary file detection: reject with 415
- Path within workspace only

---

## Config Routes

### `GET /api/config`

Read current `.claude.json`.

**Response**:
```json
{
  "model": "claude-sonnet-4-20250514",
  "permission_mode": "default",
  "allowed_tools": ["bash", "read_file", "edit_file"],
  "hooks": {
    "pre_tool_call": "",
    "post_session": ""
  },
  "mcp_servers": [],
  "sandbox": { "enabled": false }
}
```

### `PUT /api/config`

Write `.claude.json`. Body is the full config object.

**Validation**:
- JSON schema validation before writing
- Returns 400 with details on invalid fields
- Creates backup of previous config (`.claude.json.bak`)

**Response**: `200 OK` with saved config.

---

## Tool Routes

### `GET /api/tools`

List all available tools.

**Response**:
```json
{
  "tools": [
    {
      "name": "bash",
      "description": "Execute a shell command",
      "permission": "danger",
      "source": "builtin",
      "call_count": 12,
      "enabled": true,
      "parameters": {
        "command": { "type": "string", "required": true },
        "timeout": { "type": "integer", "default": 30000 }
      }
    }
  ],
  "total": 23,
  "builtin": 20,
  "plugin": 3
}
```

---

## MCP Routes

### `GET /api/mcp/servers`

List connected MCP servers.

**Response**:
```json
{
  "servers": [
    {
      "id": "fs-1",
      "name": "filesystem-server",
      "transport": "stdio",
      "command": "npx @modelcontextprotocol/server-filesystem /home/user/project",
      "status": "connected",
      "tools": [
        { "name": "read_file", "description": "Read a file" },
        { "name": "write_file", "description": "Write a file" }
      ],
      "connected_at": "2026-04-02T10:00:00Z"
    }
  ]
}
```

### `POST /api/mcp/servers`

Add and connect a new MCP server.

**Body**:
```json
{
  "name": "postgres-server",
  "transport": "sse",
  "url": "http://localhost:8080/sse"
}
```

**Response**: Server object with `status: "connecting"`.

### `DELETE /api/mcp/servers/:id`

Disconnect and remove an MCP server. Returns `204 No Content`.

---

## Security Notes

All file routes enforce:
- **Path traversal protection**: `..` segments rejected, paths resolved and checked against workspace root
- **File size limits**: 1MB max for reads
- **Binary detection**: reject non-text files for read
- **Rate limiting**: 100 req/s per route (tower middleware)

Config writes:
- **Schema validation**: JSON must match expected structure
- **Backup**: previous config saved as `.bak` before overwrite

---

*Back: [README](README.md) | Panels: [panels.md](panels.md) | Views: [views.md](views.md)*
