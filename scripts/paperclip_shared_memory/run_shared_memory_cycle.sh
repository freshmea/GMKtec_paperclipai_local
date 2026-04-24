#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

python3 "$ROOT_DIR/scripts/paperclip_shared_memory/patch_agent_instructions.py"
python3 "$ROOT_DIR/scripts/paperclip_shared_memory/collect_candidates.py"
python3 "$ROOT_DIR/scripts/paperclip_shared_memory/consolidate_shared_memory.py"

echo "Shared memory cycle completed"
