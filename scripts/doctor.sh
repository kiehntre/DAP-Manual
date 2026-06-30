#!/usr/bin/env bash
set -euo pipefail

echo "DAP Doctor"
echo "=========="
echo

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

"$DIR/inspect-network.sh" || true
echo
"$DIR/inspect-docker.sh" || true
