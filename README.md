# OpenZed Git

**Extra Git tools for Zed — not a replacement, an enhancement.**

OpenZed Git adds missing Git features to Zed through a Rust CLI and Zed Tasks integration. It does NOT replace Zed's built-in Git panel.

## What OpenZed Git Adds

- **Publish to GitHub** — VS Code-like workflow to publish local repos
- **Git Graph** — Visual commit history
- **Git Doctor** — Check your Git/GitHub setup
- **Setup Remote** — Add origin when missing
- **Set Upstream** — Easy upstream branch setup
- **Rename Branch** — Rename current branch
- **Commit + Push** — Guided commit and push flow
- **Open GitHub** — Open repo in browser

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

## Installation

### 1. Build from Source

```bash
git clone https://github.com/your-username/openzed-git.git
cd openzed-git
cargo build --release
mkdir -p ~/.local/bin
cp target/release/openzed-git ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

### 2. Install Zed Tasks

```bash
openzed-git install-zed-tasks
```

### 3. Install Dev Extension (optional)

1. Open Zed
2. Settings → Extensions
3. "Install Development Extension"
4. Select `openzed-git/zed-extension`

## Usage

### From Zed

1. Open Command Palette (`Ctrl+Shift+P` or `Cmd+Shift+P`)
2. Type `task: spawn`
3. Select a task, e.g. `OpenZed Git: Publish to GitHub`

Available tasks:
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

### From Terminal

All commands work outside Zed too:

```bash
openzed-git doctor
openzed-git graph
openzed-git publish
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

## License

MIT