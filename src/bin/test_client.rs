use anyhow::Result;
use rmcp::{
  ServiceExt,
  model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation},
  transport::StreamableHttpClientTransport,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
  // Initialize logging
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| format!("info,{}=debug", env!("CARGO_CRATE_NAME")).into()))
    .with(tracing_subscriber::fmt::layer())
    .init();

  let transport = StreamableHttpClientTransport::from_uri("http://localhost:9871/mcp");
  let client_info = ClientInfo {
    protocol_version: Default::default(),
    capabilities: ClientCapabilities::default(),
    client_info: Implementation { name: "test client".to_string(), title: None, version: "0.0.1".to_string(), website_url: None, icons: None },
  };

  let client = client_info.serve(transport).await.inspect_err(|e| {
    tracing::error!("client error: {:?}", e);
  })?;

  // Initialize
  let server_info = client.peer_info();
  tracing::info!("Connected to server: {server_info:#?}");

  // List tools
  let tools = client.list_tools(Default::default()).await?;
  tracing::info!("Available tools: {tools:#?}");

  let tool_result = client
    .call_tool(CallToolRequestParam {
      name: "prompt".into(),
      arguments: serde_json::json!({
        "prompt": "Analyze the Rust MCP server implementation and suggest architectural improvements. Focus on error handling patterns, async/concurrent processing optimizations, and protocol compliance. Consider the existing codex integration and how it could be made more robust.",
        "context": {
          "working_dir": "/Users/supagoku/projects/codecho",
          "files": [
            {
              "path": "/Users/supagoku/projects/codecho/src/main.rs"
            },
            {
              "path": "/Users/supagoku/projects/codecho/src/server.rs"
            },
            {
              "path": "/Users/supagoku/projects/codecho/src/codex.rs"
            },
            {
              "path": "/Users/supagoku/projects/codecho/src/types.rs"
            },
            {
              "path": "/Users/supagoku/projects/codecho/Cargo.toml"
            }
          ],
          "variables": {
            "project_type": "rust",
            "protocol_version": "mcp-1.0.0",
            "transport": "streamable-http",
            "max_timeout_ms": "1800000",
            "default_model": "gpt-5-codex",
            "deployment_target": "macos-arm64",
            "rust_edition": "2024"
          }
        }
      })
      .as_object()
      .cloned(),
    })
    .await?;

  tracing::info!("Tool result: {tool_result:#?}");

  client.cancel().await?;

  Ok(())
}
