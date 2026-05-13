#!/bin/bash
set -e

echo "Updating OpenZed Git..."

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Get current version before update
CURRENT_VERSION=$(cat crates/openzed-git/Cargo.toml | grep '^version' | cut -d'"' -f2)

# Pull latest changes if it's a git repo with a remote
if [ -d .git ] && git remote get-url origin &>/dev/null 2>&1; then
    echo "Pulling latest changes..."
    git pull
fi

# Build the project
echo "Building..."
cargo build --release

# Get new version after update
NEW_VERSION=$(cat crates/openzed-git/Cargo.toml | grep '^version' | cut -d'"' -f2)

# Update the binary in ~/.local/bin
echo "Updating binary..."
mkdir -p ~/.local/bin
cp target/release/openzed-git ~/.local/bin/openzed-git

# Re-install Zed tasks
echo "Re-installing Zed tasks..."
~/.local/bin/openzed-git install-zed-tasks

echo ""
if [ "$CURRENT_VERSION" != "$NEW_VERSION" ]; then
    echo "✅ OpenZed Git updated: $CURRENT_VERSION → $NEW_VERSION"
else
    echo "✅ OpenZed Git rebuilt: $CURRENT_VERSION (no version change)"
fi
