#!/usr/bin/env bash
set -euo pipefail

echo "== Host =="
hostname
uname -a

echo
echo "== IP Addresses =="
ip addr

echo
echo "== Routes =="
ip route

echo
echo "== Listening Ports =="
ss -tulpn || true

echo
echo "== Docker Networks =="
docker network ls || true
