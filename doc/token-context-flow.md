# 토큰 컨텍스트 흐름 및 최대값 설정 가이드

현재 기본 운영 경로는 `Gemma 4 26B-A4B + Qwen3.6 35B-A3B` 입니다.

- `Gemma 26B`: `65,536` 토큰으로 제한
- `Qwen3.6`: `131,072` 토큰 long-context 경로
- `PaperClip`: 역할별로 두 모델을 혼합 라우팅

## 대표 에러 예시

```text
request (100321 tokens) exceeds the available context size (65536 tokens)
```

이 에러는 보통 다음 둘 중 하나입니다.

1. 긴 대화가 `Gemma 26B` 경로로 들어갔다.
2. OpenCode `limit.context`와 llama-server `n_ctx`가 어긋났다.

## 토큰 흐름

### Gemma 26B 경로

```text
PaperClip agent
  -> model: local_llm_moe/gemma-4-26b-a4b-it
  -> OpenCode provider.local_llm_moe
  -> http://llm-moe:8000/v1
  -> docker compose service llm-moe
  -> host port 8002
```

### Qwen3.6 경로

```text
PaperClip agent
  -> model: local_llm_qwen/qwen3-6-35b-a3b
  -> OpenCode provider.local_llm_qwen
  -> http://llm-qwen:8000/v1
  -> docker compose service llm-qwen
  -> host port 8003
```

## 서버별 컨텍스트 기준

> 공식: `per-slot context = -c / -np`

| 서비스 | 모델 | 포트 | `-c` | `-np` | per-slot | 비고 |
|--------|------|------|------|-------|----------|------|
| `llm-moe` | Gemma 4 26B-A4B | 8002 | 65,536 | 1 | 65,536 | 기본 shorter/mid-context |
| `llm-qwen` | Qwen3.6 35B-A3B | 8003 | 131,072 | 1 | 131,072 | 기본 long-context |
| `llm` | Gemma 4 31B | 8000 | 65,536 | 1 | 65,536 | legacy |
| `llm-fast` | Gemma 4 E4B | 8001 | 131,072 | 4 | 32,768 | legacy |

## `.env` 기준값

```bash
MOE_CONTEXT_LENGTH=65536
MOE_PARALLEL_SLOTS=1

QWEN_CONTEXT_LENGTH=131072
QWEN_PARALLEL_SLOTS=1

CONTEXT_LENGTH=65536
PARALLEL_SLOTS=1

FAST_CONTEXT_LENGTH=131072
FAST_PARALLEL_SLOTS=4
```

운영 해석:

- `Gemma 26B`는 100K급 프롬프트를 받지 않도록 의도적으로 `65,536`에 고정
- `Qwen3.6`은 `131,072` long-context 작업을 전담
- `Qwen`은 `--cache-ram 0`으로 prompt cache를 끔

## OpenCode 동기화 포인트

`opencode-config/opencode.jsonc`와 `~/.config/opencode/opencode.jsonc` 모두 아래 기준을 따라야 합니다.

```jsonc
{
  "disabled_providers": ["local_llm", "local_llm_fast"],
  "provider": {
    "local_llm_moe": {
      "models": {
        "gemma-4-26b-a4b-it": {
          "limit": { "context": 65536, "output": 8192 }
        }
      }
    },
    "local_llm_qwen": {
      "models": {
        "qwen3-6-35b-a3b": {
          "limit": { "context": 131072, "output": 8192 }
        }
      }
    }
  }
}
```

핵심:

- `limit.context`는 llama-server의 per-slot 값과 같아야 합니다.
- OpenCode compaction은 이 값을 기준으로 동작합니다.

## PaperClip 권장 라우팅

| 에이전트 | 권장 모델 |
|----------|-----------|
| `CMO`, `UXDesigner` | `local_llm_moe/gemma-4-26b-a4b-it` |
| `ROS2 Expert`, `Robot Research Agent`, `Rust Expert` | `local_llm_qwen/qwen3-6-35b-a3b` |
| `CEO`, `CTO` | 기존 `codex_local / gpt-5.4` 유지 |

이렇게 나누면 `Gemma 26B`가 불필요하게 100K급 누적 문맥을 받는 일을 줄일 수 있습니다.

## 검증 명령

가장 빠른 일괄 점검:

```bash
cd /home/aa/vllm
./verify_local_llm_stack.sh
```

이 스크립트는 실제 `n_ctx`, OpenCode provider 노출, host/container config 동기화, PaperClip agent 라우팅을 함께 확인합니다.

```bash
for port in 8002 8003; do
  echo "port $port: $(curl -s http://localhost:$port/props | \
    python3 -c "import sys,json; d=json.load(sys.stdin); \
print(d.get('default_generation_settings',{}).get('n_ctx','N/A'))")"
done
```

```bash
PGPASSWORD=paperclip psql -h 127.0.0.1 -p 54329 -U paperclip -d paperclip \
  -P pager=off -c "SELECT name, adapter_config->>'model' AS model FROM agents ORDER BY name;"
```

## 체크리스트

- `llm-moe`의 실제 `n_ctx`가 `65536`인지 확인
- `llm-qwen`의 실제 `n_ctx`가 `131072`인지 확인
- Docker용 OpenCode config와 호스트용 OpenCode config가 같은지 확인
- PaperClip 에이전트 중 장문 작업 담당이 `Qwen`으로 가는지 확인

## 변경 이력

| 날짜 | 변경 |
|------|------|
| 2026-04-23 | 기본 운영 조합을 `Gemma 26B + Qwen3.6`으로 확정 |
| 2026-04-23 | `Gemma 26B` 컨텍스트를 `65,536`으로 제한 |
| 2026-04-23 | `Qwen3.6`을 `131,072`, `-np 1`, `--cache-ram 0`으로 long-context 전담화 |
| 2026-04-23 | OpenCode provider 활성 조합을 `local_llm_moe + local_llm_qwen`으로 변경 |
