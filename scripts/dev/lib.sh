#!/usr/bin/env bash

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../.." && pwd)"

DEFAULT_SPACETIME_URI="http://127.0.0.1:3000"
DEFAULT_SPACETIME_DB="rpg-raid-shop-local"
DEFAULT_WASM_PATH="target/wasm32-unknown-unknown/release/spacetimedb_module.wasm"

log_info() {
  echo "[dev] $*"
}

log_error() {
  echo "[dev][error] $*" >&2
}

die() {
  log_error "$*"
  exit 1
}

load_dev_env() {
  local primary="$REPO_ROOT/.env.dev"
  local fallback="$SCRIPT_DIR/.env"

  if [ -f "$primary" ]; then
    set -a
    source "$primary"
    set +a
  elif [ -f "$fallback" ]; then
    set -a
    source "$fallback"
    set +a
  fi
}

resolve_spacetime_bin() {
  if [ -n "${SPACETIME_BIN:-}" ]; then
    echo "$SPACETIME_BIN"
    return 0
  fi

  if command -v spacetime >/dev/null 2>&1; then
    command -v spacetime
    return 0
  fi

  if [ -x "$HOME/.local/bin/spacetime" ]; then
    echo "$HOME/.local/bin/spacetime"
    return 0
  fi

  return 1
}

require_command() {
  local name="$1"
  command -v "$name" >/dev/null 2>&1 || die "Missing required command: $name"
}

ensure_repo_root() {
  cd "$REPO_ROOT"
}
