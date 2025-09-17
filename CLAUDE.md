# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

codecho - A Rust-based MCP (Model Context Protocol) server that wraps codex-cli to enable AI agent execution through a standardized protocol. The server now runs as an HTTP service on port 9871 using the rmcp transport layer and manages codex-cli subprocess execution.

## Common Development Tasks

### Build and Run

```bash
# Build the project
cargo build

# Run in release mode
cargo build --release
./target/release/codecho

# Run with environment configuration
cp .env.example .env
# Edit .env with your settings
cargo run

# Format code
cargo fmt

# Check for compilation errors without building
cargo check

# Run clippy linter
cargo clippy -- -D warnings

# Build and run the test client (requires test_client feature)
cargo build --features test_client
cargo run --bin test_client --features test_client
```

### Testing

```bash
# Run all tests
cargo test

# Test a specific module
cargo test codex::
cargo test server::
cargo test types::
```

## Architecture

### Core Module Structure

The codebase follows a modular architecture with four main components:

1. **types module** (`src/types.rs`): Defines all data structures for both Codex CLI interaction and MCP protocol communication. Key types include:

   - `CodexConfig`: Configuration for the codex CLI binary with environment variable support
   - `CodexMessage`: Messages received from codex-cli's JSON output
   - `CodexPromptRequest`: Request structure for prompt execution with context support
   - `ReasoningEffort`, `Model`, `SandboxMode`: Enums for codex configuration options

2. **codex module** (`src/codex.rs`): Manages subprocess execution of codex-cli:

   - `CodexClient`: Main client struct that handles process spawning
   - Spawns codex-cli as a child process with `--json` flag for structured output
   - Parses JSON messages from stdout line by line via streaming channels
   - Implements timeout handling with configurable duration
   - Concurrent stdout/stderr processing using Tokio tasks
   - Automatic process cleanup on timeout or completion

3. **server module** (`src/server.rs`): Implements the MCP server using rmcp:

   - `CodexMcp`: Server handler implementation with tool router
   - Exposes a single tool: `prompt` for executing codex commands
   - Handles message streaming and filtering by type
   - Sends logging notifications for reasoning, errors, and token usage
   - Uses the `tool_router` and `tool_handler` macros from rmcp

4. **main module** (`src/main.rs`): Application entry point:
   - Sets up HTTP server on port 9871
   - Configures rmcp's StreamableHttpService with LocalSessionManager
   - Implements graceful shutdown on Ctrl+C
   - Initializes tracing/logging infrastructure

### Key Design Patterns

- **Async/Concurrent Processing**: Uses Tokio for async runtime with concurrent stdout/stderr handling
- **Message Streaming**: Unbounded mpsc channels for real-time message passing
- **Structured Logging**: JSON line parsing from codex-cli output enables real-time message processing
- **Timeout Management**: All codex executions have configurable timeouts (default 30 minutes)
- **Process Lifecycle**: Child processes are tracked using Arc<Mutex<>> for safe concurrent access
- **MCP Tool Routing**: Uses rmcp's macro-based tool routing for clean handler implementation

### Environment Configuration

The server reads configuration from environment variables (or `.env` file):

- `CODEX_BINARY`: Path to codex CLI (defaults to 'codex' in PATH)
- `CODEX_MODEL`: Optional model override (gpt-5-codex, gpt-5)
- `CODEX_SANDBOX_MODE`: Sandbox mode (read-only, workspace-write, danger-full-access)
- `CODEX_REASONING_EFFORT`: Reasoning effort level (low, medium, high)
- `CODEX_TIMEOUT`: Timeout in milliseconds (default 1800000 = 30 min)
- `RUST_LOG`: Logging level (trace, debug, info, warn, error)

### MCP Protocol Implementation

The server implements MCP using rmcp library with:

- HTTP transport on localhost:9871 at `/mcp` endpoint
- Tool discovery and execution through rmcp's ServerHandler trait
- Single exposed tool: `prompt` with comprehensive parameter support
- Logging notifications for different message types (info, debug, error)
- Session management via LocalSessionManager

### Prompt Execution Flow

1. Client sends prompt request with optional context (files, variables, working directory)
2. Server assembles codex CLI arguments based on request parameters
3. Spawns codex process with JSON output mode
4. Streams messages through unbounded channel
5. Filters and returns only AgentMessage type content
6. Sends logging notifications for reasoning and token usage

## Important Implementation Details

- Messages from codex are filtered by type - only `AgentMessage` types are returned to the client
- Token usage information is extracted from `TokenCount` messages and sent as debug logs
- Agent reasoning is sent as info-level logging notifications
- The server maintains a single codex-cli subprocess per request (no pooling)
- Failed processes send error notifications through the logging channel
- Process timeout automatically kills the child process to prevent resource leaks
- All enums implement Display and FromStr for seamless string conversion
