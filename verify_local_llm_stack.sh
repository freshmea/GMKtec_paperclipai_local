#!/bin/bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HOST_CONFIG="${HOME}/.config/opencode/opencode.jsonc"
REPO_CONFIG="${ROOT_DIR}/opencode-config/opencode.jsonc"
CONTAINER_NAME="opencode-web"
EXPECTED_GEMMA_MODEL="local_llm_moe/gemma-4-26b-a4b-it"
EXPECTED_QWEN_MODEL="local_llm_qwen/qwen3-6-35b-a3b"

PASS_COUNT=0
WARN_COUNT=0

pass() {
  printf '[PASS] %s\n' "$1"
  PASS_COUNT=$((PASS_COUNT + 1))
}

warn() {
  printf '[WARN] %s\n' "$1"
  WARN_COUNT=$((WARN_COUNT + 1))
}

section() {
  printf '\n== %s ==\n' "$1"
}

json_get_n_ctx() {
  python3 -c 'import json,sys; data=json.load(sys.stdin); print(data["default_generation_settings"]["n_ctx"])'
}

check_file_contains_all() {
  local file="$1"
  local label="$2"
  shift 2
  if [ ! -f "$file" ]; then
    warn "$label 파일이 없습니다: $file"
    return
  fi

  local missing=0
  for needle in "$@"; do
    if ! grep -Fq "$needle" "$file"; then
      warn "$label 에서 $needle 누락"
      missing=1
    fi
  done

  if [ "$missing" -eq 0 ]; then
    pass "$label 핵심 키 확인"
  fi
}

section "LLM props"

if [ "${MOE_CONTEXT_LENGTH:-}" != "" ] && [ "${MOE_CONTEXT_LENGTH}" != "65536" ]; then
  warn "셸 환경변수 MOE_CONTEXT_LENGTH=${MOE_CONTEXT_LENGTH} 가 .env 예상값 65536 을 override 중입니다"
fi

if [ "${QWEN_CONTEXT_LENGTH:-}" != "" ] && [ "${QWEN_CONTEXT_LENGTH}" != "131072" ]; then
  warn "셸 환경변수 QWEN_CONTEXT_LENGTH=${QWEN_CONTEXT_LENGTH} 가 .env 예상값 131072 을 override 중입니다"
fi

gemma_ctx="$(curl -fsS http://localhost:8002/props | json_get_n_ctx)" || {
  warn "Gemma props 조회 실패"
  gemma_ctx=""
}
qwen_ctx="$(curl -fsS http://localhost:8003/props | json_get_n_ctx)" || {
  warn "Qwen props 조회 실패"
  qwen_ctx=""
}

if [ "${gemma_ctx:-}" = "65536" ]; then
  pass "Gemma per-slot context = 65536"
else
  warn "Gemma per-slot context 불일치: ${gemma_ctx:-unknown}"
fi

if [ "${qwen_ctx:-}" = "131072" ]; then
  pass "Qwen per-slot context = 131072"
else
  warn "Qwen per-slot context 불일치: ${qwen_ctx:-unknown}"
fi

section "OpenCode config sync"

check_file_contains_all "$REPO_CONFIG" "repo OpenCode config" \
  '"local_llm_moe"' '"gemma-4-26b-a4b-it"' '"local_llm_qwen"' '"qwen3-6-35b-a3b"'
check_file_contains_all "$HOST_CONFIG" "host OpenCode config" \
  '"local_llm_moe"' '"gemma-4-26b-a4b-it"' '"local_llm_qwen"' '"qwen3-6-35b-a3b"'

if docker ps --format '{{.Names}}' | grep -Fxq "$CONTAINER_NAME"; then
  container_models="$(docker exec "$CONTAINER_NAME" sh -lc 'opencode models' 2>/dev/null || true)"
  if printf '%s\n' "$container_models" | grep -Fxq "$EXPECTED_GEMMA_MODEL"; then
    pass "container OpenCode models 에 Gemma provider 노출"
  else
    warn "container OpenCode models 에 Gemma provider 누락"
  fi

  if printf '%s\n' "$container_models" | grep -Fxq "$EXPECTED_QWEN_MODEL"; then
    pass "container OpenCode models 에 Qwen provider 노출"
  else
    warn "container OpenCode models 에 Qwen provider 누락"
  fi
else
  warn "OpenCode 컨테이너가 실행 중이 아닙니다: $CONTAINER_NAME"
fi

host_models="$(opencode models 2>/dev/null || true)"
if printf '%s\n' "$host_models" | grep -Fxq "$EXPECTED_GEMMA_MODEL"; then
  pass "host opencode models 에 Gemma provider 노출"
else
  warn "host opencode models 에 Gemma provider 누락"
fi

if printf '%s\n' "$host_models" | grep -Fxq "$EXPECTED_QWEN_MODEL"; then
  pass "host opencode models 에 Qwen provider 노출"
else
  warn "host opencode models 에 Qwen provider 누락"
fi

section "PaperClip agent routing"

if command -v psql >/dev/null 2>&1; then
  agent_table="$(PGPASSWORD=paperclip psql -h 127.0.0.1 -p 54329 -U paperclip -d paperclip -P pager=off -F $'\t' -A -c \
    "SELECT name, adapter_config->>'model' AS model FROM agents ORDER BY name;" 2>/dev/null || true)"
  printf '%s\n' "$agent_table"

  if printf '%s\n' "$agent_table" | grep -Fq $'CMO\tlocal_llm_moe/gemma-4-26b-a4b-it'; then
    pass "CMO -> Gemma routing 확인"
  else
    warn "CMO -> Gemma routing 누락"
  fi

  if printf '%s\n' "$agent_table" | grep -Fq $'UXDesigner\tlocal_llm_moe/gemma-4-26b-a4b-it'; then
    pass "UXDesigner -> Gemma routing 확인"
  else
    warn "UXDesigner -> Gemma routing 누락"
  fi

  for expert in "ROS2 Expert" "Robot Research Agent" "Rust Expert"; do
    if printf '%s\n' "$agent_table" | grep -Fq "${expert}"$'\t'"local_llm_qwen/qwen3-6-35b-a3b"; then
      pass "${expert} -> Qwen routing 확인"
    else
      warn "${expert} -> Qwen routing 누락"
    fi
  done
else
  warn "psql 이 없어 PaperClip agent routing 검증을 건너뜁니다"
fi

section "Summary"
printf 'pass=%s warn=%s\n' "$PASS_COUNT" "$WARN_COUNT"

if [ "$WARN_COUNT" -gt 0 ]; then
  cat <<'EOF'

다음 조치를 우선 검토하세요.
- `unset MOE_CONTEXT_LENGTH MOE_PARALLEL_SLOTS QWEN_CONTEXT_LENGTH QWEN_PARALLEL_SLOTS`
- `env -u MOE_CONTEXT_LENGTH -u MOE_PARALLEL_SLOTS docker compose up -d --force-recreate llm-moe`
- `docker compose restart opencode`
- `cat opencode-config/opencode.jsonc`
- `cat ~/.config/opencode/opencode.jsonc`
- `opencode models`
EOF
  exit 1
fi

exit 0
