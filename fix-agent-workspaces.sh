#!/bin/bash
# fix-agent-workspaces.sh — 모든 에이전트의 instructions를 workspace에 동기화
# 에이전트 고용 후 HEARTBEAT.md 등이 $AGENT_HOME에 없을 때 실행
set -e
BASE=/home/aa/vllm/paperclip-data/instances/default
count=0

for company_dir in "$BASE"/companies/*/; do
  company_id=$(basename "$company_dir")
  for agent_dir in "$company_dir"agents/*/; do
    agent_id=$(basename "$agent_dir")
    inst="$agent_dir/instructions"
    ws="$BASE/workspaces/$agent_id"

    [[ ! -d "$inst" ]] && continue
    mkdir -p "$ws/memory" "$ws/life"

    for f in "$inst"/*.md; do
      [[ -f "$f" ]] || continue
      fname=$(basename "$f")
      if [[ ! -f "$ws/$fname" ]] || [[ "$f" -nt "$ws/$fname" ]]; then
        cp "$f" "$ws/"
        echo "Synced: $fname → $agent_id"
        ((count++))
      fi
    done
  done
done

echo "Done. $count file(s) synced."
