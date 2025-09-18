use std::{
  collections::HashMap,
  env,
  fmt::{self, Display},
  str::FromStr,
};

use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, schemars::JsonSchema)]
pub enum ReasoningEffort {
  #[serde(rename = "low")]
  #[schemars(rename = "low")]
  Low,
  #[default]
  #[serde(rename = "medium")]
  #[schemars(rename = "medium")]
  Medium,
  #[serde(rename = "high")]
  #[schemars(rename = "high")]
  High,
}

impl FromStr for ReasoningEffort {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "low" => Self::Low,
      "medium" => Self::Medium,
      "high" => Self::High,
      _ => Self::Medium,
    })
  }
}

impl Display for ReasoningEffort {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Low => write!(f, "low"),
      Self::Medium => write!(f, "medium"),
      Self::High => write!(f, "high"),
    }
  }
}

#[derive(Debug, Default, Clone, Deserialize, schemars::JsonSchema)]
pub enum Model {
  #[default]
  #[serde(rename = "gpt-5-codex")]
  #[schemars(rename = "gpt-5-codex")]
  Gpt5Codex,
  #[serde(rename = "gpt-5")]
  #[schemars(rename = "gpt-5")]
  Gpt5,
}

impl Display for Model {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Gpt5Codex => write!(f, "gpt-5-codex"),
      Self::Gpt5 => write!(f, "gpt-5"),
    }
  }
}

impl FromStr for Model {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "gpt-5-codex" => Self::Gpt5Codex,
      "gpt-5" => Self::Gpt5,
      _ => Self::Gpt5Codex,
    })
  }
}

#[derive(Debug, Default, Clone, Deserialize, schemars::JsonSchema)]
pub enum SandboxMode {
  #[default]
  #[serde(rename = "read-only")]
  #[schemars(rename = "read-only")]
  ReadOnly,
  #[serde(rename = "workspace-write")]
  #[schemars(rename = "workspace-write")]
  WorkspaceWrite,
  #[serde(rename = "danger-full-access")]
  #[schemars(rename = "danger-full-access")]
  DangerFullAccess,
}

impl Display for SandboxMode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ReadOnly => write!(f, "read-only"),
      Self::WorkspaceWrite => write!(f, "workspace-write"),
      Self::DangerFullAccess => write!(f, "danger-full-access"),
    }
  }
}

impl FromStr for SandboxMode {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "read-only" => Self::ReadOnly,
      "workspace-write" => Self::WorkspaceWrite,
      "danger-full-access" => Self::DangerFullAccess,
      _ => Self::ReadOnly,
    })
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CodexConfig {
  pub binary: String,
  pub timeout_ms: u64,
  pub model: Option<Model>,
  pub sandbox_mode: Option<SandboxMode>,
  pub reasoning_effort: Option<ReasoningEffort>,
}

impl Default for CodexConfig {
  fn default() -> Self {
    Self { binary: "codex".to_string(), model: None, sandbox_mode: None, reasoning_effort: None, timeout_ms: 1800000 }
  }
}

impl CodexConfig {
  pub fn from_env() -> Self {
    let model = env::var("CODEX_MODEL").ok().and_then(|s| s.parse::<Model>().ok());
    let sandbox_mode = env::var("CODEX_SANDBOX_MODE").ok().and_then(|s| s.parse::<SandboxMode>().ok());
    let reasoning_effort = env::var("CODEX_REASONING_EFFORT").ok().and_then(|s| s.parse::<ReasoningEffort>().ok());

    Self {
      binary: env::var("CODEX_BINARY").unwrap_or_else(|_| "codex".to_string()),
      model,
      sandbox_mode,
      reasoning_effort,
      timeout_ms: env::var("CODEX_TIMEOUT").ok().and_then(|s| s.parse::<u64>().ok()).unwrap_or(1800000),
    }
  }
}

#[derive(Debug, Default, Clone, Deserialize, schemars::JsonSchema)]
pub struct CodexPromptRequest {
  #[schemars(description = "The prompt to execute")]
  pub prompt: String,
  #[schemars(description = "The context/working directory to execute the prompt in")]
  pub context: Option<Context>,
  #[schemars(description = "The timeout for the prompt")]
  pub timeout: Option<u64>,
  #[schemars(description = "The model to use")]
  pub model: Option<Model>,
  #[schemars(description = "The reasoning effort to use")]
  pub reasoning_effort: Option<ReasoningEffort>,
  #[schemars(description = "The sandbox mode to use")]
  pub sandbox_mode: Option<SandboxMode>,
}

#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub struct Context {
  #[schemars(description = "The files to include in the context")]
  pub files: Option<Vec<ContextFile>>,
  #[schemars(description = "The variables to include in the context")]
  pub variables: Option<HashMap<String, String>>,
  #[schemars(description = "The working directory to include in the context")]
  pub working_dir: Option<String>,
}

#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub struct ContextFile {
  #[schemars(description = "The path to the file")]
  pub path: String,
  #[schemars(description = "The content of the file")]
  pub content: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CodexMessage {
  pub id: Option<String>,
  pub msg: Option<MessageContent>,
  pub prompt: Option<String>,
  pub model: Option<String>,
  pub sandbox: Option<String>,
  pub provider: Option<String>,
  pub reasoning: Option<String>,
  pub workdir: Option<String>,
  pub approval: Option<String>,
  #[serde(rename = "reasoning effort")]
  pub reasoning_effort: Option<String>,
  #[serde(rename = "reasoning summaries")]
  pub reasoning_summaries: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct MessageContent {
  #[serde(rename = "type")]
  pub msg_type: MessageType,
  pub message: Option<String>,
  pub text: Option<String>,
  pub info: Option<TokenUsageInfo>,
  pub model_context_window: Option<u64>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
  TaskStarted,
  AgentReasoning,
  #[default]
  AgentMessage,
  TokenCount,
  AgentReasoningSectionBreak,
  Error,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenUsageInfo {
  pub total_token_usage: Option<TokenUsage>,
  pub last_token_usage: Option<TokenUsage>,
  pub model_context_window: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenUsage {
  pub input_tokens: u64,
  pub cached_input_tokens: Option<u64>,
  pub output_tokens: u64,
  pub reasoning_output_tokens: Option<u64>,
  pub total_tokens: u64,
}
