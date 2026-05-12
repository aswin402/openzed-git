# Commands Reference

Complete reference for all OpenZed Git commands.

## Available Commands

### publish

Publish local project to GitHub.

```bash
openzed-git publish
```

**What it does:**
1. Checks if directory is a Git repo (auto-init if not)
2. Asks for default branch name
3. Creates initial commit if needed
4. Checks GitHub CLI installation and authentication
5. Creates GitHub repository
6. Pushes to remote
7. Offers to open in browser

**Requirements:**
- Git
- GitHub CLI (`gh`) for publish feature

---

### graph

Show Git commit graph with branch history.

```bash
openzed-git graph
```

**Output format:**
```
● 474be86  2026-05-12  (HEAD → main, origin/main)  style: updated tui
│ 4949ba2  2026-05-11                              docs: updated docs
│ a68b90c  2026-05-10                              first
```

**Features:**
- Commit hash in CYAN
- Date in TEXT_MUTED
- Branch refs in PURPLE/YELLOW
- Commit message in TEXT
- Graph symbols in CYAN

---

### status-plus

Show enhanced Git status with clear formatting.

```bash
openzed-git status-plus
```

**Features:**
- Branch name with upstream tracking
- Color-coded file status:
  - Staged: GREEN ●
  - Modified: YELLOW ◎
  - Deleted: RED ◼
  - Untracked: TEXT_MUTED ○
- Clean, readable output

---

### commit

Guided conventional commit assistant.

```bash
openzed-git commit
```

**What it does:**
1. Shows changed files
2. Offers staging options:
   - Select specific files to stage
   - Stage all files
   - Use already staged files
3. Choose commit type:
   - feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert
4. Enter optional scope (no spaces allowed)
5. Enter commit summary (required)
6. Optionally add commit body
7. Preview final message
8. Confirm and commit

**Features:**
- Multi-select file staging
- 12 conventional commit types
- Optional scope with validation
- Summary length warning (>72 chars)
- Final preview with confirmation
- Shows commit hash after success

**Output format:**
```
  feat(graph): add commit dates

  Commit these changes? yes
  ✓ Commit created successfully.
  Commit: abc1234
```

**Status:** ✅ Done in v0.2.3

---

### commit-push

Commit staged files and push in one step.

```bash
openzed-git commit-push
```

**Flow:**
1. Show current staged files
2. Ask for commit message
3. Commit changes
4. Push to remote
5. Show result

**Note:** Use `git add` first to stage files.

---

### push

Push to remote and optionally set upstream.

```bash
openzed-git push
```

**Features:**
- Auto-detects new branches
- Offers to set upstream for new branches
- Shows push result with branch info

---

### pull

Pull from remote.

```bash
openzed-git pull
```

**Features:**
- Shows pull result
- Handles merge conflicts gracefully
- Reports any errors

---

### branches

List all local and remote branches.

```bash
openzed-git branches
```

**Output:**
- Current branch marked
- Remote branches shown
- Branch tracking info

---

### remotes

List configured Git remotes.

```bash
openzed-git remotes
```

**Output:**
```
origin  https://github.com/user/repo.git (fetch)
origin  https://github.com/user/repo.git (push)
```

---

### setup-remote

Add or update remote origin.

```bash
openzed-git setup-remote
```

**Prompts:**
1. Remote name (default: origin)
2. Remote URL

**Safety:** Warns if remote already exists.

---

### rename-branch

Rename current branch.

```bash
openzed-git rename-branch
```

**Prompts:**
1. New branch name
2. Confirmation

**Safety:** Only renames current branch.

---

### set-upstream

Set upstream branch for current branch.

```bash
openzed-git set-upstream
```

**Prompts:**
1. Select remote
2. Select or enter branch name

---

### doctor

Check Git setup and configuration.

```bash
openzed-git doctor
```

**Checks:**
- Git installation
- Git version
- Git config (user.name, user.email)
- Remote origin (if exists)
- GitHub CLI (if installed)
- GitHub authentication (if applicable)

