#!/bin/bash
echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu

echo "Building for Linux..."
cargo build --release --target x86_64-unknown-linux-gnu

# echo "Building for macOS..."
# cargo build --release --target x86_64-apple-darwin

echo "All builds completed!"
