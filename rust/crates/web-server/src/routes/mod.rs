mod config;
mod files;
mod health;
mod mcp;
mod sessions;
mod tools;

use std::sync::Arc;

use axum::Router;
use axum::routing::{delete, get, post, put};

use crate::state::AppState;
use crate::ws::handler::ws_upgrade;

pub fn build_routes(state: Arc<AppState>) -> Router {
    Router::new()
        // Status
        .route("/api/status", get(health::status))
        // Sessions
        .route("/api/sessions", post(sessions::create_session))
        .route("/api/sessions", get(sessions::list_sessions))
        .route("/api/sessions/{id}", get(sessions::get_session))
        .route("/api/sessions/{id}", delete(sessions::delete_session))
        .route("/api/sessions/{id}/export", get(sessions::export_session))
        // Files
        .route("/api/files", get(files::list_files))
        .route("/api/files/read", get(files::read_file))
        // Config
        .route("/api/config", get(config::get_config))
        .route("/api/config", put(config::put_config))
        // Tools
        .route("/api/tools", get(tools::list_tools))
        // MCP
        .route("/api/mcp/servers", get(mcp::list_servers))
        .route("/api/mcp/servers", post(mcp::add_server))
        .route("/api/mcp/servers/{id}", delete(mcp::remove_server))
        // WebSocket
        .route("/ws", get(ws_upgrade))
        .with_state(state)
}
