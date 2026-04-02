mod error;
mod routes;
mod state;
mod ws;

use std::net::SocketAddr;

use axum::Router;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

use crate::routes::build_routes;
use crate::state::AppState;

/// Start the web server on the given address.
///
/// # Errors
///
/// Returns an error if the server fails to bind or serve.
pub async fn serve(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let state = AppState::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = build_routes(state)
        .layer(cors)
        .layer(CompressionLayer::new());

    tracing::info!("web-server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Default address for the web server.
#[must_use]
pub fn default_addr() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], 3100))
}
