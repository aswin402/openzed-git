# Zed Tasks Setup

How OpenZed Git tasks get installed and integrated with Zed.

## Overview

Zed Tasks allow running shell commands from within Zed. OpenZed Git uses this mechanism to provide Git helper workflows.

## Task Installation Flow

1. User runs `openzed-git install-zed-tasks`
2. The command finds Zed's tasks.json location:
   - Linux/macOS: `~/.config/zed/tasks.json`
   - Windows: `%APPDATA%\Zed\tasks.json`
3. Parses existing tasks.json (if it exists)
4. Merges OpenZed Git tasks without overwriting existing tasks
5. Saves the updated tasks.json

## Task Format

```json
{
  "label": "OpenZed Git: Publish to GitHub",
  "command": "openzed-git publish",
  "use_new_terminal": true,
  "allow_concurrent_runs": false,
  "reveal": "always"
}
```

- `label` - Shown in Zed's task picker
- `command` - The CLI command to run
- `use_new_terminal` - Open a new terminal for this task
- `allow_concurrent_runs` - Prevent concurrent executions
- `reveal` - Always show the terminal when running

## Using Tasks in Zed

1. Open Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`)
2. Type `task: spawn`
3. Select an OpenZed Git task
4. Zed opens the terminal and runs the command

## Manual Installation

If you prefer to install tasks manually:

1. Copy tasks from `zed-extension/tasks.json` or `zed/global-tasks.json`
2. Add them to your `~/.config/zed/tasks.json`
3. Restart Zed