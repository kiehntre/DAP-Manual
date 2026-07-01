#!/usr/bin/env bash
set -euo pipefail

OUT="docs/99-Inventory/System-Snapshot.md"
mkdir -p "$(dirname "$OUT")"

safe_run() {
  echo '```text'
  timeout 15s "$@" 2>&1 || true
  echo '```'
}

{
  echo "# System Snapshot"
  echo
  echo "Generated: $(date -Iseconds)"
  echo

  echo "## Host"
  safe_run hostnamectl

  echo "## Kernel"
  safe_run uname -a

  echo "## CPU"
  safe_run lscpu

  echo "## Memory"
  safe_run free -h

  echo "## Block Devices"
  safe_run lsblk

  echo "## Filesystems - Safe Root Check"
  safe_run df -h /

  echo "## Mounts"
  safe_run findmnt

  echo "## Broken Mount Clues"
  safe_run bash -c "findmnt -rn -o TARGET,FSTYPE | grep -Ei 'fuse|rclone|mergerfs|archive|rom|games|ngc|dc' || true"

  echo "## Network"
  safe_run ip addr

  echo "## Routes"
  safe_run ip route

  echo "## PCI Devices"
  safe_run lspci

  echo "## GPU"
  safe_run bash -c "nvidia-smi || echo 'nvidia-smi not available'"

  echo "## Docker"
  safe_run bash -c "docker --version && docker compose version && docker ps --format 'table {{.Names}}\t{{.Status}}\t{{.Ports}}'"

  echo "## Docker Networks"
  safe_run docker network ls

  echo "## Listening Ports"
  safe_run ss -tulpn

} > "$OUT"

echo "Wrote $OUT"
