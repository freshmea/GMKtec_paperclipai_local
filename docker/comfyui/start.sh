#!/usr/bin/env bash
set -euo pipefail

mkdir -p \
  /opt/comfyui/models \
  /opt/comfyui/input \
  /opt/comfyui/output \
  /opt/comfyui/user \
  /opt/comfyui/custom_nodes

python - <<'PY'
from pathlib import Path

TARGETS = [
  Path("/opt/venv/lib/python3.12/site-packages/comfyui_manager/glob/manager_core.py"),
  Path("/opt/venv/lib/python3.12/site-packages/comfyui_manager/legacy/manager_core.py"),
]

REPLACEMENTS = [
  (
    'with open(requirements_path, "r") as requirements_file:\n                for line in requirements_file:\n',
    'lines = manager_util.robust_readlines(requirements_path)\n            for line in lines:\n',
  ),
  (
    'with open(manager_requirements_path, "r") as f:\n        for line in f:\n',
    'for line in manager_util.robust_readlines(manager_requirements_path):\n',
  ),
]

patched = []
for target in TARGETS:
  if not target.exists():
    continue

  source = target.read_text(encoding="utf-8")
  updated = source
  for old, new in REPLACEMENTS:
    updated = updated.replace(old, new)

  if updated != source:
    target.write_text(updated, encoding="utf-8")
    patched.append(str(target))

if patched:
  print("[startup] patched ComfyUI-Manager requirement readers:")
  for path in patched:
    print(f"[startup]   {path}")
PY

cd /opt/comfyui

exec python main.py \
  --listen "${COMFYUI_LISTEN:-0.0.0.0}" \
  --port "${COMFYUI_INTERNAL_PORT:-8188}" \
  --disable-auto-launch \
  --enable-manager
