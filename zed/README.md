# Zed Task Files

This folder contains fallback task configurations for OpenZed Git.

## Global Tasks

`global-tasks.json` - Tasks available in all Zed projects.

To install globally:
```bash
cp global-tasks.json ~/.config/zed/tasks.json
```

Or run:
```bash
openzed-git install-zed-tasks
```

## Project Tasks

`project-tasks.json` - Example tasks.json for individual projects.

To use per-project, copy to `.zed/tasks.json` in your project root.

## Keybindings

See `keymap-example.json` for example keybindings.

On Linux, use `ctrl-alt-g` prefix instead of `cmd-alt-g`.