use axum::Json;

/// `GET /api/tools` — list all built-in tools with descriptions and permission levels.
pub async fn list_tools() -> Json<serde_json::Value> {
    let tools = serde_json::json!([
        { "name": "bash",            "description": "Execute a shell command",                        "permission": "danger", "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "read_file",       "description": "Read file contents with optional line range",    "permission": "read",   "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "write_file",      "description": "Write content to a file",                       "permission": "write",  "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "edit_file",       "description": "Edit file with old/new string replacement",     "permission": "write",  "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "multi_edit",      "description": "Apply multiple edits to a file at once",        "permission": "write",  "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "list_dir",        "description": "List directory contents",                       "permission": "read",   "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "file_search",     "description": "Search files by glob pattern",                  "permission": "read",   "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "grep_search",     "description": "Search file contents via regex",                "permission": "read",   "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "semantic_search", "description": "Natural language semantic code search",         "permission": "read",   "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "web_search",      "description": "Search the web for information",               "permission": "network","source": "builtin", "call_count": 0, "enabled": true },
        { "name": "fetch_webpage",   "description": "Fetch and parse a web page",                   "permission": "network","source": "builtin", "call_count": 0, "enabled": true },
        { "name": "todo_write",      "description": "Create or update a todo list",                  "permission": "write",  "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "todo_read",       "description": "Read the current todo list",                    "permission": "read",   "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "task",            "description": "Spawn a sub-agent task",                        "permission": "danger", "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "exit_plan_mode",  "description": "Exit plan mode and confirm plan",               "permission": "read",   "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "create_dir",      "description": "Create a directory",                            "permission": "write",  "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "move_file",       "description": "Move or rename a file",                         "permission": "write",  "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "delete_file",     "description": "Delete a file",                                "permission": "danger", "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "apply_patch",     "description": "Apply a unified diff patch",                   "permission": "write",  "source": "builtin", "call_count": 0, "enabled": true },
        { "name": "run_in_terminal", "description": "Run a command in a persistent terminal",       "permission": "danger", "source": "builtin", "call_count": 0, "enabled": true },
    ]);

    let arr = tools.as_array().unwrap();
    let total = arr.len();
    let builtin = arr.iter().filter(|t| t["source"] == "builtin").count();

    Json(serde_json::json!({
        "tools": tools,
        "total": total,
        "builtin": builtin,
        "plugin": total - builtin,
    }))
}
