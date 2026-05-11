# Zed Integration

OpenZed Git uses Zed Tasks to provide Git helper workflows.

## How It Works

1. OpenZed Git extension provides tasks in `tasks.json`
2. When user runs `task: spawn`, Zed shows available tasks
3. User selects an OpenZed Git task
4. Zed opens terminal and runs the `openzed-git` CLI command
5. The Rust binary executes the Git workflow

## Tasks Provided

The extension provides these tasks (also available as CLI commands):

- `OpenZed Git: Publish to GitHub` - Publish local repo to GitHub
- `OpenZed Git: Git Graph` - Show commit history
- `OpenZed Git: Status+` - Show enhanced git status
- `OpenZed Git: Commit + Push` - Guided commit and push
- `OpenZed Git: Push / Set Upstream` - Push with upstream setup
- `OpenZed Git: Pull` - Pull from remote
- `OpenZed Git: Branches` - List branches
- `OpenZed Git: Remotes` - List remotes
- `OpenZed Git: Setup Remote` - Add remote origin
- `OpenZed Git: Rename Branch` - Rename current branch
- `OpenZed Git: Set Upstream` - Set upstream branch
- `OpenZed Git: Open GitHub Repo` - Open repo in browser
- `OpenZed Git: Doctor` - Check Git setup

## Installation

### Option 1: Development Extension

1. Open Zed
2. Go to Settings → Extensions
3. Click "Install Development Extension"
4. Select the `zed-extension` folder

### Option 2: Manual Tasks Install

```bash
openzed-git install-zed-tasks
```

This merges OpenZed Git tasks into `~/.config/zed/tasks.json`.

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