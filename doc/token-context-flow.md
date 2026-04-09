# 토큰 컨텍스트 흐름 및 최대값 설정 가이드

## 에러 원인 분석

```
request (33121 tokens) exceeds the available context size (32768 tokens)
```

CTO 에이전트의 대화가 누적되어 33,121 토큰이 되었으나, llama-server의 per-slot 컨텍스트가 32,768이라 초과 에러 발생.

---

## 토큰 흐름 경로

```
Paperclip Agent (CTO)
  ↓ heartbeat → API 호출 누적 → 토큰 증가
  ↓
OpenCode adapter (opencode_local)
  ↓ model: "local_llm_moe/gemma-4-26b-a4b-it"
  ↓
opencode.jsonc → provider.local_llm_moe → baseURL: http://llm-moe:8000/v1
  ↓
docker-compose llm-moe → llama-server (port 8002)
  ↓
llama-server -c CONTEXT_LENGTH -np PARALLEL_SLOTS
  ↓
실제 per-slot context = CONTEXT_LENGTH ÷ PARALLEL_SLOTS
```

---

## 설정 포인트별 최대값

### 1단계: llama-server (docker-compose.yml)

서버별 컨텍스트 설정. `-c`는 **전체** KV cache 크기, `-np`는 동시 처리 슬롯 수.

> **핵심 공식: `per-slot context = -c ÷ -np`**

| 서버 | 모델 | 포트 | `-c` (전체) | `-np` (슬롯) | **per-slot 컨텍스트** | 메모리 제한 |
|------|------|------|------------|-------------|---------------------|-----------|
| llm | gemma-4-31B Q4 | 8000 | 131,072 | 2 | **65,536** | 64g |
| llm-fast | gemma-4-E4B Q4 | 8001 | 131,072 | 4 | **32,768** | 16g |
| llm-moe | gemma-4-26B MoE Q4 | 8002 | 131,072 | **2** | **65,536** | 48g |

**설정 파일**: `.env`

```bash
CONTEXT_LENGTH=131072     # llm (31B)
PARALLEL_SLOTS=2          # llm (31B)

FAST_CONTEXT_LENGTH=131072  # llm-fast (E4B)
FAST_PARALLEL_SLOTS=4       # llm-fast (E4B)

MOE_CONTEXT_LENGTH=131072   # llm-moe (26B MoE) ← Paperclip 에이전트 사용
MOE_PARALLEL_SLOTS=2        # llm-moe — 2로 설정해야 65K per-slot 확보
```

**주의사항**:
- 메모리 부족 시 llama-server가 자동으로 `-c`를 축소함
- 실제 할당된 값은 `curl http://localhost:{port}/props`의 `default_generation_settings.n_ctx`로 확인
- KV cache 타입(q8_0)은 메모리 사용량에 직접 영향

### 2단계: OpenCode 설정 (opencode.jsonc) — 자동 컨텍스트 동기화

모델별 `limit.context`와 `compaction` 설정으로 OpenCode가 **자동으로 대화를 압축**합니다.

```jsonc
{
  "provider": {
    "local_llm_moe": {
      "models": {
        "gemma-4-26b-a4b-it": {
          "name": "Gemma 4 26B MoE",
          "limit": {
            "context": 65536,   // ← llama-server per-slot n_ctx와 일치시켜야 함
            "output": 8192
          }
        }
      },
      "options": {
        "baseURL": "http://llm-moe:8000/v1"
      }
    }
  },
  "compaction": {
    "auto": true,     // 컨텍스트 한도 근접 시 자동 요약/압축 (기본값: true)
    "prune": true,    // 오래된 도구 출력 자동 정리 (기본값: true)
    "reserved": 4096  // 압축 중 오버플로 방지용 여유 버퍼
  }
}
```

**동기화 핵심**:
- `limit.context` = llama-server의 `per-slot n_ctx` (= `-c ÷ -np`)
- OpenCode가 이 값을 기준으로 컨텍스트 사용량 추적
- 한도에 도달하면 `compaction` 에이전트가 대화를 자동 요약/축소
- `reserved: 4096` → 압축 작업 자체가 컨텍스트 오버플로를 일으키지 않도록 여유 확보

