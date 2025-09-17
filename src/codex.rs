use crate::types::*;
use anyhow::{Context, Result, anyhow};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::{Mutex, mpsc};

#[derive(Debug)]
pub struct CodexClient {
  config: CodexConfig,
}

impl CodexClient {
  pub fn new(config: CodexConfig) -> Self {
    Self { config }
  }

  pub async fn start_prompt_streaming(&self, request: CodexPromptRequest, message_tx: mpsc::UnboundedSender<CodexMessage>) -> Result<()> {
    let mut args = self.assemble_args(request.clone());
    let prompt = self.assemble_prompt(request.clone());
    args.push(prompt);

    let mut child = Command::new(&self.config.binary)
      .args(&args)
      .stdin(Stdio::null())
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .spawn()
      .context("Failed to spawn codex process")?;

    let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to capture stdout"))?;
    let stderr = child.stderr.take().ok_or_else(|| anyhow!("Failed to capture stderr"))?;

    let child = Arc::new(Mutex::new(child));
    let timeout_ms = request.timeout.unwrap_or(self.config.timeout_ms);

    tokio::spawn({
      let message_tx = message_tx.clone();

      async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
          if let Ok(msg) = serde_json::from_str::<CodexMessage>(&line) {
            let _ = message_tx.send(msg.clone());
          }
        }
      }
    });

    // Spawn stderr reader
    tokio::spawn(async move {
      let reader = BufReader::new(stderr);
      let mut lines = reader.lines();
      let mut error_buffer = String::new();

      while let Ok(Some(line)) = lines.next_line().await {
        error_buffer.push_str(&line);
        error_buffer.push('\n');
      }

      if !error_buffer.is_empty() {
        tracing::error!("Codex stderr: {}", error_buffer);
      }
    });

    // Wait to die
    tokio::spawn({
      let child = child.clone();

      async move {
        tokio::time::sleep(Duration::from_millis(timeout_ms)).await;

        let mut child = child.lock().await;
        let _ = child.kill().await;
      }
    });

    tokio::spawn({
      let child = child.clone();

      async move {
        loop {
          {
            match child.lock().await.try_wait() {
              Ok(Some(exit_status)) => {
                if !exit_status.success() {
                  let _ = message_tx.send(CodexMessage {
                    msg: Some(MessageContent {
                      msg_type: MessageType::Error,
                      text: Some(format!("Codex process exited with status: {}", exit_status)),
                      ..Default::default()
                    }),
                    ..Default::default()
                  });
                }

                break;
              }
              Ok(None) => {
                tokio::time::sleep(Duration::from_millis(100)).await;
              }
              Err(e) => {
                let _ = message_tx.send(CodexMessage {
                  msg: Some(MessageContent { msg_type: MessageType::Error, text: Some(e.to_string()), ..Default::default() }),
                  ..Default::default()
                });
              }
            }
          }
        }
      }
    });

    Ok(())
  }

  fn assemble_args(&self, request: CodexPromptRequest) -> Vec<String> {
    let mut args = vec!["exec".to_string(), "--json".to_string()];

    let sandbox_mode = if let Some(sandbox_mode) = request.sandbox_mode {
      sandbox_mode.to_string()
    } else if let Some(sandbox_mode) = self.config.sandbox_mode.clone() {
      sandbox_mode.to_string()
    } else {
      SandboxMode::ReadOnly.to_string()
    };

    args.push("--sandbox".to_string());
    args.push(sandbox_mode);

    let model = if let Some(model) = request.model {
      model.to_string()
    } else if let Some(model) = self.config.model.clone() {
      model.to_string()
    } else {
      Model::Gpt5Codex.to_string()
    };

    args.push("--model".to_string());
    args.push(model);

    let reasoning_effort = if let Some(reasoning_effort) = request.reasoning_effort {
      reasoning_effort.to_string()
    } else if let Some(reasoning_effort) = self.config.reasoning_effort.clone() {
      reasoning_effort.to_string()
    } else {
      ReasoningEffort::Medium.to_string()
    };

    args.push("--config".to_string());
    args.push(format!("model_reasoning_effort={}", reasoning_effort));

    let working_dir = request.context.as_ref().and_then(|c| c.working_dir.clone()).unwrap_or(".".to_string());
    args.push("--cd".to_string());
    args.push(working_dir.clone());

    args
  }

  fn assemble_prompt(&self, request: CodexPromptRequest) -> String {
    use std::fmt::Write;

    let mut prompt = String::new();

    if let Some(context) = &request.context {
      writeln!(&mut prompt, "# Context").unwrap();
      writeln!(&mut prompt).unwrap();

      if let Some(working_dir) = &context.working_dir {
        writeln!(&mut prompt, "Working directory: {}", working_dir).unwrap();
        writeln!(&mut prompt).unwrap();
      }

      if let Some(files) = &context.files {
        writeln!(&mut prompt, "## Files").unwrap();
        writeln!(&mut prompt).unwrap();

        for file in files {
          write!(&mut prompt, "{}", file.path).unwrap();

          if let Some(content) = &file.content {
            writeln!(&mut prompt, ":").unwrap();
            writeln!(&mut prompt, "{}", content).unwrap();
            writeln!(&mut prompt).unwrap();
          } else {
            writeln!(&mut prompt).unwrap();
          }
        }

        writeln!(&mut prompt).unwrap();
      }

      if let Some(variables) = &context.variables {
        writeln!(&mut prompt, "## Variables").unwrap();
        writeln!(&mut prompt).unwrap();

        for (key, value) in variables {
          writeln!(&mut prompt, "{}: {}", key, value).unwrap();
        }

        writeln!(&mut prompt).unwrap();
      }
    }

    writeln!(&mut prompt, "## User Prompt").unwrap();
    writeln!(&mut prompt).unwrap();
    prompt.push_str(&request.prompt);

    prompt
  }
}
