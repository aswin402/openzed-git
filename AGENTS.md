# AGENTS.md — OpenZed Git

## Overview

OpenZed Git is a Rust CLI tool that adds missing Git features to the Zed editor through a combination of:
- A Rust binary (`openzed-git`) providing Git operations
- Zed Tasks (JSON config files) for integrating into Zed's command palette
- A TUI with Aura Dark theme for user interaction

## Project Structure

```
.
├── Cargo.toml              # Workspace root (members: crates/openzed-git)
├── install.sh              # Initial installation script
├── update.sh               # Build and update script
├── crates/
│   └── openzed-git/
│       ├── Cargo.toml      # Version: 1.5.0
│       └── src/
│           ├── main.rs         # Entry point, error handling
│           ├── cli.rs          # Clap CLI definition (Commands enum)
│           ├── commands/      # All command implementations (25 commands)
│           │   ├── mod.rs      # execute() dispatcher
│           │   ├── publish.rs, graph.rs, push.rs, etc.
│           ├── core/           # Core utilities
│           │   ├── mod.rs
│           │   ├── git.rs       # Git operations (spawns git CLI)
│           │   ├── github.rs    # GitHub CLI (gh) wrapper
│           │   ├── config.rs    # TOML config (global + project)
│           │   ├── errors.rs    # OpenZedGitError enum
│           │   ├── error_logger.rs  # Error logging to markdown
│           │   └── ...
│           └── ui/             # TUI components
│               ├── mod.rs       # Spinner, ProgressBar
│               ├── aura.rs      # Aura Dark theme colors
│               ├── notifications.rs  # Error display TUI
│               ├── prompts.rs   # User input helpers
│               └── banner.rs    # ASCII banner
├── zed/                    # Zed Tasks JSON configs
│   ├── global-tasks.json   # Main task definitions
│   └── project-tasks.json
└── docs/                   # User documentation
```

## Essential Commands

### Build & Install
```bash
cargo build --release                    # Build binary
./install.sh                             # Build + install to ~/.local/bin + install Zed tasks
./update.sh                              # Pull + rebuild + reinstall tasks
```

### Development
```bash
cargo build                               # Debug build
cargo run -- publish                     # Run with args
```

### Testing
- No test suite exists; commands are tested manually

## Commands List (27 commands)

| Command | Description |
|---------|-------------|
| `publish` | Publish local project to GitHub |
| `graph` | Show Git graph / commit history |
| `status-plus` | Show richer Git status |
| `commit-push` | Commit staged files and push |
| `push` | Push and optionally set upstream |
| `pull` | Pull from remote |
| `branches` | List branches |
| `remotes` | List remotes |
| `setup-remote` | Setup remote origin |
| `rename-branch` | Rename current branch |
| `set-upstream` | Set upstream branch |
| `doctor` | Check Git setup and configuration |
| `open-github` | Open GitHub repository in browser |
| `install-zed-tasks` | Install OpenZed Git tasks in Zed |
| `stash` | Manage Git stashes |
| `switch` | Switch branches interactively |
| `undo-last-commit` | Undo last commit safely |
| `unstage` | Unstage files |
| `restore-file` | Restore files to last commit |
| `commit` | Guided conventional commit assistant |
| `pr-create` | Create a pull request on GitHub |
| `pr-list` | List open pull requests |
| `pr-checkout` | Checkout a pull request locally |
| `pr-open` | Open a pull request in browser |
| `conflicts` | Help resolve merge conflicts |
| `rebase-helper` | Help with Git rebase operations |
| `menu` | Open interactive menu |
| `config` | Manage configuration |
| `theme` | Manage themes |
| `install-keybindings` | Install Zed keybindings |
| `log` | View error logs |

## Architecture Patterns

### Command Dispatch
All commands follow the same pattern:
1. Define command in `cli.rs` (add to `Commands` enum with doc comment)
2. Create module in `commands/` with `run()` function returning `Result<()>`
3. Add dispatcher entry in `commands/mod.rs::execute()` match

