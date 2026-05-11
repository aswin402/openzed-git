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