#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INSTANCE_DIR="$ROOT_DIR/paperclip-data/instances/default"
SHARED_DIR="$INSTANCE_DIR/shared-memory"
export PATH="$HOME/.npm-global/bin:$PATH"

echo "[1/5] Installing qmd"
npm install -g @tobilu/qmd

echo "[2/5] Ensuring shared-memory scaffold"
mkdir -p "$SHARED_DIR/candidates" "$SHARED_DIR/facts" "$SHARED_DIR/logs" "$SHARED_DIR/wiki"

if [[ ! -f "$SHARED_DIR/README.md" ]]; then
  cat > "$SHARED_DIR/README.md" <<'EOF'
# Shared Memory

Instance-scoped shared operational memory for Paperclip.

- `candidates/`: raw promoted candidates from agent-local memory
- `facts/`: deduplicated shared operational facts
- `logs/`: collection and consolidation logs
- `wiki/`: optional promoted summaries
EOF
fi

echo "[3/5] Registering qmd collections"
qmd collection list >/tmp/qmd-collections.txt 2>/dev/null || true

if ! grep -q "paperclip-workspaces" /tmp/qmd-collections.txt 2>/dev/null; then
  (
    cd "$INSTANCE_DIR/workspaces"
    qmd collection add . --name paperclip-workspaces
  )
fi

if ! grep -q "paperclip-shared-memory" /tmp/qmd-collections.txt 2>/dev/null; then
  (
    cd "$SHARED_DIR"
    qmd collection add . --name paperclip-shared-memory
  )
fi

echo "[4/5] Adding collection context"
qmd context add qmd://paperclip-workspaces "Paperclip agent workspaces, AGENTS, HEARTBEAT, daily memory, and local PARA files" >/dev/null 2>&1 || true
qmd context add qmd://paperclip-shared-memory "Promoted shared operational memory and summaries for the Paperclip instance" >/dev/null 2>&1 || true

echo "[5/5] Verifying qmd installation"
qmd collection list
qmd search "CEO inbox" -c paperclip-workspaces | sed -n '1,20p' || true

echo "qmd bootstrap complete"
