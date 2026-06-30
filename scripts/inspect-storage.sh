#!/usr/bin/env bash
set -euo pipefail

echo "== Filesystems =="
df -h

echo
echo "== Block Devices =="
lsblk

echo
echo "== Mounts =="
findmnt
