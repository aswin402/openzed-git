# Notes

## Zed Extension Development

Zed extensions can provide tasks through a `tasks.json` file in the extension directory.

## Install Dev Extension

1. Open Zed
2. Press `,` to open settings
3. Go to "Extensions" tab
4. Click "Install Development Extension"
5. Select this folder

## Tasks

The extension provides OpenZed Git tasks. If the extension tasks don't appear, you can manually install tasks by running:

```bash
openzed-git install-zed-tasks
```

## Requirements

- `openzed-git` binary must be in PATH
- Git
- `gh` CLI for GitHub features