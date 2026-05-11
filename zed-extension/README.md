# OpenZed Git

Extra Git tools for Zed — not a replacement, an enhancement.

## What is OpenZed Git?

OpenZed Git adds missing Git features to Zed. It is NOT a replacement for Zed's built-in Git panel.

Zed's Git panel handles normal Git operations like staging, committing, viewing diffs, push/pull, etc.
OpenZed Git handles advanced workflows that are missing or inconvenient in Zed:

- **Publish to GitHub** — VS Code-like workflow to publish local repos
- **Git Graph** — Visual commit history
- **Git Doctor** — Check your Git/GitHub setup
- **Setup Remote** — Add origin when missing
- **Set Upstream** — Easy upstream branch setup
- **Rename Branch** — Rename current branch
- **Commit + Push** — Guided commit and push flow
- **Open GitHub** — Open repo in browser

## Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/your-username/openzed-git.git
cd openzed-git

# Build release
cargo build --release

# Install binary
mkdir -p ~/.local/bin
cp target/release/openzed-git ~/.local/bin/openzed-git

# Add to PATH if needed
export PATH="$HOME/.local/bin:$PATH"
```

### Install Zed Tasks

```bash
# Run the install command
openzed-git install-zed-tasks
```

### Install Dev Extension in Zed

1. Open Zed
2. Go to Settings → Extensions
3. Click "Install Development Extension"
4. Select the `openzed-git/zed-extension` folder

## Usage

### From Zed

1. Open Command Palette (`Ctrl+Shift+P` or `Cmd+Shift+P`)
2. Type `task: spawn`
3. Select the OpenZed Git task you want

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

All tasks are also available as CLI commands:

```bash
openzed-git publish
openzed-git graph
openzed-git status-plus
openzed-git commit-push
openzed-git push
openzed-git pull
openzed-git branches
openzed-git remotes
openzed-git setup-remote
openzed-git rename-branch
openzed-git set-upstream
openzed-git open-github
openzed-git doctor
```

## Keybindings

Add to your `keymap.json` for quick access:

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "cmd-alt-g p": ["task::Spawn", { "task_name": "OpenZed Git: Publish to GitHub" }],
      "cmd-alt-g g": ["task::Spawn", { "task_name": "OpenZed Git: Git Graph" }],
      "cmd-alt-g d": ["task::Spawn", { "task_name": "OpenZed Git: Doctor" }]
    }
  }
]
```

On Linux, use `ctrl-alt-g` instead of `cmd-alt-g`.

## Safety

OpenZed Git is designed to be safe:
- Never force pushes
- Never deletes branches
- Never overwrites remote origin automatically
- Always asks for confirmation before destructive operations
- Shows exact commands before important operations

## Requirements

- Git installed
- GitHub CLI (`gh`) for publish feature
- Zed (for extension features)

## License

MIT