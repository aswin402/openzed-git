# Completed Features

> **OpenZed Git** = Missing Git workflows for Zed, powered by a safe Rust CLI and beautiful Aura Dark terminal UI.

This document tracks all completed features across releases.

---

## Project Overview

| Item | Detail |
|------|--------|
| **Current Version** | v0.5.1 |
| **Total Commands** | 30 |
| **Total Rust Files** | 48 |
| **Documentation** | 11 files |
| **License** | MIT |

### Release Timeline

| Version | Status | Focus |
|---------|--------|-------|
| **v0.1.0** | ✅ Released | Core Git workflows, GitHub integration, Zed tasks |
| **v0.2.0** | ✅ Released | Stash management |
| **v0.2.1** | ✅ Released | Branch Switcher |
| **v0.2.2** | ✅ Released | Undo/Recovery Commands |
| **v0.2.3** | ✅ Released | Conventional Commit Assistant |
| **v0.3.0** | ✅ Released | GitHub PR Commands |
| **v0.4.0** | ✅ Released | Conflict helper, rebase helper |
| **v0.5.0** | ✅ Released | Config files, themes, keybindings, main menu |
| **v0.5.1** | ✅ Released | Error notification + gitlogerror.md logging |

---

## v0.3.0 - GitHub PR Commands ✅

**Released:** GitHub Pull Request workflow commands.

### PR Commands

| Command | Description |
|---------|-------------|
| `pr-create` | Create PR with preview and confirmation |
| `pr-list` | List open PRs with actions |
| `pr-checkout` | Checkout PR locally |
| `pr-open` | Open PR in browser |

### PR Features

**pr-create:**
- Shows current branch
- Checks/creates upstream
- Base branch selection (default: main)
- PR title, body, draft option
- Preview before creation
- Shows PR URL
- Open in browser option

**pr-list:**
- Fetches up to 20 open PRs
- Displays PR number, branch, title, state, author
- Actions: Open in browser, Checkout, Refresh, Cancel

**pr-checkout:**
- Lists open PRs
- Confirms before checkout
- Warns about uncommitted changes

**pr-open:**
- Opens current branch's PR if exists
- Lists PRs if no current PR

### PR Safety

- Never force pushes
- Validates GitHub CLI is installed/authenticated
- Confirms before destructive actions
- Warns about uncommitted changes

### PR Core Functions

| Function | Description |
|----------|-------------|
| `ensure_gh_ready()` | Check gh installed and authenticated |
| `pr_list()` | Get open PRs via gh |
| `pr_create(base, head, title, body, draft)` | Create PR |
| `pr_checkout(number)` | Checkout PR locally |
| `pr_view_web(number)` | Open PR in browser |
| `pr_view_web_current()` | Open current branch PR |
| `current_pr_url()` | Get current PR URL |
| `has_upstream()` | Check if branch has upstream |
| `push_current_branch()` | Push and set upstream |

### PullRequest Struct

```rust
pub struct PullRequest {
    pub number: u32,
    pub title: String,
    pub head: String,
    pub base: String,
    pub state: String,
    pub is_draft: bool,
    pub author: String,
    pub updated_at: String,
    pub url: String,
}
```

---

## v0.4.0 - Conflict and Rebase Helper ✅

**Released:** Guided conflict and rebase recovery workflows.

### Conflict Helper

| Command | Description |
|---------|-------------|
| `conflicts` | Interactive merge conflict resolution |

**Conflicts Features:**
- Detects conflicted files using `git diff --name-only --diff-filter=U`
- Shows Git state: MERGE_CONFLICT
- Lists conflicted files in cyan
- Opens files in Zed if `zed` CLI is available
- Multi-select files to mark as resolved
- Shows resolution steps
- Continue or abort merge with confirmation

**Conflicts Safety:**
- Never auto-resolves conflicts
- Never auto-stages all files automatically
- Abort requires explicit confirmation (default: NO)
- Only selected files are marked resolved

### Rebase Helper

| Command | Description |
|---------|-------------|
| `rebase-helper` | Interactive rebase assistance |

**Rebase Features:**
- Detects rebase state (rebase-merge or rebase-apply)
- Shows Git state: REBASE_IN_PROGRESS
- Lists conflicted files during rebase
- Opens files in Zed if `zed` CLI is available
- Continue rebase after resolving
- Skip current commit (with strong warning)
- Abort rebase (with strong warning)
- Shows rebase steps

**Rebase Safety:**
- Never auto-resolves conflicts
- Skip requires explicit confirmation (default: NO)
- Abort shows warning about losing changes
- Original commits preserved in reflog for recovery

### GitState Enum

```rust
pub enum GitState {
    Clean,
    MergeInProgress,
    RebaseInProgress,
    Conflicts,
}
```

### Conflict/Rebase Core Functions

