# Troubleshooting

## Common Issues

### "openzed-git: command not found"

The binary is not in your PATH.

**Fix:**
```bash
# Build and install
cargo build --release
mkdir -p ~/.local/bin
cp target/release/openzed-git ~/.local/bin/

# Add to PATH
export PATH="$HOME/.local/bin:$PATH"
```

### Tasks not appearing in Zed

**Fix:**
```bash
openzed-git install-zed-tasks
```

Then restart Zed.

### Git not found

OpenZed Git requires Git to be installed and in PATH.

**Check:**
```bash
git --version
```

### GitHub CLI issues

For `publish` command, GitHub CLI (`gh`) must be:
1. Installed
2. Authenticated

**Check:**
```bash
gh --version
gh auth status
```

**Fix:**
```bash
# Install from https://cli.github.com/
# Then authenticate
gh auth login
```

### Permission denied when installing tasks

**Fix:**
```bash
# Check if tasks.json exists
ls -la ~/.config/zed/tasks.json

# Create directory if needed
mkdir -p ~/.config/zed
```

### Doctor shows "user.name not configured"

**Fix:**
```bash
git config --global user.name "Your Name"
git config --global user.email "you@example.com"
```

## Getting Help

If you encounter issues not listed here:

1. Run `openzed-git doctor` to check setup
2. Try running the command directly in terminal to see error messages
3. Check that Git and gh are working outside of OpenZed Git

## Conflict and Rebase Issues

### "I am stuck in a merge conflict"

**Symptoms:** Git shows `MERGE_CONFLICT` state.

**Solution:**
```bash
# Run OpenZed Git conflict helper
openzed-git conflicts

# Or manually:
# 1. List conflicted files
git diff --name-only --diff-filter=U

# 2. Open files and resolve conflicts
# 3. Stage resolved files
git add <file>

# 4. Continue merge
git merge --continue
```

### "I am stuck in a rebase"

**Symptoms:** Git shows `REBASE_IN_PROGRESS` state.

**Solution:**
```bash
# Run OpenZed Git rebase helper
openzed-git rebase-helper

# Or manually:
# 1. Resolve any conflicts
# 2. Stage resolved files
git add <file>

# 3. Continue rebase
git rebase --continue

# Or skip the current commit
git rebase --skip

# Or abort the rebase
git rebase --abort
```

### "git rebase --continue failed"

**Symptoms:** Rebase continues to fail after running `git rebase --continue`.

**Solution:**
1. Check `git status` for remaining conflicts
2. Resolve all conflicts manually
3. Stage all resolved files with `git add`
4. Run `git rebase --continue` again
5. If still failing, check for merge conflict markers

### "Zed CLI not found"

**Symptoms:** Conflict helper can't open files in Zed.

**Solution:**
1. Install Zed CLI (if not installed)
2. Ensure `zed` is in your PATH
3. Or open files manually in your editor
4. Use `git add <file>` to mark files as resolved

### "I accidentally skipped a commit during rebase"

**Solution:**
```bash
# Find the skipped commit in reflog
git reflog

# Recover the commit
git branch <recovery-branch> <commit-hash>
```

## Config Issues

### "Config file is invalid"


**Symptoms:** Warning about invalid config file.


**Solution:**
1. Check config file syntax
2. Global config: `~/.config/openzed-git/config.toml`
3. Project config: `.openzed-git.toml`
4. Valid TOML syntax required (no trailing commas, proper quotes)
5. Invalid configs are ignored - safe defaults are used

### "Config changes not taking effect"

**Solution:**
1. Restart openzed-git ( configs loaded at startup)
2. Check config file paths:
   - Global: `~/.config/openzed-git/config.toml`
   - Project: `.openzed-git.toml`
3. Project config overrides global config

## Keybinding Issues

### "Keybindings not showing in Zed"


**Symptoms:** OpenZed Git keybindings don't work in Zed.


**Solution:**
1. Run `openzed-git install-keybindings`
2. Or manually add keybindings to `~/.config/zed/keymap.json`
3. Restart Zed
4. Check keymap.json is valid JSON

### "Zed keymap path not found"

**Solution:**
1. Ensure Zed is installed
2. Keymap location: `~/.config/zed/keymap.json`
3. Create directory if needed: `mkdir -p ~/.config/zed`
4. Use `openzed-git install-keybindings` to create keymap

### "cmd-alt-g not working"

**Solution:**
1. On macOS, use `cmd-alt-g` (or `ctrl-alt-g` depending on keyboard)
2. On Linux, use `ctrl-alt-g`
3. On Windows, use `ctrl-alt-g`
4. Check Zed keymap.json for conflicts

## Theme Issues

### "Theme not changing"

**Symptoms:** Theme remains Aura Dark after setting new theme.

**Solution:**
1. Ensure global config exists: `~/.config/openzed-git/config.toml`
2. Set theme using `openzed-git theme` command
3. Restart openzed-git
4. Check config file has `theme = "theme-name"` (with quotes)

### "Theme preview looks broken"


**Explanation:** Theme preview shows current styling, not the selected theme.
Actual theme is controlled by config setting.

## Error Logs

### Where to find error logs

Error logs are saved to:
- Project: `.openzed-git/gitlogerror.md`
- Fallback: `~/.config/openzed-git/gitlogerror.md`

### Understanding error notifications

When a command fails, OpenZed Git shows:
- Error type (Git, GitHub, Config, etc.)
- A brief message explaining what happened
- A suggested fix when available
- The path to the detailed error log

### Sharing logs for bug reports

1. Find the error log file
2. Copy the error section (from `## timestamp` to `---`)
3. Include in your bug report

### Common error types

| Error Type | Cause | Solution |
|------------|-------|----------|
| Git Error | Git command failed | Check git status |
| GitHub CLI Error | gh command failed | Install or update gh |
| GitHub Auth Error | Not logged in | Run `gh auth login` |
| Config Error | Invalid config file | Recreate config |
| Zed Error | Zed CLI issue | Install/enable Zed CLI |