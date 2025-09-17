# Contributing to codecho

This document provides guidelines and instructions for contributing to the project.

## Getting Started

### Prerequisites

- Rust 1.75 or higher
- Cargo (comes with Rust)
- Git
- Codex CLI (for testing)

### Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/codecho.git
   cd codecho
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/codecho.git
   ```
4. Create a `.env` file from the example:
   ```bash
   cp .env.example .env
   ```
5. Build the project:
   ```bash
   cargo build
   ```

## Development Workflow

### Creating a Feature Branch

Always create a new branch for your work:

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### Code Standards

#### Rust Style

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` and fix any issues
- Follow the formatting rules from `rustfmt.toml`

#### Code Organization

- Keep modules focused and single-purpose
- Place new types in `src/types.rs`
- Server logic goes in `src/server.rs`
- Codex interaction code goes in `src/codex.rs`

#### Documentation

- Update CLAUDE.md if you change the architecture
- Update README.md for user-facing changes

### Commit Guidelines

#### Commit Message Format

Use conventional commit format:

```
type(scope): subject

body (optional)

footer (optional)
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code restructuring without changing functionality
- `chore`: Maintenance tasks, dependency updates
- `perf`: Performance improvements

Examples:

```bash
git commit -m "feat(server): add timeout configuration for prompts"
git commit -m "fix(codex): handle empty stdout responses gracefully"
git commit -m "docs: update environment variables in README"
```

#### Commit Best Practices

- Make atomic commits (one logical change per commit)
- Write clear, descriptive commit messages
- Reference issue numbers when applicable: `fix(server): handle timeout errors (#42)`
- Sign your commits if you have GPG set up: `git commit -S`

## Submitting Changes

### Pull Request Process

1. Update your branch with the latest upstream changes:

   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. Push your branch to your fork:

   ```bash
   git push origin feature/your-feature-name
   ```

3. Create a pull request on GitHub

### Pull Request Guidelines

#### PR Title

Follow the same format as commit messages:

- `feat(server): add streaming response support`
- `fix(codex): resolve timeout race condition`

#### PR Description Template

```markdown
## Description

Brief description of what this PR does

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing

- [ ] Unit tests pass locally with `cargo test`
- [ ] Code follows style guidelines (`cargo fmt` and `cargo clippy`)
- [ ] Self-review of code completed
- [ ] Documentation updated if needed

## Related Issues

Closes #(issue number)

## Additional Context

Any additional information or screenshots
```

### Code Review

- Be patient and respectful during code review
- Address all feedback or explain why you disagree
- Make requested changes in new commits (don't force-push during review)
- Once approved, you may squash commits if requested

## Project Structure

### Key Files and Directories

```
codecho/
├── src/
│   ├── main.rs          # Application entry point, HTTP server setup
│   ├── server.rs        # MCP server implementation using rmcp
│   ├── codex.rs         # Codex CLI client and process management
│   ├── types.rs         # Shared types and data structures
│   └── bin/
│       └── test_client.rs # Test client for development
├── Cargo.toml           # Project dependencies and metadata
├── CLAUDE.md            # Claude Code integration guide
├── README.md            # Project documentation
└── CONTRIBUTING.md      # This file
```

### Architecture Overview

The project uses:

- **rmcp**: MCP protocol implementation
- **tokio**: Async runtime
- **axum**: HTTP server framework
- **serde**: JSON serialization
- **tracing**: Structured logging

## Common Tasks

### Adding a New Tool

1. Define request/response types in `src/types.rs`
2. Add the tool method to `CodexMcp` in `src/server.rs` with the `#[tool]` attribute
3. Implement the tool logic
4. Update the tool router
5. Update documentation

### Modifying Codex Integration

1. Update types in `src/types.rs` if needed
2. Modify `CodexClient` in `src/codex.rs`
3. Update argument assembly in `assemble_args()`
4. Update environment variable documentation

### Debugging

Enable detailed logging:

```bash
RUST_LOG=debug cargo run
# or
RUST_LOG=trace cargo run
```

Use the test client for debugging:

```bash
cargo run --bin test_client --features test_client
```

## Getting Help

- Open an issue for bugs or feature requests
- Start a discussion for questions or ideas
- Check existing issues and PRs before creating new ones
- Join our community chat (if applicable)

## License

By contributing to codecho, you agree that your contributions will be licensed under the same license as the project (MIT).

## Recognition

Contributors will be recognized in:

- The project's contributors list
- Release notes for significant contributions
- Special mentions for exceptional contributions

Thank you for contributing to codecho!
