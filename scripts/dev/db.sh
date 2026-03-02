#!/usr/bin/env bash
set -euo pipefail

source "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/lib.sh"

load_dev_env

SPACETIME_URI="${SPACETIME_URI:-$DEFAULT_SPACETIME_URI}"
SPACETIME_DB="${SPACETIME_DB:-$DEFAULT_SPACETIME_DB}"
SPACETIME_BIN="$(resolve_spacetime_bin || true)"

[ -n "$SPACETIME_BIN" ] || die "Unable to find SpacetimeDB CLI. Set SPACETIME_BIN or add 'spacetime' to PATH."

WASM_PATH="${WASM_PATH:-$DEFAULT_WASM_PATH}"

usage() {
  cat <<'EOF'
Usage: scripts/dev/db.sh <command>

Commands:
  start     Start local SpacetimeDB node (blocking)
  publish   Publish server module to the configured DB
  generate  Regenerate Rust client bindings from module wasm
  config    Print effective configuration
  sync      Publish + regenerate bindings
  help      Show this help

Environment overrides:
  SPACETIME_BIN, SPACETIME_URI, SPACETIME_DB, WASM_PATH
EOF
}

command="${1:-help}"

case "$command" in
  start)
    ensure_repo_root
    log_info "Starting local SpacetimeDB at 0.0.0.0:3000"
    "$SPACETIME_BIN" start --listen-addr 0.0.0.0:3000 --in-memory --non-interactive
    ;;
  publish)
    ensure_repo_root
    log_info "Publishing module to $SPACETIME_URI (db=$SPACETIME_DB)"
    "$SPACETIME_BIN" publish "$SPACETIME_DB" -s "$SPACETIME_URI" --anonymous -y --module-path crates/spacetimedb_module
    ;;
  generate)
    ensure_repo_root
    if [ ! -f "$WASM_PATH" ]; then
      log_info "WASM not found at $WASM_PATH; building module first"
      cargo build --release -p spacetimedb_module --target wasm32-unknown-unknown
    fi
    log_info "Generating Rust bindings from $WASM_PATH"
    "$SPACETIME_BIN" generate --lang rust --out-dir crates/client_bevy/src/module_bindings --bin-path "$WASM_PATH" -y
    ;;
  config)
    cat <<EOF
SPACETIME_BIN=$SPACETIME_BIN
SPACETIME_URI=$SPACETIME_URI
SPACETIME_DB=$SPACETIME_DB
WASM_PATH=$WASM_PATH
REPO_ROOT=$REPO_ROOT
EOF
    ;;
  sync)
    "$0" publish
    "$0" generate
    ;;
  help|-h|--help)
    usage
    ;;
  *)
    echo "Unknown command: $command" >&2
    usage
    exit 1
    ;;
esac
