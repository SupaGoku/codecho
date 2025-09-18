# codecho

A high-performance MCP (Model Context Protocol) server that wraps [codex-cli](https://github.com/openai/codex/tree/main/codex-cli) to enable AI agent execution through a standardized protocol. codecho allows AI assistants to execute complex, multi-step tasks autonomously through the codex CLI tool.

## Features

- 🚀 **MCP Protocol Support**: Full implementation of MCP 1.0.0 specification
- 🤖 **AI Agent Execution**: Execute autonomous AI agents via codex-cli
- ⚡ **Real-time Streaming**: Stream execution progress and results
- ⏱️ **Configurable Timeouts**: Set execution time limits for safety
- 📊 **Token Usage Tracking**: Monitor LLM token consumption

## Prerequisites

Before installing codecho, ensure you have the following installed:

1. **Node.js** (v18 or higher) - Required for codex-cli
2. **codex-cli** - The underlying AI agent execution tool

For contributing you also need to have rust

### Installing Prerequisites

#### Node.js

```bash
# macOS (using Homebrew)
brew install node

# Linux (using NodeSource repositories)
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs
```

#### codex-cli

```bash
npm install -g codex-cli

# Verify installation
codex --version
```

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/SupaGoku/codecho.git
cd codecho

# Build in release mode
cargo build --release

# The binary will be available at ./target/release/codecho
```

### Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/SupaGoku/codecho/releases) page.

#### macOS Security Notice

The macOS binaries are not code-signed because I'm not paying Apple $99/year just to distribute this free open-source tool. When you first run codecho on macOS, you'll see a security warning. Here's how to bypass it:

**Option 1: Remove Quarantine (Recommended)**
```bash
# Remove the quarantine flag that macOS adds to downloaded files
xattr -d com.apple.quarantine codecho

# Make it executable
chmod +x codecho
```

**Option 2: System Settings**
1. Try to run `codecho` (it will be blocked)
2. Open System Settings > Privacy & Security
3. Look for "codecho was blocked from use because it is not from an identified developer"
4. Click "Open Anyway"

**Option 3: Right-click Method**
1. Right-click (or Control-click) the `codecho` file in Finder
2. Select "Open" from the context menu
3. Click "Open" in the dialog that appears

#### Linux/Windows Installation

```bash
# Linux - Make it executable
chmod +x codecho

# Add to PATH (optional)
sudo mv codecho /usr/local/bin/
```

## Configuration

Create a `.env` file in your working directory or set environment variables:

```bash
# Copy the example configuration
cp .env.example .env

# Edit with your preferred settings
```

### Environment Variables

| Variable                 | Description                                                          | Default                 |
| ------------------------ | -------------------------------------------------------------------- | ----------------------- |
| `CODEX_BINARY`           | Path to codex CLI binary                                             | `codex` (searches PATH) |
| `CODEX_MODEL`            | LLM model to use                                                     | `gpt-5-codex`           |
| `CODEX_REASONING_EFFORT` | Effort: `low`, `medium`, `high`                                      | `medium`                |
| `CODEX_WORKING_DIR`      | Working directory for execution                                      | `.`                     |
| `CODEX_SANDBOX_MODE`     | Sandbox policy: `read-only`, `workspace-write`, `danger-full-access` | `read-only`             |
| `CODEX_TIMEOUT`          | Execution timeout in milliseconds                                    | `1800000` (30 minutes)  |

## Running as a Service

### macOS (launchd)

1. First, find your Node.js and npm paths:

```bash
# Find Node.js binary location
which node

# Find npm global bin directory (where codex will be installed)
npm config get prefix

# Common locations:
# Homebrew (Apple Silicon): /opt/homebrew/bin/node, /opt/homebrew/bin
# Homebrew (Intel): /usr/local/bin/node, /usr/local/bin
# nvm: ~/.nvm/versions/node/vX.X.X/bin
# Direct install: /usr/local/bin
```

2. Create a plist file at `~/Library/LaunchAgents/com.codecho.server.plist` (adjust paths based on step 1):

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.codecho.server</string>

    <key>ProgramArguments</key>
    <array>
        <string>/your/path/to/codecho</string>
    </array>

    <key>EnvironmentVariables</key>
    <dict>
        <key>PATH</key>
        <string>/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/opt/homebrew/opt/node/bin:/Users/YOUR_USERNAME/.npm-global/bin</string>
        <key>NODE_PATH</key>
        <string>/Users/[YOUR_USERNAME]/.local/bin:/your/path/to/node:/your/path/to/codecho:/your/path/to/codex:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin</string>
        <key>CODEX_BINARY</key>
        <string>codex</string>
        <key>CODEX_TIMEOUT</key>
        <string>1800000</string>
        <key>RUST_LOG</key>
        <string>info</string>
    </dict>

    <key>WorkingDirectory</key>
    <string>/Users/YOUR_USERNAME</string>

    <key>StandardOutPath</key>
    <string>/tmp/codecho.out.log</string>

    <key>StandardErrorPath</key>
    <string>/tmp/codecho.err.log</string>

    <key>RunAtLoad</key>
    <true/>

    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

3. Load and start the service:

```bash
# Get your user ID
id -u  # Usually returns 501 for the first user

# Load the service using modern launchctl (macOS 10.11+)
launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/com.codecho.server.plist

# Or if you know your UID (e.g., 501)
launchctl bootstrap gui/501 ~/Library/LaunchAgents/com.codecho.server.plist

# Check status
launchctl list | grep codecho

# View logs
tail -f /tmp/codecho.out.log
tail -f /tmp/codecho.err.log
```

**Note about domains:** The `gui/501` format specifies the domain where the service runs:

- `gui` = GUI session (for user services that need desktop access)
- `501` = Your user ID (find with `id -u`)
- Together: "Run in the GUI session for user 501"

4. To stop or unload:

```bash
# Stop and unload the service (modern syntax)
launchctl bootout gui/$(id -u)/com.codecho.server

# Or with explicit UID
launchctl bootout gui/501/com.codecho.server

# Remove the plist file if you want to completely uninstall
rm ~/Library/LaunchAgents/com.codecho.server.plist
```

**Legacy commands:** If you're on an older macOS version (pre-10.11), use:

```bash
# Old load command
launchctl load ~/Library/LaunchAgents/com.codecho.server.plist

# Old unload command
launchctl unload ~/Library/LaunchAgents/com.codecho.server.plist
```

### Linux (systemd)

1. Create a service file at `/etc/systemd/system/codecho.service`:

```ini
[Unit]
Description=codecho MCP Server
After=network.target

[Service]
Type=simple
User=YOUR_USERNAME
WorkingDirectory=/home/YOUR_USERNAME
Environment="PATH=/Users/[YOUR_USERNAME]/.local/bin:/your/path/to/node:/your/path/to/codecho:/your/path/to/codex:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin"
Environment="CODEX_BINARY=codex"
Environment="CODEX_TIMEOUT=1800000"
Environment="RUST_LOG=info"
ExecStart=/your/path/to/codecho
Restart=always
RestartSec=10

# Logging
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

2. Enable and start the service:

```bash
# Reload systemd configuration
sudo systemctl daemon-reload

# Enable auto-start on boot
sudo systemctl enable codecho

# Start the service
sudo systemctl start codecho

# Check status
sudo systemctl status codecho

# View logs
journalctl -u codecho -f
```

3. To stop or disable:

```bash
# Stop the service
sudo systemctl stop codecho

# Disable auto-start
sudo systemctl disable codecho
```

## Usage

### As an MCP Server

codecho can be used with any MCP-compatible client. Configure your MCP client to connect to codecho:

```json
{
  "mcpServers": {
    "codecho": {
      "type": "http",
      "url": "http://localhost:9871/mcp"
    }
  }
}
```

**codecho can even be used in codex!** 🎉🎉🎉

You just need to route it through [mcp-proxy](https://github.com/sparfenyuk/mcp-proxy)

```
~/.codex/config.toml

[mcp_servers.codecho]
command = "mcp-proxy"
args = ["--transport", "streamablehttp", "http://localhost:9871/mcp"]
```

### Available Tools

#### `prompt`

Execute a codex prompt with optional context:

```json
{
  "prompt": "Write a Python script that calculates fibonacci numbers",
  "context": {
    "working_dir": "/path/to/project",
    "files": [
      {
        "path": "requirements.txt",
        "content": "numpy==1.24.0"
      }
    ],
    "variables": {
      "max_number": "100"
    }
  },
  "timeout": 60000
}
```

## Troubleshooting

### Common Issues

#### "launchctl load/unload failed: Input/output error"

**Problem**: Getting errors when using old launchctl commands on modern macOS.

**Solutions**:

Use modern launchctl syntax (macOS 10.11+):

```bash
# Instead of: launchctl load <plist>
launchctl bootstrap gui/$(id -u) <plist>

# Instead of: launchctl unload <plist>
launchctl bootout gui/$(id -u)/<service-name>

# Example for codecho:
launchctl bootstrap gui/501 ~/Library/LaunchAgents/com.codecho.server.plist
launchctl bootout gui/501/com.codecho.server
```

#### "codex: command not found"

**Problem**: codecho cannot find the codex CLI binary.

**Solutions**:

1. Ensure codex is installed: `npm install -g codex-cli`
2. Add Node.js global bin to PATH:

   ```bash
   # Find npm global bin directory
   npm config get prefix

   # Add to PATH (add to ~/.bashrc or ~/.zshrc)
   export PATH="$(npm config get prefix)/bin:$PATH"
   ```

3. Specify full path in CODEX_BINARY environment variable

#### "Node.js not in PATH when running as service"

**Problem**: Service cannot find Node.js or npm packages.

**Solutions**:

**macOS**: Ensure PATH includes Homebrew and npm directories in plist:

```xml
<key>PATH</key>
<string>/usr/local/bin:/usr/bin:/bin:/opt/homebrew/bin:/Users/YOUR_USERNAME/.npm-global/bin</string>
```

**Linux**: Add Node paths to systemd service:

```ini
Environment="PATH=/usr/local/bin:/usr/bin:/bin:/home/YOUR_USERNAME/.npm-global/bin"
```

#### "Permission denied" errors

**Problem**: codecho doesn't have permissions to access files or directories.

**Solutions**:

1. Check file permissions: `ls -la /path/to/file`
2. Ensure service user has appropriate permissions
3. Set proper CODEX_WORKING_DIR with write access
4. Review CODEX_SANDBOX_MODE setting

#### Service crashes or restarts frequently

**Problem**: codecho service keeps restarting.

**Solutions**:

1. Check logs for errors:
   - macOS: `/tmp/codecho.err.log`
   - Linux: `journalctl -u codecho`
2. Verify all dependencies are accessible
3. Increase CODEX_TIMEOUT if tasks are timing out
4. Check system resources (memory, CPU)

#### "Failed to spawn codex process"

**Problem**: codecho cannot start the codex subprocess.

**Solutions**:

1. Test codex directly: `codex --version`
2. Check for conflicting environment variables
3. Verify working directory exists and is writable
4. Ensure no antivirus is blocking execution

### Debug Mode

Enable detailed logging for troubleshooting:

```bash
# Set debug logging
export RUST_LOG=debug

# Or trace for maximum verbosity
export RUST_LOG=trace

# Run codecho
codecho
```

### Getting Help

1. Check the [Issues](https://github.com/SupaGoku/codecho/issues) page for known problems
2. Enable debug logging and collect logs before reporting issues
3. Include your configuration (without sensitive data) when seeking help

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/SupaGoku/codecho.git
cd codecho

# Install Rust dependencies
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings
```

### Architecture Overview

- **types module**: Data structures for Codex and MCP communication
- **codex module**: Subprocess management and codex-cli interaction
- **server module**: MCP protocol implementation and server

## License

MIT License - See [LICENSE](LICENSE) file for details

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.
