use rmcp::{
  ErrorData as McpError, Peer, RoleServer, ServerHandler,
  handler::server::{router::tool::ToolRouter, wrapper::Parameters},
  model::{CallToolResult, Content, ErrorCode, LoggingLevel, LoggingMessageNotificationParam, ServerCapabilities, ServerInfo},
  service::RequestContext,
  tool, tool_handler, tool_router,
};
use serde_json::Value;
use tokio::sync::mpsc;

use crate::{
  codex::CodexClient,
  types::{CodexConfig, CodexMessage, CodexPromptRequest, MessageType},
};

#[derive(Debug)]
pub struct CodexMcp {
  tool_router: ToolRouter<Self>,
  codex_client: CodexClient,
}

#[tool_router]
impl CodexMcp {
  pub fn new() -> Self {
    let codex_config = CodexConfig::from_env();

    Self { tool_router: Self::tool_router(), codex_client: CodexClient::new(codex_config) }
  }

  #[tool(description = "Execute a prompt using codex-cli")]
  async fn prompt(&self, ctx: RequestContext<RoleServer>, Parameters(params): Parameters<CodexPromptRequest>) -> Result<CallToolResult, McpError> {
    let (msg_tx, mut msg_rx) = mpsc::unbounded_channel();

    let _ = ctx
      .peer
      .notify_logging_message(LoggingMessageNotificationParam {
        level: LoggingLevel::Info,
        logger: Some("codex.prompt".to_string()),
        data: "Starting execution...".into(),
      })
      .await;

    self.codex_client.start_prompt_streaming(params, msg_tx).await.map_err(|e| McpError {
      code: ErrorCode::INTERNAL_ERROR,
      message: e.to_string().into(),
      data: None,
    })?;

    let agent_message = self.get_agent_message(ctx, &mut msg_rx).await;

    Ok(CallToolResult::success(vec![Content::text(agent_message)]))
  }
}

impl CodexMcp {
  async fn get_agent_message(&self, ctx: RequestContext<RoleServer>, msg_rx: &mut mpsc::UnboundedReceiver<CodexMessage>) -> String {
    loop {
      match msg_rx.recv().await {
        Some(CodexMessage { msg: Some(msg_content), .. }) => {
          tracing::info!("Message content: {msg_content:#?}");

          match msg_content.msg_type {
            MessageType::AgentMessage => {
              return msg_content.message.unwrap_or_default();
            }
            MessageType::AgentReasoning => self.handle_angent_reasoning(ctx.peer.clone(), msg_content.text.unwrap_or_default().into()).await,
            MessageType::TokenCount => self.handle_token_count(ctx.peer.clone(), serde_json::to_value(msg_content.info).unwrap_or_default()).await,
            MessageType::Error => self.handle_error(ctx.peer.clone(), msg_content.message.unwrap_or_default().into()).await,
            _ => {}
          }
        }
        Some(CodexMessage { msg: None, .. }) => {
          // Skip messages without content
        }
        None => {
          // Channel closed, return empty string
          return String::new();
        }
      }
    }
  }

  async fn handle_angent_reasoning(&self, peer: Peer<RoleServer>, data: Value) {
    self.send_logging_message(peer, LoggingLevel::Info, Some("codex.reasoning".to_string()), data).await;
  }

  async fn handle_error(&self, peer: Peer<RoleServer>, data: Value) {
    self.send_logging_message(peer, LoggingLevel::Error, None, data).await;
  }

  async fn handle_token_count(&self, peer: Peer<RoleServer>, data: Value) {
    self.send_logging_message(peer, LoggingLevel::Debug, None, data).await;
  }

  async fn send_logging_message(&self, peer: Peer<RoleServer>, level: LoggingLevel, logger: Option<String>, data: Value) {
    let _ = peer.notify_logging_message(LoggingMessageNotificationParam { level, logger, data }).await;
  }
}

#[tool_handler]
impl ServerHandler for CodexMcp {
  fn get_info(&self) -> ServerInfo {
    ServerInfo {
      instructions: Some("A codex-cli wrapper to enable AI agent execution".into()),
      capabilities: ServerCapabilities::builder().enable_tools().build(),
      ..Default::default()
    }
  }
}
