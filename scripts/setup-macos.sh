#!/bin/sh
# One-shot setup script for macOS (Homebrew required).
set -e

echo "==> Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

echo "==> Adding Cortex-M compilation targets..."
rustup target add thumbv7em-none-eabihf

echo "==> Installing OpenOCD and GDB via Homebrew..."
brew install openocd gdb

brew install --cask gcc-arm-embedded

echo "==> Done. Verify with:"
echo "    openocd --version"
echo "    gdb --version"
