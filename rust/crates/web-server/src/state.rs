use std::sync::Arc;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::ws::events::ServerEvent;

/// A stored message in a session transcript.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub id: String,
    pub role: String,
    pub content: String,
    pub timestamp: u64,
}

/// Cumulative usage statistics for a session.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_hits: u64,
    pub cost: f64,
}

/// Handle to an active session including its event broadcast channel and transcript.
pub struct SessionHandle {
    pub session_id: String,
    pub name: String,
    pub model: String,
    pub created_at: String,
    pub updated_at: String,
    pub messages: Vec<StoredMessage>,
    pub usage: SessionUsage,
    pub tx: broadcast::Sender<ServerEvent>,
}

/// Shared application state accessible from all handlers.
pub struct AppState {
    pub sessions: DashMap<String, SessionHandle>,
    pub model: String,
}

impl AppState {
    #[must_use]
    pub fn new() -> Arc<Self> {
        let model = std::env::var("ANTHROPIC_MODEL")
            .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string());

        Arc::new(Self {
            sessions: DashMap::new(),
            model,
        })
    }
}
