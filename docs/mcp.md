# MCP Integration

MCP (Model Context Protocol) lets you connect external tool servers to Claw Code. Any MCP server becomes part of the agent's tool catalog automatically.

## Overview

When an MCP server is configured, Claw Code:

1. Starts or connects to the server on launch
2. Calls `tools/list` (JSON-RPC) to discover its tools
3. Exposes each tool to the model as `mcp__<server-name>__<tool-name>`
4. Routes tool calls to the server via `tools/call`

---

## Supported Transports

| Transport | Config type | Use case |
|---|---|---|
| `stdio` | `McpStdioTransport` | Local subprocess (most common) |
| `sse` | `McpRemoteTransport` | Remote HTTP server-sent events |
| `http` | `McpRemoteTransport` | Remote HTTP streamable |
| `websocket` | `McpRemoteTransport` | Remote WebSocket |
| `sdk` | `McpSdkTransport` | Managed SDK server by name |
| `managed-proxy` | `McpManagedProxyTransport` | Anthropic-managed CCR proxy |

---

## Configuration

MCP servers are configured in `.claude.json` under the `mcp.servers` key. Config can live in user, project, or local config files.

### stdio (subprocess)

The most common configuration. Claw Code spawns the server as a child process.

```json
{
  "mcp": {
    "servers": {
      "filesystem": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-filesystem", "/home/user/projects"],
        "env": {
          "NODE_ENV": "production"
        }
      }
    }
  }
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | `"stdio"` | ✅ | Transport type |
| `command` | string | ✅ | Executable to run |
| `args` | array | | Arguments |
| `env` | object | | Extra environment variables |
| `toolCallTimeoutMs` | integer | | Timeout for tool calls (default: 60,000ms) |

---

### SSE (remote)

Connect to a remote MCP server using server-sent events.

```json
{
  "mcp": {
    "servers": {
      "remote-tools": {
        "type": "sse",
        "url": "https://mcp.example.com/events",
        "headers": {
          "Authorization": "Bearer my-token"
        }
      }
    }
  }
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | `"sse"` | ✅ | Transport type |
| `url` | string | ✅ | SSE endpoint URL |
| `headers` | object | | HTTP headers |
| `headersHelper` | string | | Script/command to generate auth headers |
| `auth` | object | | OAuth config (see below) |

---

### HTTP (streamable)

```json
{
  "mcp": {
    "servers": {
      "http-tools": {
        "type": "http",
        "url": "https://mcp.example.com/mcp",
        "headers": {
          "X-Api-Key": "..."
        }
      }
    }
  }
}
```

---

### WebSocket

```json
{
  "mcp": {
    "servers": {
      "ws-tools": {
        "type": "websocket",
        "url": "wss://mcp.example.com/ws"
      }
    }
  }
}
```

---

## OAuth for Remote Servers

Remote MCP servers that require OAuth can be configured:

```json
{
  "mcp": {
    "servers": {
      "secure-server": {
        "type": "sse",
        "url": "https://mcp.example.com/events",
        "auth": {
          "type": "oauth",
          "clientId": "my-client-id",
          "authorizationUrl": "https://auth.example.com/authorize",
          "tokenUrl": "https://auth.example.com/token",
          "scopes": ["tools:read", "tools:write"]
        }
      }
    }
  }
}
```

---

## Tool Naming

Tools from MCP servers are prefixed to avoid collisions:

```
mcp__<server-name>__<tool-name>
```

Examples if your server is named `filesystem`:

```
mcp__filesystem__read_file
mcp__filesystem__write_file
mcp__filesystem__list_directory
```

The prefix is also used in `--allowedTools`:

```bash
claw --allowedTools "mcp__filesystem__read_file,mcp__filesystem__list_directory"
```

---

## Multiple Servers

Configure as many servers as you need. They all run in parallel.

```json
{
  "mcp": {
    "servers": {
      "filesystem": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-filesystem", "/home/user"]
      },
      "github": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-github"],
        "env": {
          "GITHUB_PERSONAL_ACCESS_TOKEN": "ghp_..."
        }
      },
      "memory": {
        "type": "stdio",
        "command": "npx",
        "args": ["-y", "@modelcontextprotocol/server-memory"]
      }
    }
  }
}
```

---

## Scoping

MCP server config follows the same three-level hierarchy as all config:

- **User** (`~/.claude.json`) — available in all projects
- **Project** (`<repo>/.claude.json`) — available in this repo only
- **Local** (`<repo>/.claude/settings.local.json`) — machine-local, not committed

Project-scoped servers only activate inside that repo's working directory.

---

## Common MCP Servers

Some widely used community servers:

| Server | npm package | What it provides |
|---|---|---|
| Filesystem | `@modelcontextprotocol/server-filesystem` | File read/write/list for a given path |
| GitHub | `@modelcontextprotocol/server-github` | GitHub API (repos, issues, PRs, search) |
| Memory | `@modelcontextprotocol/server-memory` | Persistent key-value memory |
| PostgreSQL | `@modelcontextprotocol/server-postgres` | Database query tool |
| Brave Search | `@modelcontextprotocol/server-brave-search` | Web search via Brave API |
| Puppeteer | `@modelcontextprotocol/server-puppeteer` | Browser automation |

Install them locally:

```bash
npm install -g @modelcontextprotocol/server-filesystem
```

Or use `npx -y` in the config to install on first run (as shown in the examples above).

---

## Debugging MCP Connections

Check that your servers connected and their tools were discovered:

```
/config mcp
```

This shows each configured server, its transport type, and whether it connected successfully. Each connected server's tool list will be visible in the agent's available tools.
