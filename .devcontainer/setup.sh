#!/usr/bin/env bash
set -euo pipefail

echo "==> Devcontainer setup started"

# ---- Rust project init ----
if [ ! -f Cargo.toml ]; then
  echo "==> Cargo.toml not found. Initializing Rust project..."
  cargo init
else
  echo "==> Cargo.toml found. Skipping cargo init."
fi

# ---- Claude Code ----
if ! command -v claude >/dev/null 2>&1; then
  echo "==> Installing Claude Code..."
  curl -fsSL https://claude.ai/install.sh | bash
else
  echo "==> Claude Code already installed. Skipping."
fi

echo "==> Devcontainer setup finished"
