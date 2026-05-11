# OpenZed Git

**Extra Git tools for Zed — not a replacement, an enhancement.**

OpenZed Git adds missing Git features to Zed through a Rust CLI and Zed Tasks integration. It does NOT replace Zed's built-in Git panel.

## Features

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

## How It Works

```
Zed extension / task provider
        ↓
OpenZed Git tasks appear in Zed task list
        ↓
User selects task
        ↓
Zed terminal opens
        ↓
Rust binary runs focused flow
```

## Safety Rules

- Never force pushes
- Never deletes branches
- Never overwrites remote origin automatically
- Always asks for confirmation before destructive operations

## Requirements

- Git
- GitHub CLI (`gh`) for publish feature
- Zed (for extension integration)

## Documentation

- [Zed Integration](docs/zed-integration.md)
- [Zed Tasks Setup](docs/zed-tasks-setup.md)
- [Troubleshooting](docs/troubleshooting.md)
- [Roadmap](docs/roadmap.md)

## License

MIT