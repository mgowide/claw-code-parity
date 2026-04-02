use serde::Serialize;

/// Events sent from the server to the client over WebSocket.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEvent {
    TextDelta {
        text: String,
    },
    ThinkingStart {},
    ThinkingEnd {},
    ToolUseStart {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    ToolResult {
        id: String,
        output: String,
        is_error: bool,
    },
    Diff {
        path: String,
        old_content: String,
        new_content: String,
    },
    Usage {
        input_tokens: u64,
        output_tokens: u64,
        cache_hits: u64,
        cost: f64,
    },
    TurnComplete {
        turn_index: u64,
    },
    PermissionRequest {
        id: String,
        tool: String,
        description: String,
    },
    SessionCompacted {
        removed_messages: u64,
    },
    Error {
        message: String,
    },
    Connected {
        session_id: String,
        model: String,
    },
}
