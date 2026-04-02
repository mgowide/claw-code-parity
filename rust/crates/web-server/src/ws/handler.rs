use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::stream::StreamExt;
use futures::SinkExt;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::state::{AppState, SessionHandle};
use crate::ws::commands::ClientCommand;
use crate::ws::events::ServerEvent;

/// Upgrade an HTTP request to a WebSocket connection.
pub async fn ws_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    // Create a default session for this connection
    let session_id = Uuid::new_v4().to_string();
    let model = state.model.clone();
    let (tx, mut rx) = broadcast::channel::<ServerEvent>(256);

    let handle = SessionHandle {
        session_id: session_id.clone(),
        model: model.clone(),
        created_at: String::new(),
        tx: tx.clone(),
    };
    state.sessions.insert(session_id.clone(), handle);

    // Send Connected event
    let connected = ServerEvent::Connected {
        session_id: session_id.clone(),
        model: model.clone(),
    };
    if let Ok(json) = serde_json::to_string(&connected) {
        let _ = sender.send(Message::Text(json.into())).await;
    }

    // Spawn a task to forward broadcast events to the WebSocket sender
    let write_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&event) {
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Read incoming messages from the client
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => {
                if let Ok(cmd) = serde_json::from_str::<ClientCommand>(&text) {
                    handle_command(&state, &tx, &session_id, cmd).await;
                } else {
                    let err = ServerEvent::Error {
                        message: "invalid command format".to_string(),
                    };
                    if let Ok(json) = serde_json::to_string(&err) {
                        let _ = tx.send(err);
                        tracing::warn!("bad client message: {json}");
                    }
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Clean up
    write_task.abort();
    state.sessions.remove(&session_id);
    tracing::info!("client disconnected, session {session_id} removed");
}

async fn handle_command(
    _state: &Arc<AppState>,
    tx: &broadcast::Sender<ServerEvent>,
    session_id: &str,
    cmd: ClientCommand,
) {
    match cmd {
        ClientCommand::SendMessage { text, .. } => {
            // Simulate a realistic turn with tool calls for frontend testing.
            // Phase 3+ will wire this to ConversationRuntime.
            simulate_turn(tx, &text).await;
        }
        ClientCommand::CancelTurn { .. } => {
            let _ = tx.send(ServerEvent::TurnComplete { turn_index: 0 });
        }
        ClientCommand::SwitchModel { model } => {
            let _ = tx.send(ServerEvent::Connected {
                session_id: session_id.to_string(),
                model,
            });
        }
        ClientCommand::ApprovePermission { request_id } => {
            // Simulate completing a tool after approval
            let _ = tx.send(ServerEvent::ToolResult {
                id: request_id,
                output: "Permission granted — operation completed.".to_string(),
                is_error: false,
            });
        }
        ClientCommand::DenyPermission { request_id } => {
            let _ = tx.send(ServerEvent::ToolResult {
                id: request_id,
                output: "Permission denied by user.".to_string(),
                is_error: true,
            });
        }
        ClientCommand::Compact { .. } => {
            let _ = tx.send(ServerEvent::SessionCompacted {
                removed_messages: 5,
            });
        }
        ClientCommand::ResumeSession { .. } => {
            let _ = tx.send(ServerEvent::Error {
                message: "session resume not yet implemented".to_string(),
            });
        }
    }
}

/// Simulates a multi-step assistant turn with tool calls.
///
/// Depending on the user message, different tool scenarios are demonstrated:
/// - Messages containing "edit" or "write" → file edit with diff
/// - Messages containing "run" or "bash" → bash tool with terminal output
/// - Messages containing "read" or "search" → read_file tool with code output
/// - Messages containing "permission" or "danger" → permission request flow
/// - Default → thinking + text response with a read_file tool call
async fn simulate_turn(tx: &broadcast::Sender<ServerEvent>, text: &str) {
    let lower = text.to_lowercase();

    // Start with thinking
    let _ = tx.send(ServerEvent::ThinkingStart {});
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    let _ = tx.send(ServerEvent::ThinkingEnd {});

    if lower.contains("edit") || lower.contains("write") {
        // Simulate file edit with diff
        let tool_id = Uuid::new_v4().to_string();
        let _ = tx.send(ServerEvent::ToolUseStart {
            id: tool_id.clone(),
            name: "edit_file".to_string(),
            input: serde_json::json!({
                "path": "src/main.rs",
                "old_string": "fn main() {\n    println!(\"Hello\");\n}",
                "new_string": "fn main() {\n    println!(\"Hello, world!\");\n}"
            }),
        });
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        let _ = tx.send(ServerEvent::Diff {
            path: "src/main.rs".to_string(),
            old_content: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
            new_content: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
        });

        let _ = tx.send(ServerEvent::ToolResult {
            id: tool_id,
            output: "File edited successfully.".to_string(),
            is_error: false,
        });

        let _ = tx.send(ServerEvent::TextDelta {
            text: "I've updated `src/main.rs` to print \"Hello, world!\" instead of \"Hello\".".to_string(),
        });
    } else if lower.contains("run") || lower.contains("bash") {
        // Simulate bash tool with terminal output
        let tool_id = Uuid::new_v4().to_string();
        let _ = tx.send(ServerEvent::ToolUseStart {
            id: tool_id.clone(),
            name: "bash".to_string(),
            input: serde_json::json!({ "command": "cargo test --workspace" }),
        });
        tokio::time::sleep(std::time::Duration::from_millis(400)).await;

        let _ = tx.send(ServerEvent::ToolResult {
            id: tool_id,
            output: "\x1b[32mCompiling\x1b[0m claw v0.1.0\n\x1b[32m  Running\x1b[0m unittests src/main.rs\n\nrunning 3 tests\ntest tests::test_parse ... \x1b[32mok\x1b[0m\ntest tests::test_format ... \x1b[32mok\x1b[0m\ntest tests::test_validate ... \x1b[32mok\x1b[0m\n\ntest result: \x1b[32mok\x1b[0m. 3 passed; 0 failed\n".to_string(),
            is_error: false,
        });

        let _ = tx.send(ServerEvent::TextDelta {
            text: "All 3 tests passed successfully.".to_string(),
        });
    } else if lower.contains("read") || lower.contains("search") {
        // Simulate read_file tool with code
        let tool_id = Uuid::new_v4().to_string();
        let _ = tx.send(ServerEvent::ToolUseStart {
            id: tool_id.clone(),
            name: "read_file".to_string(),
            input: serde_json::json!({ "path": "src/lib.rs", "start_line": 1, "end_line": 20 }),
        });
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        let _ = tx.send(ServerEvent::ToolResult {
            id: tool_id,
            output: "use std::collections::HashMap;\nuse serde::{Deserialize, Serialize};\n\n/// Core configuration for the application.\n#[derive(Debug, Clone, Serialize, Deserialize)]\npub struct Config {\n    pub model: String,\n    pub max_tokens: u64,\n    pub temperature: f64,\n    pub tools: Vec<String>,\n}\n\nimpl Default for Config {\n    fn default() -> Self {\n        Self {\n            model: \"claude-sonnet-4-20250514\".to_string(),\n            max_tokens: 8192,\n            temperature: 0.7,\n            tools: vec![],\n        }\n    }\n}".to_string(),
            is_error: false,
        });

        let _ = tx.send(ServerEvent::TextDelta {
            text: "Here's the content of `src/lib.rs`. It defines a `Config` struct with model, token, and temperature settings.".to_string(),
        });
    } else if lower.contains("permission") || lower.contains("danger") {
        // Simulate a permission request
        let perm_id = Uuid::new_v4().to_string();
        let _ = tx.send(ServerEvent::TextDelta {
            text: "I need to run a destructive command. Requesting permission...".to_string(),
        });

        let _ = tx.send(ServerEvent::PermissionRequest {
            id: perm_id.clone(),
            tool: "bash".to_string(),
            description: "Execute: rm -rf target/ && cargo build --release".to_string(),
        });

        // The turn doesn't complete here — it waits for ApprovePermission/DenyPermission
        return;
    } else {
        // Default: simple tool call + text response
        let tool_id = Uuid::new_v4().to_string();
        let _ = tx.send(ServerEvent::ToolUseStart {
            id: tool_id.clone(),
            name: "read_file".to_string(),
            input: serde_json::json!({ "path": "Cargo.toml" }),
        });
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let _ = tx.send(ServerEvent::ToolResult {
            id: tool_id,
            output: "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"2\"\n".to_string(),
            is_error: false,
        });

        let _ = tx.send(ServerEvent::TextDelta {
            text: format!("I received your message: \"{text}\". This is a simulated response with a tool call demonstration."),
        });
    }

    // Usage and turn complete
    let _ = tx.send(ServerEvent::Usage {
        input_tokens: 1250,
        output_tokens: 340,
        cache_hits: 800,
        cost: 0.0042,
    });
    let _ = tx.send(ServerEvent::TurnComplete { turn_index: 0 });
}
