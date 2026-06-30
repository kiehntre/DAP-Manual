#!/usr/bin/env bash
set -euo pipefail

echo "== NVIDIA =="
nvidia-smi || true

echo
echo "== Vulkan =="
vulkaninfo --summary 2>/dev/null || true
