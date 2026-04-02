use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::broadcast;

use crate::ws::events::ServerEvent;

/// Handle to an active session including its event broadcast channel.
pub struct SessionHandle {
    pub session_id: String,
    pub model: String,
    pub created_at: String,
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
