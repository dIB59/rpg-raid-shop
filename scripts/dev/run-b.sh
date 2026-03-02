#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
SPACETIME_GUEST="Guest_B" "$SCRIPT_DIR/run-client.sh"
