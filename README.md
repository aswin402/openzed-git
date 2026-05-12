# OpenZed Git

<p align="center">
  <img src="assets/logo.png" alt="OpenZed Git Logo" width="600"/>
</p>

**Extra Git tools for Zed — not a replacement, an enhancement.**

OpenZed Git adds missing Git features to Zed through a Rust CLI and Zed Tasks integration. It does NOT replace Zed's built-in Git panel.

## Features

| Command | Status | Description |
|---|---|---|
| `publish` | ✅ Done | Publish local project to GitHub |
| `graph` | ✅ Done | Show Git graph / commit history |
| `status-plus` | ✅ Done | Show richer Git status |
| `commit-push` | ✅ Done | Commit staged files and push |
| `push` | ✅ Done | Push and optionally set upstream |
| `pull` | ✅ Done | Pull from remote |
| `branches` | ✅ Done | List branches |
| `remotes` | ✅ Done | List remotes |
| `setup-remote` | ✅ Done | Setup remote origin |
| `rename-branch` | ✅ Done | Rename current branch |
| `set-upstream` | ✅ Done | Set upstream branch |
| `doctor` | ✅ Done | Check Git setup and configuration |
| `open-github` | ✅ Done | Open GitHub repository in browser |
| `install-zed-tasks` | ✅ Done | Install OpenZed Git tasks in Zed |
| `stash` | ✅ Done | Manage Git stashes (save/list/apply/pop/drop) |
| `switch` | ✅ Done | Switch branches interactively |
| `conflicts` | ✅ Done | Help resolve merge conflicts |
| `rebase-helper` | ✅ Done | Help with Git rebase operations |
| `menu` | ✅ Done | Central interactive menu |
| `config` | ✅ Done | Manage global/project config |
| `theme` | ✅ Done | Theme listing and preview |
| `install-keybindings` | ✅ Done | Install Zed keybindings |

## Quick Start

### 1. Clone and Build

```bash
git clone <your-repo-url>
cd openzed-git
bash update.sh
```

### 2. Use in Zed

1. Open Command Palette (`Ctrl+Shift+P` or `Cmd+Shift+P`)
2. Type `task: spawn`
3. Select a task, e.g. `OpenZed Git: Publish to GitHub`

Or use directly from terminal:

```bash
openzed-git doctor      # Check your setup
openzed-git publish     # Publish to GitHub
openzed-git graph       # View commit history
```

## Installation

### From Source

```bash
git clone <your-repo-url>
cd openzed-git
bash update.sh
```

This will:
- Build the release binary
- Install to `~/.local/bin/openzed-git`
- Install Zed tasks

### Manual Install

```bash
cargo build --release
mkdir -p ~/.local/bin
cp target/release/openzed-git ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Install Zed tasks
openzed-git install-zed-tasks
```

## Available Tasks in Zed

- `OpenZed Git: Publish to GitHub`
- `OpenZed Git: Git Graph`
- `OpenZed Git: Status+`
- `OpenZed Git: Commit + Push`
- `OpenZed Git: Push / Set Upstream`
- `OpenZed Git: Pull`
- `OpenZed Git: Branches`
- `OpenZed Git: Remotes`
- `OpenZed Git: Setup Remote`
- `OpenZed Git: Rename Branch`
- `OpenZed Git: Set Upstream`
- `OpenZed Git: Open GitHub Repo`
- `OpenZed Git: Doctor`
- `OpenZed Git: Stash`
- `OpenZed Git: Branch Switcher`
- `OpenZed Git: Undo Last Commit`
- `OpenZed Git: Commit Assistant`
- `OpenZed Git: Create Pull Request`
- `OpenZed Git: List Pull Requests`
- `OpenZed Git: Checkout Pull Request`
- `OpenZed Git: Open Pull Request`
- `OpenZed Git: Conflict Helper`
- `OpenZed Git: Rebase Helper`
- `OpenZed Git: Menu`
- `OpenZed Git: Config`
- `OpenZed Git: Theme`
- `OpenZed Git: Install Keybindings`

## Configuration

OpenZed Git supports configuration files:

- **Global config:** `~/.config/openzed-git/config.toml`
- **Project config:** `.openzed-git.toml` (overrides global)

Create and manage with:
```bash
openzed-git config
```

## Keybindings

Install Zed keybindings for quick access:
```bash
openzed-git install-keybindings
```

Suggested bindings (cmd-alt-g prefix):
- `cmd-alt-g m` - Menu
- `cmd-alt-g s` - Status+
- `cmd-alt-g g` - Git Graph
- `cmd-alt-g c` - Commit Assistant
- `cmd-alt-g p` - Publish to GitHub
- `cmd-alt-g r` - Pull Requests

## Error Logs

OpenZed Git shows friendly TUI error notifications and saves detailed logs to:

```text
.openzed-git/gitlogerror.md
```

Fallback:

```text
~/.config/openzed-git/gitlogerror.md
```

Error logs include command, error type, message, and git context for debugging.

## Screenshots

### Publish to GitHub
<img src="assets/screenshots/publish.png" alt="Publish to GitHub" />

### Git Graph
<img src="assets/screenshots/graph.png" alt="Git Graph" />

### Status+
<img src="assets/screenshots/status-plus.png" alt="Status Plus" />

## Documentation

- [Zed Integration](docs/zed-integration.md)
- [Zed Tasks Setup](docs/zed-tasks-setup.md)
- [Commands Reference](docs/commands.md)
- [Aura Dark Theme](docs/theme.md)
- [Troubleshooting](docs/troubleshooting.md)
- [Safety Rules](docs/safety.md)
- [Configuration (Future)](docs/config.md)
- [Roadmap](docs/roadmap.md)

## License

MIT