### Git Operations
- All git operations spawn the `git` CLI via `std::process::Command`
- `GitInfo` struct in `core/git.rs` provides static methods
- No git library used — raw CLI spawning

### GitHub Operations  
- GitHub CLI (`gh`) is the only GitHub integration
- `GithubInfo` in `core/github.rs` wraps gh commands
- Requires `gh auth login` before use

### Error Handling
- Custom TUI error display (Aura Dark themed) instead of raw panics
- Errors classified into types: Git, GitHubCli, GitHubAuth, GitHubApi, Config, FileSystem, Zed, Internal
- Detailed error logs written to `.openzed-git/gitlogerror.md`

### Configuration
- **Global config**: `~/.config/openzed-git/config.toml`
- **Project config**: `.openzed-git.toml` (overrides global)
- Managed via `config.rs` which handles merge/override logic

## Code Conventions

### File Naming
- Snake_case for files: `commit_push.rs`, `pr_checkout.rs`
- Command modules use verb_noun or noun pattern

### Rust Patterns
- Use `anyhow::Result<()>` for fallible operations
- Use `thiserror` for typed errors in `core/errors.rs`
- Direct CLI spawning over library wrappers
- Heavy use of `context()` for error messages

### TUI/UI Patterns
- Use `aura.rs` constants for colors (CYAN, GREEN, RED, etc.)
- Use `separator()`, `success()`, `error_msg()`, `warning_msg()` helpers
- Use `dialoguer` for user prompts (Select, Input, Confirm)
- Use `input_with_default` from `prompts.rs` for simple inputs

### Module Structure
Each command module typically:
```rust
use crate::core::git::GitInfo;
use crate::ui::aura::aura::{CHECK, GREEN, ...};
use anyhow::Result;

pub fn run() -> Result<()> {
    // Implementation
}
```

## Key Files & Their Responsibilities

| File | Purpose |
|------|---------|
| `main.rs` | Entry point, error classification, TUI error display |
| `cli.rs` | All 25 commands defined via Clap derive |
| `commands/mod.rs` | Dispatcher matching Cli variant to module |
| `core/git.rs` | All git CLI operations |
| `core/github.rs` | All gh CLI operations |
| `core/config.rs` | Config loading, paths, keybindings JSON |
| `core/error_logger.rs` | Markdown error log writer |
| `ui/notifications.rs` | TUI error notification rendering |
| `ui/aura.rs` | Color constants, text helpers |
| `ui/prompts.rs` | Input helpers |

## Dependencies

Key crates used:
- `clap` (4, derive) — CLI parsing
- `dialoguer` — User input dialogs
- `colored` — Terminal colors
- `serde`/`serde_json` — Serialization
- `anyhow` — Error handling
- `thiserror` — Typed errors
- `directories` — Platform config dirs
- `open` — Open URLs in browser
- `toml` — Config parsing
- `chrono` — Timestamps

## Zed Integration

Tasks are defined in JSON at `zed/global-tasks.json` and installed via `install-zed-tasks` command which writes to `~/.config/zed/tasks.json`.

The `cmd-alt-g` prefix is suggested for keybindings.

## Gotchas & Notes

1. **No version bump on publish**: `update.sh` extracts version from `Cargo.toml` but does NOT bump it automatically.

2. **Git auto-init**: The `publish` command auto-initializes a git repo if none exists.

3. **Config override logic**: Project config doesn't do full override — certain fields use default if empty rather than override. See `config.rs:90-113`.

4. **`#![allow(unused)]`**: Many modules have this at the top — unused imports are common.

5. **Error logs location**: Written to `.openzed-git/gitlogerror.md` in current dir, fallback to `~/.config/openzed-git/gitlogerror.md`.

6. **Binary installation**: Installed to `~/.local/bin/openzed-git` — ensure this is in PATH.

7. **GH auth required**: Most GitHub operations require `gh auth login` first.

8. **No tests**: This project has no automated test suite.