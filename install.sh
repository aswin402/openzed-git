#!/bin/bash
set -e

echo "Installing OpenZed Git..."

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Build the project
echo "Building..."
cd "$SCRIPT_DIR"
cargo build --release

# Create bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Copy the binary
cp target/release/openzed-git ~/.local/bin/openzed-git

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo "⚠️  ~/.local/bin is not in your PATH."
    echo "Add it by running:"
    echo '  echo "export PATH=\$HOME/.local/bin:\$PATH" >> ~/.bashrc'
    echo '  source ~/.bashrc'
fi

# Install Zed tasks
echo ""
echo "Installing Zed tasks..."
~/.local/bin/openzed-git install-zed-tasks

echo ""
echo "✅ OpenZed Git installed successfully!"
echo ""
echo "Usage:"
echo "  openzed-git                    # Show banner"
echo "  openzed-git doctor             # Check setup"
echo "  openzed-git publish             # Publish to GitHub"
echo "  openzed-git --version          # Show version"
echo ""
echo "In Zed:"
echo "  Command Palette → task: spawn → OpenZed Git: <command>"
