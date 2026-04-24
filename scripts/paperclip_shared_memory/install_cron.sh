#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CRON_CMD="*/30 * * * * /bin/bash $ROOT_DIR/scripts/paperclip_shared_memory/scheduled_shared_memory_cycle.sh"

TMP_CRON="$(mktemp)"
crontab -l 2>/dev/null | grep -v 'scheduled_shared_memory_cycle.sh' > "$TMP_CRON" || true
echo "$CRON_CMD" >> "$TMP_CRON"
crontab "$TMP_CRON"
rm -f "$TMP_CRON"

echo "Installed cron entry:"
crontab -l | grep 'scheduled_shared_memory_cycle.sh'