---

### open-github

Open GitHub repository in browser.

```bash
openzed-git open-github
```

**Requirements:** GitHub remote must be configured.

---

### install-zed-tasks

Install OpenZed Git tasks in Zed.

```bash
openzed-git install-zed-tasks
```

**What it does:**
1. Finds Zed's tasks.json location
2. Merges OpenZed Git tasks
3. Preserves existing tasks

---

## Planned Commands

These commands are planned for future versions.

### Stash Commands (v0.2.0)

### stash

Manage Git stashes (save, list, apply, pop, drop).

```bash
openzed-git stash
```

**What it does:**
Shows an interactive menu with options:
- Save stash - Stash current changes with a message
- List stashes - Show all saved stashes
- Apply stash - Select and apply a stash (keeps stash)
- Pop latest stash - Apply and remove the most recent stash
- Drop stash - Select and delete a stash (with confirmation)
- Cancel - Exit the stash menu

**Features:**
- Detects if there are unstaged changes
- Asks for stash message (default: "work in progress")
- Shows stash list with index, branch, and message
- Confirms before destructive actions (pop, drop)
- Shows conflict warnings after applying/popping

**Output format:**
```
  stash@{0} (main) work in progress
  stash@{1} (main) WIP on main: abc1234
```

**Status:** ✅ Done in v0.2.0

#### stash-list

List all stashes.

```bash
openzed-git stash-list
```

#### stash-pop

Apply and remove latest stash.

```bash
openzed-git stash-pop
```

#### stash-apply

Apply stash without removing it.

```bash
openzed-git stash-apply <stash-id>
```

#### stash-drop

Remove a stash.

```bash
openzed-git stash-drop <stash-id>
```

---

### Conflict Commands (v0.4.0)

#### conflicts

Help resolve merge conflicts.

```bash
openzed-git conflicts
```

**Subcommands:**
- `--list` - List files with conflicts
- `--open` - Open conflicting file in editor
- `--abort-merge` - Abort ongoing merge
- `--continue-rebase` - Continue after resolving

---

### Undo/Recovery Commands (v0.2.2)

#### undo-last-commit

Undo the last commit safely with soft or mixed reset.

```bash
openzed-git undo-last-commit
```

**What it does:**
1. Shows last commit (hash, message, date)
2. Offers two reset types:
   - Soft reset - keep changes staged
   - Mixed reset - keep changes unstaged
3. Requires confirmation before reset

**Safety:**
- Never uses `--hard`
- Always confirms before reset
- Shows clear success message

**Status:** ✅ Done in v0.2.2

#### unstage

Unstage files from index.

```bash
openzed-git unstage
```

**What it does:**
1. Shows multi-select list of staged files
2. User selects files to unstage
3. Runs `git restore --staged <file>`

**Safety:**
- Only affects selected files
- Changes remain in working directory
- No file content is discarded

**Status:** ✅ Done in v0.2.2

#### restore-file

Restore files to last committed state.

```bash
openzed-git restore-file
```

**What it does:**
1. Shows multi-select list of modified files
2. Warns about discarding changes
3. Requires explicit confirmation
4. Runs `git restore <file>`

**Safety:**
- Confirmation defaults to NO
- Only restores selected files
- Warns before destructive action

**Status:** ✅ Done in v0.2.2

---

### GitHub PR Commands (v0.3.0)

#### pr-create

Create a pull request on GitHub.

```bash
openzed-git pr-create
```

**What it does:**
1. Shows current branch
2. Checks/creates upstream
3. Asks for base branch (default: main)
4. Warns if on base branch
5. Asks for PR title and optional body
6. Asks for draft PR
7. Previews PR details
8. Confirms before creating
9. Shows PR URL
10. Offers to open in browser

**Safety:**
- Never force pushes
- Never creates PR without confirmation
- Validates GitHub CLI is installed/authenticated

**Status:** ✅ Done in v0.3.0

#### pr-list

List open pull requests.

```bash
openzed-git pr-list
```

