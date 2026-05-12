# Safety Rules

OpenZed Git is designed with safety as a core principle. Git operations can be destructive, so we take extra care to prevent accidental data loss.

## Core Safety Rules

### No Force Push

- OpenZed Git **never** uses `git push --force`
- All pushes are safe, standard `git push`
- Force push must be explicitly requested with `--force` flag (future)

### No Automatic Branch Deletion

- Branch deletion requires explicit confirmation
- Never deletes branches without asking first
- Protected branches are not deletable via OpenZed Git

### No Remote Overwrite Without Confirmation

- If a remote origin already exists, OpenZed Git warns you
- Gives you the choice to use existing remote or cancel
- Never replaces `origin` without explicit permission

### No Destructive Reset by Default

- `git reset` is not exposed as a standalone command
- Undo last commit uses safe `git revert` or asks for confirmation
- Reset operations require explicit user consent

### Confirmation Prompts

For any potentially destructive action, OpenZed Git:

1. Clearly explains what will happen
2. Shows the current state vs. proposed change
3. Requires explicit `yes` confirmation
4. Allows cancellation at any time

## GitHub Repository Creation

- Repository creation is always explicit
- User provides name, description, and visibility
- No automatic private/public conversion
- URL is always shown after creation

## Safety Checklist

Before any push or publish:
- [ ] Remote URL is shown for confirmation
- [ ] Branch name is displayed
- [ ] Warning for first push (new branch)

Before any branch operation:
- [ ] Current branch is shown
- [ ] Target branch name is requested
- [ ] Confirmation required for rename/delete

## Emergency Recovery

If something goes wrong:

```bash
# View recent commits
git log --oneline -10

# View all ref changes
git reflog

# Recover deleted branch
git branch <branch-name> <commit-hash>

# Undo last commit (safe)
git revert HEAD

# Cancel a rebase
git rebase --abort

# Cancel a merge
git merge --abort
```

## Reporting Issues

If you encounter any safety-related issues:

1. Run `openzed-git doctor` to check your setup
2. Note the exact error message
3. Report at the project GitHub issues page

## Future Safety Features (v0.4.0)

Planned safety improvements:

- [x] Conflict helper with visual diff
- [x] Rebase helper with step-by-step guidance
- [x] Safer reset flows with confirmation
- [x] Recovery guide integrated into CLI
- [x] Git reflog viewer for easy recovery

## Conflict and Rebase Safety

### Conflict Resolution

- Conflicts are **never auto-resolved**
- Conflicts are **never auto-staged**
- Only **selected files** are marked as resolved
- Abort merge requires **explicit confirmation** (default: NO)

### Rebase Operations

- Skip commit requires **explicit confirmation** (default: NO)
- Abort rebase requires **explicit confirmation** (default: NO)
- Abort shows warning about **losing changes during rebase**
- Original commits are **preserved in reflog** for recovery

### Recovery Commands

```bash
# Cancel a rebase
git rebase --abort

# Cancel a merge
git merge --abort

# View reflog for recovery
git reflog

# Recover a lost commit
git branch <branch-name> <commit-hash>
```
