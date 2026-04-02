use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::stream::StreamExt;
use futures::SinkExt;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::state::{AppState, SessionHandle, SessionUsage, StoredMessage};
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

    // Create a session for this connection
    let session_id = Uuid::new_v4().to_string();
    let model = state.model.clone();
    let (tx, mut rx) = broadcast::channel::<ServerEvent>(256);
    let now = unix_now();

    let handle = SessionHandle {
        session_id: session_id.clone(),
        name: "New chat".to_string(),
        model: model.clone(),
        created_at: now.to_string(),
        updated_at: now.to_string(),
        messages: Vec::new(),
        usage: SessionUsage::default(),
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

    // Clean up: keep session in memory so it appears in the list,
    // but drop the broadcast sender so old subscribers notice it's gone.
    if let Some(mut entry) = state.sessions.get_mut(&session_id) {
        entry.updated_at = unix_now().to_string();
    }
    tracing::info!("client disconnected, session {session_id} retained in list");
}

async fn handle_command(
    state: &Arc<AppState>,
    tx: &broadcast::Sender<ServerEvent>,
    session_id: &str,
    cmd: ClientCommand,
) {
    match cmd {
        ClientCommand::SendMessage { text, attachments, .. } => {
            // Build full user content — append file attachments as XML context blocks
            let full_text = if attachments.is_empty() {
                text.clone()
            } else {
                let mut s = text.clone();
                for att in &attachments {
                    s.push_str(&format!("\n\n<file name=\"{}\">{}</file>", att.name, att.content));
                }
                s
            };

            // Persist user message and auto-name session from first message
            if let Some(mut entry) = state.sessions.get_mut(session_id) {
                if entry.messages.is_empty() {
                    let name = full_text.chars().take(50).collect::<String>();
                    entry.name = name.trim().to_string();
                }
                entry.messages.push(StoredMessage {
                    id: Uuid::new_v4().to_string(),
                    role: "user".to_string(),
                    content: full_text.clone(),
                    timestamp: unix_now(),
                });
                entry.updated_at = unix_now().to_string();
            }

            // Simulate a realistic turn with tool calls for frontend testing.
            // Phase 4+ will wire this to ConversationRuntime.
            let response = simulate_turn(tx, &full_text).await;

            // Persist assistant response
            if let Some(mut entry) = state.sessions.get_mut(session_id) {
                entry.messages.push(StoredMessage {
                    id: Uuid::new_v4().to_string(),
                    role: "assistant".to_string(),
                    content: response,
                    timestamp: unix_now(),
                });
                // Accumulate usage
                entry.usage.input_tokens += 1250;
                entry.usage.output_tokens += 340;
                entry.usage.cache_hits += 800;
                entry.usage.cost += 0.0042;
                entry.updated_at = unix_now().to_string();
            }
        }
        ClientCommand::CancelTurn { .. } => {
            let _ = tx.send(ServerEvent::TurnComplete { turn_index: 0 });
        }
        ClientCommand::SwitchModel { model } => {
            if let Some(mut entry) = state.sessions.get_mut(session_id) {
                entry.model = model.clone();
            }
            let _ = tx.send(ServerEvent::Connected {
                session_id: session_id.to_string(),
                model,
            });
        }
        ClientCommand::ApprovePermission { request_id } => {
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
/// Returns the primary text response for transcript storage.
///
/// Depending on the user message, different tool scenarios are demonstrated:
/// - Messages containing "edit" or "write" → file edit with diff
/// - Messages containing "run" or "bash" → bash tool with terminal output
/// - Messages containing "read" or "search" → read_file tool with code output
/// - Messages containing "permission" or "danger" → permission request flow
/// - Default → thinking + text response with a read_file tool call
async fn simulate_turn(tx: &broadcast::Sender<ServerEvent>, text: &str) -> String {
    let lower = text.to_lowercase();

    // Start with thinking
    let _ = tx.send(ServerEvent::ThinkingStart {});
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    let _ = tx.send(ServerEvent::ThinkingEnd {});

    let response_text;

    if lower.contains("edit") || lower.contains("write") {
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
        response_text = "I've updated `src/main.rs` to print \"Hello, world!\" instead of \"Hello\".".to_string();
    } else if lower.contains("run") || lower.contains("bash") {
        let tool_id = Uuid::new_v4().to_string();
        let _ = tx.send(ServerEvent::ToolUseStart {
            id: tool_id.clone(),
            name: "bash".to_string(),
            input: serde_json::json!({ "command": "cargo test --workspace" }),
        });
        tokio::time::sleep(std::time::Duration::from_millis(400)).await;
        let _ = tx.send(ServerEvent::ToolResult {
            id: tool_id,
            output: "\x1b[32mCompiling\x1b[0m claw v0.1.0\n\ntest result: \x1b[32mok\x1b[0m. 3 passed; 0 failed\n".to_string(),
            is_error: false,
        });
        response_text = "All 3 tests passed successfully.".to_string();
    } else if lower.contains("read") || lower.contains("search") {
        let tool_id = Uuid::new_v4().to_string();
        let _ = tx.send(ServerEvent::ToolUseStart {
            id: tool_id.clone(),
            name: "read_file".to_string(),
            input: serde_json::json!({ "path": "src/lib.rs", "start_line": 1, "end_line": 20 }),
        });
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        let _ = tx.send(ServerEvent::ToolResult {
            id: tool_id,
            output: "pub struct Config {\n    pub model: String,\n    pub max_tokens: u64,\n}".to_string(),
            is_error: false,
        });
        response_text = "Here's the content of `src/lib.rs`. It defines a `Config` struct with model and token settings.".to_string();
    } else if lower.contains("permission") || lower.contains("danger") {
        let perm_id = Uuid::new_v4().to_string();
        let intro = "I need to run a destructive command. Requesting permission...".to_string();
        let _ = tx.send(ServerEvent::TextDelta { text: intro.clone() });
        let _ = tx.send(ServerEvent::PermissionRequest {
            id: perm_id,
            tool: "bash".to_string(),
            description: "Execute: rm -rf target/ && cargo build --release".to_string(),
        });
        // Turn remains open — waiting for approve/deny
        return intro;
    } else {
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
        response_text = format!("I received your message: \"{text}\". This is a simulated response with a tool call demonstration.");
    }

    let _ = tx.send(ServerEvent::TextDelta { text: response_text.clone() });
    let _ = tx.send(ServerEvent::Usage {
        input_tokens: 1250,
        output_tokens: 340,
        cache_hits: 800,
        cost: 0.0042,
    });
    let _ = tx.send(ServerEvent::TurnComplete { turn_index: 0 });

    response_text
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