**What it does:**
1. Fetches up to 20 open PRs
2. Displays clean list with:
   - PR number in PURPLE
   - Branch → base in CYAN
   - State (OPEN/DRAFT) in GREEN/YELLOW
   - Updated date and author
3. Offers actions:
   - Open PR in browser
   - Checkout PR
   - Refresh list
   - Cancel

**Safety:**
- Confirms before checkout
- Warns about uncommitted changes

**Status:** ✅ Done in v0.3.0

#### pr-checkout

Checkout a pull request locally.

```bash
openzed-git pr-checkout
```

**What it does:**
1. Lists open PRs
2. User selects PR
3. Confirms checkout
4. Runs `gh pr checkout <number>`

**Safety:**
- Confirms before checkout
- Warns about uncommitted changes
- Never auto-stashes or discards

**Status:** ✅ Done in v0.3.0

#### pr-open

Open a pull request in browser.

```bash
openzed-git pr-open
```

**What it does:**
1. Tries to open current branch's PR
2. If none, lists PRs to choose from
3. Opens selection in browser

**Status:** ✅ Done in v0.3.0

---

### Conflict Helper (v0.4.0)

#### conflicts

Help resolve merge conflicts interactively.

```bash
openzed-git conflicts
```

**What it does:**
1. Detects merge conflicts using `git diff --name-only --diff-filter=U`
2. Shows conflicted files with Git state (MERGE_CONFLICT)
3. Provides interactive menu with options:
   - Open conflicted files in Zed
   - Show resolution steps
   - Mark selected files as resolved
   - Continue merge
   - Abort merge
   - Cancel

**Features:**
- Shows all conflicted files in cyan
- Opens files in Zed if `zed` CLI is available
- Multi-select files to mark as resolved
- Confirms before aborting merge
- Shows clear resolution steps

**Safety:**
- Never auto-resolves conflicts
- Never auto-stages all files
- Abort requires explicit confirmation (default: NO)
- Only selected files are marked resolved

**Status:** ✅ Done in v0.4.0

#### rebase-helper

Help with Git rebase operations.

```bash
openzed-git rebase-helper
```

**What it does:**
1. Detects rebase state (rebase-merge or rebase-apply)
2. Shows current state (REBASE_IN_PROGRESS) and conflicted files
3. Provides interactive menu with options:
   - Open conflicted files in Zed
   - Mark selected files as resolved
   - Continue rebase
   - Skip current commit
   - Abort rebase
   - Show rebase steps
   - Cancel

**Features:**
- Detects both rebase-merge and rebase-apply directories
- Warns before skip/abort with clear consequences
- Skip requires explicit confirmation (default: NO)
- Abort requires explicit confirmation (default: NO)
- Shows reflog recovery note on abort

**Safety:**
- Never auto-resolves conflicts
- Never auto-stages files
- Skip commits requires explicit confirmation
- Abort rebase shows warning about losing changes
- Original commits preserved in reflog

**Status:** ✅ Done in v0.4.0

---

## Main Menu (v0.5.0)

### menu

Open interactive main menu for all workflows.

```bash
openzed-git menu
```

**What it does:**
1. Shows main menu with all major commands
2. Has submenus for Pull Requests, Config, and Theme
3. Calls existing command handlers internally
4. Allows cancel at any time

**Features:**
- Central hub for all workflows
- Pull Requests submenu with Create/List/Checkout/Open
- Config and Theme submenus
- Reuses existing command implementations

**Status:** ✅ Done in v0.5.0

---

## Config System (v0.5.0)

### config

Manage global and project configuration.

```bash
openzed-git config
```

**What it does:**
1. Show current config (merged global + project)
2. Create global config at `~/.config/openzed-git/config.toml`
3. Create project config at `.openzed-git.toml`
4. Edit configs in editor (zed or vim fallback)
5. Reset global config with confirmation

**Config priority:**
- Project config (`.openzed-git.toml`) overrides global config
- Safe defaults used if configs are missing

