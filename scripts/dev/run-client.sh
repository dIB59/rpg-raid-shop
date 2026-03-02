#!/usr/bin/env bash
set -euo pipefail

source "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/lib.sh"

load_dev_env
ensure_repo_root
require_command cargo

SPACETIME_URI="${SPACETIME_URI:-$DEFAULT_SPACETIME_URI}"
SPACETIME_DB="${SPACETIME_DB:-$DEFAULT_SPACETIME_DB}"
SPACETIME_GUEST="${SPACETIME_GUEST:-${1:-Guest_$(hostname)-$(date +%s)}}"

log_info "Launching client (guest=$SPACETIME_GUEST, db=$SPACETIME_DB, uri=$SPACETIME_URI)"

SPACETIME_URI="$SPACETIME_URI" SPACETIME_DB="$SPACETIME_DB" SPACETIME_GUEST="$SPACETIME_GUEST" cargo run -p client_bevy
