#!/bin/sh
set -eu

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: Rust and Cargo are required."
    echo "Install Rust from https://rustup.rs/"
    exit 1
fi

if ! command -v playerctl >/dev/null 2>&1; then
    echo "Error: playerctl is required."
    echo "Install playerctl using your distribution's package manager."
    exit 1
fi

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

cd "$SCRIPT_DIR"

echo "Building Versync..."
cargo build --release --locked

echo "Installing Versync..."
mkdir -p "$HOME/.local/bin"
install -m 755 target/release/versync "$HOME/.local/bin/versync"

echo
echo "Versync installed successfully!"
echo "Location: $HOME/.local/bin/versync"
echo "Run: versync"