| Function | Description |
|---------|-------------|
| `conflicted_files()` | Get list of conflicted files |
| `has_merge_in_progress()` | Check if merge is active |
| `has_rebase_in_progress()` | Check if rebase is active |
| `mark_resolved(files)` | Stage resolved files |
| `continue_merge()` | Continue after merge |
| `abort_merge()` | Abort merge with confirmation |
| `continue_rebase()` | Continue rebase |
| `skip_rebase()` | Skip current commit |
| `abort_rebase()` | Abort rebase |
| `open_in_zed(files)` | Open files in Zed |
| `zed_available()` | Check if zed CLI exists |

---

## All Commands

| Command | Description | Version |
|---------|-------------|---------|
| `publish` | Full GitHub publish workflow | v0.1.0 |
| `graph` | Git commit history with colors | v0.1.0 |
| `status-plus` | Enhanced git status | v0.1.0 |
| `commit-push` | Commit + push in one step | v0.1.0 |
| `push` | Push with upstream setup | v0.1.0 |
| `pull` | Pull from remote | v0.1.0 |
| `branches` | List all branches | v0.1.0 |
| `remotes` | List remotes | v0.1.0 |
| `setup-remote` | Add remote origin | v0.1.0 |
| `rename-branch` | Rename current branch | v0.1.0 |
| `set-upstream` | Set upstream branch | v0.1.0 |
| `doctor` | Check Git setup | v0.1.0 |
| `open-github` | Open repo in browser | v0.1.0 |
| `install-zed-tasks` | Install Zed tasks | v0.1.0 |
| `stash` | Interactive stash menu | v0.2.0 |
| `switch` | Interactive branch switcher | v0.2.1 |
| `undo-last-commit` | Soft or mixed reset | v0.2.2 |
| `unstage` | Multi-select file unstaging | v0.2.2 |
| `restore-file` | Safe file restoration | v0.2.2 |
| `commit` | Guided conventional commit | v0.2.3 |
| `pr-create` | Create PR on GitHub | v0.3.0 |
| `pr-list` | List open PRs | v0.3.0 |
| `pr-checkout` | Checkout PR locally | v0.3.0 |
| `pr-open` | Open PR in browser | v0.3.0 |
| `conflicts` | Conflict helper | v0.4.0 |
| `rebase-helper` | Rebase helper | v0.4.0 |

---

## Command Categories

| Category | Commands |
|----------|----------|
| **Publishing** | `publish`, `open-github` |
| **History** | `graph`, `status-plus` |
| **Commit** | `commit`, `commit-push` |
| **Remote** | `push`, `pull`, `branches`, `remotes`, `setup-remote`, `set-upstream` |
| **Local** | `rename-branch` |
| **Stash** | `stash` |
| **Branch** | `switch` |
| **Undo/Recovery** | `undo-last-commit`, `unstage`, `restore-file` |
| **Pull Request** | `pr-create`, `pr-list`, `pr-checkout`, `pr-open` |
| **Conflict/Rebase** | `conflicts`, `rebase-helper` |
| **Menu** | `menu` |
| **Config** | `config` |
| **Theme** | `theme` |
| **Keybindings** | `install-keybindings` |
| **Utility** | `doctor`, `install-zed-tasks` |

---

## Build Status

```bash
✅ cargo fmt     - Passed
✅ cargo check   - Passed (0 warnings)
✅ cargo build   - Passed
```

---

## Changelog

### v0.5.1 (Current)
- **New:** Global error notification system
- **New:** Error classification (Git, GitHub, Config, etc.)
- **New:** `gitlogerror.md` persistent error logging
- **New:** `ErrorReport` struct for error handling
- **New:** `ErrorType` enum for error classification
- **New:** `show_error_notification()` TUI function
- **New:** Error context collection (dir, branch, state)

### v0.5.0
- **New:** `menu` - Central interactive menu
- **New:** `config` - Global/project config management
- **New:** `theme` - Theme listing and preview
- **New:** `install-keybindings` - Zed keybinding installer
- **New:** `OpenZedConfig` struct with TOML support
- **New:** Config precedence (project overrides global)
- **New:** Suggested Zed keybindings (cmd-alt-g prefix)

### v0.4.0
- **New:** `conflicts` - Interactive conflict helper
- **New:** `rebase-helper` - Interactive rebase helper
- **New:** `GitState` enum for merge/rebase detection

### v0.2.3
- **New:** `commit` - Guided conventional commit assistant

### v0.2.2
- **New:** `undo-last-commit`, `unstage`, `restore-file`

### v0.2.1
- **New:** `switch` - Interactive branch switcher

### v0.2.0
- **New:** `stash` - Interactive stash menu

### v0.1.0
- **Initial release** with 14 core Git commands

---

## Contributing

Found a bug or want a feature? Open an issue on GitHub.