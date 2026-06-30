#!/usr/bin/env bash
set -euo pipefail

echo "== Docker Version =="
docker --version || true

echo
echo "== Compose Version =="
docker compose version || true

echo
echo "== Containers =="
docker ps --format 'table {{.Names}}\t{{.Status}}\t{{.Ports}}'

echo
echo "== Networks =="
docker network ls

echo
echo "== Volumes =="
docker volume ls

echo
echo "== Disk Usage =="
docker system df
