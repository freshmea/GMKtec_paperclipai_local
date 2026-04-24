#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INSTANCE_DIR="$ROOT_DIR/paperclip-data/instances/default"
LOG_DIR="$INSTANCE_DIR/shared-memory/logs"
LOCK_FILE="$INSTANCE_DIR/shared-memory/.scheduled_cycle.lock"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"

mkdir -p "$LOG_DIR"
export PATH="$HOME/.npm-global/bin:$PATH"

exec 9>"$LOCK_FILE"
if ! flock -n 9; then
  echo "[$STAMP] shared-memory cycle skipped: lock held" >> "$LOG_DIR/scheduled_cycle.log"
  exit 0
fi

{
  echo "[$STAMP] start"
  qmd update || true
  qmd embed --max-docs-per-batch 64 --max-batch-mb 16 || true
  python3 "$ROOT_DIR/scripts/paperclip_shared_memory/patch_agent_instructions.py"
  python3 "$ROOT_DIR/scripts/paperclip_shared_memory/collect_candidates.py"
  python3 "$ROOT_DIR/scripts/paperclip_shared_memory/consolidate_shared_memory.py"
  ls -1t "$INSTANCE_DIR/shared-memory/candidates"/candidates_*.jsonl 2>/dev/null | tail -n +25 | xargs -r rm -f
  echo "[$STAMP] done"
} >> "$LOG_DIR/scheduled_cycle.log" 2>&1
