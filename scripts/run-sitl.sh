#!/usr/bin/env bash
set -euo pipefail
project_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
binary="$project_dir/.sitl/bin/arducopter"
if [[ ! -x "$binary" ]]; then
    echo "Run: bash scripts/setup-sitl.sh" >&2
    exit 1
fi
mkdir -p "$project_dir/.sitl/state"
cd "$project_dir/.sitl/state"
exec "$binary" --model quad --speedup 1 \
    --defaults "$project_dir/scripts/sitl.parm" \
    --serial0="udpclient:127.0.0.1:${SITL_GCS_PORT:-14550}" "$@"
