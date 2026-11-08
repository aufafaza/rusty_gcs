#!/usr/bin/env bash
set -euo pipefail
project_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ "$(uname -sm)" != "Linux x86_64" ]]; then
    echo "This binary requires Linux x86_64." >&2
    exit 1
fi
mkdir -p "$project_dir/.sitl/bin"
binary="$project_dir/.sitl/bin/arducopter"
curl --fail --location --retry 2 \
    https://firmware.ardupilot.org/Copter/stable-4.6.3/SITL_x86_64_linux_gnu/arducopter \
    --output "$binary.download"
chmod +x "$binary.download"
"$binary.download" --help >/dev/null
mv -- "$binary.download" "$binary"
echo "Installed ArduCopter 4.6.3. Start with: bash scripts/run-sitl.sh"
