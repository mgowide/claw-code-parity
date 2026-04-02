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
            // For now, echo back while we wire up the real runtime.
            // Phase 2+ will connect to ConversationRuntime here.
            let _ = tx.send(ServerEvent::TextDelta {
                text: format!("Echo: {text}"),
            });
            let _ = tx.send(ServerEvent::TurnComplete { turn_index: 0 });
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
        ClientCommand::ApprovePermission { .. }
        | ClientCommand::DenyPermission { .. }
        | ClientCommand::Compact { .. }
        | ClientCommand::ResumeSession { .. } => {
            // Stub — will be implemented in later phases
            let _ = tx.send(ServerEvent::Error {
                message: "not yet implemented".to_string(),
            });
        }
    }
}