| 설정 파일 | 용도 |
|-----------|------|
| `~/.config/opencode/opencode.jsonc` | 호스트 직접 접속 (localhost:800x) |
| `opencode-config/opencode.jsonc` | Docker 내부 접속 (llm:8000, llm-moe:8000 등) |

### 3단계: Paperclip 에이전트 (DB adapter_config)

에이전트별 어댑터 설정 (Postgres `agents.adapter_config`):

```json
{
  "model": "local_llm_moe/gemma-4-26b-a4b-it",
  "graceSec": 20,
  "timeoutSec": 0,
  "dangerouslySkipPermissions": true
}
```

- `model` 형식: `{provider_key}/{model_name}` → opencode.jsonc의 provider로 라우팅
- 토큰 관련 설정은 없음 → 서버 측 한도에 의존
- heartbeat 주기(`runtimeConfig.heartbeat.intervalSec`)가 길수록 컨텍스트 누적 가능성 증가

### 4단계: llama-server 내부 동작

```
요청 도착 → 프롬프트 토큰화 (n_prompt_tokens)
  ↓
n_prompt_tokens > per-slot n_ctx ?
  → YES: 400 에러 "exceed_context_size_error"
  → NO: 추론 시작
```

---

## 검증 명령어

```bash
# 각 서버의 실제 per-slot 컨텍스트 확인
for port in 8000 8001 8002; do
  echo "port $port: $(curl -s http://localhost:$port/props | \
    python3 -c "import sys,json; d=json.load(sys.stdin); \
    print(d.get('default_generation_settings',{}).get('n_ctx','N/A'))")"
done

# 에이전트의 adapter_config 확인
PGPASSWORD=paperclip psql -h 127.0.0.1 -p 54329 -U paperclip -d paperclip \
  -P pager=off -c "SELECT name, adapter_config->>'model' as model FROM agents;"
```

---

## 트러블슈팅: 컨텍스트 초과 에러 발생 시

1. **즉시 확인**: `curl http://localhost:{port}/props`로 실제 n_ctx 확인
2. **limit.context 불일치**: opencode.jsonc의 `limit.context`가 per-slot n_ctx와 일치하는지 확인
3. **per-slot 부족**: `.env`에서 `{PREFIX}_PARALLEL_SLOTS`를 줄이기 (2배 확보)
4. **compaction 동작 확인**: opencode.jsonc에 `compaction.auto: true` 설정 확인
5. **전체 context 부족** (메모리 한계): 메모리 제한 조정 또는 `-c` 값을 현실적으로 낮추기
6. **에이전트 대화 비대**: Paperclip heartbeat 주기 조정 또는 대화 리셋

---

## 동기화 체크리스트

llama-server 설정을 변경할 때마다 **반드시** opencode.jsonc의 limit도 동기화:

```
llama-server per-slot = -c ÷ -np
                          ↕ 일치
opencode.jsonc limit.context = per-slot 값
```

| 서버 | per-slot n_ctx | opencode limit.context |
|------|---------------|----------------------|
| llm (31B) | 65,536 | 65,536 ✓ |
| llm-fast (E4B) | 32,768 | 32,768 ✓ |
| llm-moe (26B MoE) | 65,536 | 65,536 ✓ |

---

## 변경 이력

| 날짜 | 변경 | 이유 |
|------|------|------|
| 2026-04-09 | `MOE_PARALLEL_SLOTS`: 4→2 | CTO 에이전트 33K 토큰 초과 (per-slot 32K→65K) |
| 2026-04-09 | opencode.jsonc에 `limit.context` 추가 | OpenCode 자동 compaction이 서버 한도를 인식하도록 동기화 |
| 2026-04-09 | `compaction` 설정 추가 | auto=true, prune=true, reserved=4096 — 대화 자동 압축 활성화 |
