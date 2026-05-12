# Roadmap

## v0.1.0 - MVP ✅

- [x] Rust CLI with all current commands
- [x] Publish to GitHub workflow
- [x] Git Graph
- [x] Status+
- [x] Commit + Push
- [x] Push / Set Upstream
- [x] Pull
- [x] Branches
- [x] Remotes
- [x] Setup Remote
- [x] Rename Branch
- [x] Set Upstream
- [x] Open GitHub
- [x] Doctor
- [x] Install Zed Tasks command
- [x] Basic Zed extension structure

## v0.2.0 - Better Git Workflows ✅

- [ ] Graph with dates, authors, branch colors
- [x] Stash management ✅
  - [x] `stash` - Stash current changes (interactive menu)
  - [x] `stash-list` - List all stashes
  - [x] `stash-pop` - Apply and remove latest stash
  - [x] `stash-apply` - Apply stash without removing
  - [x] `stash-drop` - Remove a stash
- [x] Branch switcher ✅
  - [x] `switch` - Interactive branch switcher
- [x] Undo/recovery commands ✅
  - [x] `undo-last-commit` - Safe soft/mixed reset
  - [x] `unstage` - Multi-select file unstaging
  - [x] `restore-file` - Safe file restoration
- [x] Conventional commit assistant ✅
  - [x] `commit` - Guided commit with type/scope/message

## v0.3.0 - GitHub Workflows ✅

- [x] Create PR
  - [x] `pr-create` - Create pull request via GitHub CLI
- [x] List PRs
  - [x] `pr-list` - List open pull requests
- [x] Checkout PR
  - [x] `pr-checkout` - Checkout PR locally
- [x] Open PR/repo/commit in browser
  - [x] `pr-open` - Open PR in browser
- [ ] GitHub issue list/open
  - [ ] `issues-list` - List GitHub issues
  - [ ] `issues-open` - Open issue in browser

## v0.4.0 - Safety and Recovery ✅

- [x] Conflict helper
  - [x] `conflicts` - List conflicting files, open in Zed, resolve, continue/abort merge
  - [x] `rebase-helper` - Continue/skip/abort rebase with safety confirmations
- [x] Safer reset flows
  - [x] `reset-soft` - Soft reset with confirmation
  - [x] `reset-hard` - Hard reset with double confirmation
- [x] Recovery guide
  - [x] Integrated into CLI help
- [ ] Git reflog viewer
- [ ] `reflog` - Show reflog with nice formatting

## v0.6.0 - Future 🔜

- [ ] Light mode theme
- [ ] Screenshots
- [ ] Git Graph screenshot
- [ ] Status+ screenshot
- [ ] Interactive menus (show menu when no subcommand)

## Completed Features

| Feature | Version | Status |
|---------|---------|--------|
| Rust CLI | v0.1.0 | ✅ Done |
| Publish to GitHub | v0.1.0 | ✅ Done |
| Git Graph | v0.1.0 | ✅ Done |
| Status+ | v0.1.0 | ✅ Done |
| Commit + Push | v0.1.0 | ✅ Done |
| Push / Set Upstream | v0.1.0 | ✅ Done |
| Pull | v0.1.0 | ✅ Done |
| Branches | v0.1.0 | ✅ Done |
| Remotes | v0.1.0 | ✅ Done |
| Setup Remote | v0.1.0 | ✅ Done |
| Rename Branch | v0.1.0 | ✅ Done |
| Set Upstream | v0.1.0 | ✅ Done |
| Open GitHub | v0.1.0 | ✅ Done |
| Doctor | v0.1.0 | ✅ Done |
| Install Zed Tasks | v0.1.0 | ✅ Done |
| Aura Dark Theme | v0.1.0 | ✅ Done |
| Stash Management | v0.2.0 | ✅ Done |
| PR Commands | v0.3.0 | ✅ Done |
| Conflict Helper | v0.4.0 | ✅ Done |
| Rebase Helper | v0.4.0 | ✅ Done |
| Main Menu | v0.5.0 | ✅ Done |
| Config System | v0.5.0 | ✅ Done |
| Theme System | v0.5.0 | ✅ Done |
| Keybinding Installer | v0.5.0 | ✅ Done |
| Light Mode Theme | v0.6.0 | 🔜 Planned |

## Contributing

Found a bug or want a feature? Open an issue on GitHub.