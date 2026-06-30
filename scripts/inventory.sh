#!/usr/bin/env bash
set -euo pipefail

OUT="${1:-docs/36-Appendix/inventory.md}"

mkdir -p "$(dirname "$OUT")"

{
  echo "# Inventory"
  echo
  echo "Generated: $(date -Iseconds)"
  echo
  echo "## Host"
  echo
  echo '```text'
  hostname
  uname -a
  echo '```'
  echo
  echo "## Docker Containers"
  echo
  echo '```text'
  docker ps --format 'table {{.Names}}\t{{.Status}}\t{{.Ports}}' || true
  echo '```'
  echo
  echo "## Docker Networks"
  echo
  echo '```text'
  docker network ls || true
  echo '```'
} > "$OUT"

echo "Wrote $OUT"
