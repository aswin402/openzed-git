# Zed Integration

OpenZed Git uses Zed Tasks to provide Git helper workflows.

## How It Works

1. Tasks are installed via `openzed-git install-zed-tasks`
2. When user runs `task: spawn`, Zed shows available tasks
3. User selects an OpenZed Git task
4. Zed **opens a new terminal** with TTY support and runs the command
5. Interactive commands work because the new terminal has a proper TTY attached

## Why Use Zed Tasks?

| Running Direct | Via Zed Tasks |
|---------------|---------------|
| Interactive commands need a terminal with TTY | ✅ All commands work (Zed opens new terminal) |
| May fail in scripts, CI, or piped commands | ✅ Works because TTY is available |
| Clear error message if no TTY: "Interactive input requires a terminal" | ✅ Works correctly |

## Tasks Provided

All these tasks open a **new terminal**, so interactive commands work:

**Non-Interactive (always work):**
- `OpenZed Git: Doctor` - Check Git setup
- `OpenZed Git: Git Graph` - Show commit history
- `OpenZed Git: Status+` - Enhanced git status
- `OpenZed Git: Branches` - List branches
- `OpenZed Git: Remotes` - List remotes

**Interactive (work via Zed Tasks with new terminal):**
- `OpenZed Git: Publish to GitHub` - Create and push repo
- `OpenZed Git: Commit + Push` - Guided commit and push
- `OpenZed Git: Commit Assistant` - Conventional commit helper
- `OpenZed Git: Menu` - Interactive main menu
- `OpenZed Git: Stash` - Save/list/apply stashes
- `OpenZed Git: Branch Switcher` - Switch/create branches
- `OpenZed Git: Conflict Helper` - Resolve merge conflicts
- `OpenZed Git: Rebase Helper` - Help with rebases
- `OpenZed Git: Config` - Manage configuration
- `OpenZed Git: Theme` - Theme management

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