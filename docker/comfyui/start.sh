#!/usr/bin/env bash
set -euo pipefail

mkdir -p \
  /opt/comfyui/models \
  /opt/comfyui/input \
  /opt/comfyui/output \
  /opt/comfyui/user \
  /opt/comfyui/custom_nodes

cd /opt/comfyui

exec python main.py \
  --listen "${COMFYUI_LISTEN:-0.0.0.0}" \
  --port "${COMFYUI_INTERNAL_PORT:-8188}" \
  --disable-auto-launch
