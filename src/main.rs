mod codex;
mod server;
mod types;

use anyhow::Result;
use rmcp::transport::{StreamableHttpService, streamable_http_server::session::local::LocalSessionManager};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::server::CodexMcp;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();

  // Initialize tracing subscriber
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
    .with(tracing_subscriber::fmt::layer())
    .init();

  tracing::info!("Starting MCP server on port 9871");

  let service = StreamableHttpService::new(
    //
    || Ok(CodexMcp::new()),
    LocalSessionManager::default().into(),
    Default::default(),
  );

  let router = axum::Router::new().nest_service("/mcp", service);
  let tcp_listener = tokio::net::TcpListener::bind("localhost:9871".to_string()).await?;
  let _ = axum::serve(tcp_listener, router)
    .with_graceful_shutdown(async {
      tokio::signal::ctrl_c().await.unwrap();
    })
    .await;

  Ok(())
}