**Config options:**
```toml
default_branch = "main"
default_visibility = "public"
auto_open_browser = true
theme = "aura-dark"
commit_style = "conventional"
default_pr_base = "main"
remote = "origin"
github_owner = "your-username"
```

**Safety:**
- Never overwrites existing config without confirmation
- Reset requires explicit confirmation (default: NO)

**Status:** ✅ Done in v0.5.0

---

## Theme System (v0.5.0)

### theme

Manage and preview themes.

```bash
openzed-git theme
```

**What it does:**
1. Show current theme
2. List available themes
3. Preview theme with sample UI elements
4. Set theme in global config

**Available themes:**
- `aura-dark` - Aura Dark (purple/pink accent, dark background) - **default**
- `minimal-dark` - Minimal Dark (clean, simple dark theme)
- `zed-dark` - Zed Dark (match Zed editor's dark theme)

**Features:**
- Shows sample success/warning/error messages
- Shows link styling
- Shows menu item styling
- Shows file names and refs

**Status:** ✅ Done in v0.5.0

---

## Keybinding Installer (v0.5.0)

### install-keybindings

Install suggested Zed keybindings.

```bash
openzed-git install-keybindings
```

**What it does:**
1. Detect Zed keymap.json location
2. Show suggested keybindings
3. Safely append to existing keymap
4. Or create new keymap if none exists
5. Or print snippet only

**Suggested keybindings:**
```json
{
  "context": "Workspace",
  "bindings": {
    "cmd-alt-g m": ["task::Spawn", { "task_name": "OpenZed Git: Menu" }],
    "cmd-alt-g s": ["task::Spawn", { "task_name": "OpenZed Git: Status+" }],
    "cmd-alt-g g": ["task::Spawn", { "task_name": "OpenZed Git: Git Graph" }],
    "cmd-alt-g c": ["task::Spawn", { "task_name": "OpenZed Git: Commit Assistant" }],
    "cmd-alt-g p": ["task::Spawn", { "task_name": "OpenZed Git: Publish to GitHub" }],
    "cmd-alt-g r": ["task::Spawn", { "task_name": "OpenZed Git: Pull Requests" }]
  }
}
```

**Safety:**
- Never overwrites existing keybindings silently
- Shows suggested bindings before appending
- Can view current keymap before modifying

**Status:** ✅ Done in v0.5.0

---

### Branch Switcher (v0.2.0)

#### switch

Interactive branch switcher for local and remote branches.

```bash
openzed-git switch
```

**What it does:**
Shows an interactive menu with options:
- Switch branch - Switch to a local branch
- Create new branch - Create and switch to a new branch
- Checkout remote branch - Checkout a remote branch
- Cancel - Exit the menu

**Features:**
- Shows current branch in CYAN
- Marks current branch with `*` prefix
- Validates branch names (no spaces, no empty)
- Warns if branch already exists
- Shows warning for uncommitted changes before switching
- Confirms before switching with dirty working directory
- Handles existing local branches when checking out remote

**Safety:**
- Never auto-stashes or discards changes
- Confirms if uncommitted changes exist before branch operations
- Validates branch names to prevent git errors

**Output format:**
```
  Current branch: main
  
  What do you want to do?
    ▸ Switch branch
      Create new branch
      Checkout remote branch
      Cancel
```

**Status:** ✅ Done in v0.2.1

---

### Remote Manager (v0.5.0)

#### remote-manager

Manage Git remotes interactively.

```bash
openzed-git remote-manager
```

**Features:**
- List remotes
- Add new remote
- Remove remote
- Update remote URL

---

### Tag Commands (v0.5.0)

#### tags

List all tags.

```bash
openzed-git tags
```

#### tag-create

Create a new tag.

```bash
openzed-git tag-create <tag-name>
```

#### tag-push

Push tags to remote.

```bash
openzed-git tag-push
```

---

## Command Status Legend

| Status | Meaning |
|--------|---------|
| ✅ Done | Available in current version |
| 🚧 In Progress | Being developed |
| 🔜 Planned | Planned for future release |
