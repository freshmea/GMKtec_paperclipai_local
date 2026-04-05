# llama-server 최적화 가이드

## 개요

Gemma 4 31B IT (Q4_K_M) 모델을 llama-server에서 운영할 때의 최적 설정을 정리합니다.
OpenCode, PaperClip 등 다수의 클라이언트가 장기간 연결하여 사용하는 시나리오에 맞춰 튜닝되었습니다.

- **모델:** `gemma-4-31B-it-Q4_K_M.gguf` (17.05 GB, 30.70B 파라미터)
- **훈련 컨텍스트:** 262,144 토큰
- **하드웨어:** AMD Ryzen AI MAX+ 395, Radeon 8060S (108 GB VRAM via GTT), 128 GB LPDDR5

---

## 1. 최종 설정

### docker-compose.yml (llm 서비스 command)

```yaml
command: >
  llama-server
  --host 0.0.0.0
  --port 8000
  -m /models/gemma-4-31B-it-Q4_K_M.gguf
  --alias gemma-4-31b-it
  -t 32
  -ngl 999
  -fa on
  --no-mmap
  -c 262144
  -np 2
  --cache-type-k q8_0
  --cache-type-v q8_0
  --reasoning-format deepseek
```

### .env 환경변수

```env
CONTEXT_LENGTH=262144
PARALLEL_SLOTS=2
LLM_MEMORY_LIMIT=48g
```

### 런타임 확인 결과

| 항목 | 값 |
|------|-----|
| `n_ctx` (총 컨텍스트) | 262,144 |
| `n_ctx_seq` (슬롯당) | 131,072 (128K) |
| `total_slots` | 2 |
| KV 캐시 타입 | q8_0 (K, V 모두) |
| Non-SWA KV 캐시 | 10,880 MiB (글로벌 10레이어) |
| SWA KV 캐시 | 1,275 MiB (슬라이딩 윈도우 50레이어) |
| **총 디바이스 메모리** | **~30 GiB / 57 GiB 여유** |

---

## 2. 주요 옵션 설명

### 컨텍스트 크기 (`-c`)

```
-c ${CONTEXT_LENGTH:-131072}
```

총 KV 캐시 크기를 토큰 수로 지정합니다. 병렬 슬롯(`-np`)으로 나뉘어 슬롯당 컨텍스트가 결정됩니다.

**왜 262K인가?**

- Gemma 4의 훈련 컨텍스트가 262,144이므로 최대 활용
- OpenCode 시스템 프롬프트 + 도구 정의 = ~9,500 토큰 → 여유 있는 컨텍스트 필수
- PaperClip과 연동한 장기 세션에서는 대화 이력이 빠르게 누적됨
- KV 캐시 양자화(q8_0) 덕분에 메모리 부담 최소화

**컨텍스트 크기별 메모리 사용량 (2슬롯, q8_0 기준):**

| `-c` 값 | 슬롯당 | Non-SWA KV | 총 디바이스 메모리 | 비고 |
|----------|--------|------------|-------------------|------|
| 32,768 | 16K | 1,360 MiB | ~21 GiB | 최소 |
| 131,072 | 64K | 5,440 MiB | ~25 GiB | |
| 262,144 | 128K | 10,880 MiB | ~30 GiB | **현재 (훈련 최대)** |

> **참고:** SWA KV 캐시(~1,275 MiB)는 슬라이딩 윈도우 크기에 의해 결정되며 `-c` 값에 비례하지 않습니다.

### 병렬 슬롯 (`-np`)

```
-np ${PARALLEL_SLOTS:-2}
```

동시 요청 처리 수입니다. llama-server의 unified KV 캐시에서 총 컨텍스트를 슬롯 수로 나눕니다.

| 슬롯 | `-c 262144` 기준 슬롯당 | 적합한 경우 |
|------|------------------------|------------|
| 1 | 262K | 단일 사용자, 최대 컨텍스트 |
| **2** | **128K** | **OpenCode + PaperClip 동시 사용** |
| 4 | 64K | 다수 사용자 동시 접속 |

> **주의:** 기본값 `auto`는 4슬롯으로 설정됩니다. 이 경우 `-c 32768`이면 슬롯당 **8K**밖에 안 되어 OpenCode 시스템 프롬프트(~9.5K)가 초과합니다.

### KV 캐시 양자화 (`--cache-type-k`, `--cache-type-v`)

```
--cache-type-k q8_0
--cache-type-v q8_0
```

KV 캐시를 f16 → q8_0으로 양자화하여 **메모리를 절반**으로 줄입니다.

| 타입 | 메모리 (262K, 2슬롯 기준) | 정밀도 | 권장 |
|------|-------------------------|--------|------|
| `f16` | ~24 GiB | 최고 | 메모리 여유 충분 시 |
| **`q8_0`** | **~12 GiB** | **거의 무손실** | **권장** |
| `q4_0` | ~6 GiB | 약간 손실 | 극단적 메모리 절약 시 |

q8_0은 품질 손실이 거의 없으면서 메모리를 절반으로 줄여, 동일 메모리로 2배 더 큰 컨텍스트를 사용할 수 있습니다.

### Reasoning 포맷 (`--reasoning-format`)

```
--reasoning-format deepseek
```

Gemma 4가 `<|channel>thought` 토큰으로 "생각" 과정을 출력하는 방식을 제어합니다.

| 옵션 | thinking | content | 비고 |
|------|----------|---------|------|
| `auto` (기본값) | `reasoning_content`에 분리 | **비어있을 수 있음** | OpenCode 호환 안 됨 |
| **`deepseek`** | **`reasoning_content`에 분리** | **답변만** | **권장** |
| `none` | content에 태그 포함 | thinking 태그 노출 | 태그가 사용자에게 보임 |

**`deepseek` 포맷이 최선인 이유:**

1. 모델이 thinking 후 더 나은 답변을 생성함 (품질 향상)
2. thinking 내용은 `reasoning_content` 필드로 분리됨
3. OpenCode (Vercel AI SDK)는 `content` 필드만 사용 → 깔끔한 출력
4. `--reasoning off` 불필요 — OpenCode가 `reasoning_content`를 자연스럽게 무시함

> **참고:** `--reasoning off`를 추가하면 모델의 사고 과정 자체를 차단하여 응답 품질이 저하됩니다.

---

## 3. Gemma 4 하이브리드 KV 캐시 구조

Gemma 4는 **SWA (Sliding Window Attention) + Global Attention** 하이브리드 아키텍처를 사용합니다:

```
┌─────────────────────────────────────────────────────┐
│  Gemma 4 31B — 60 레이어                             │
│                                                     │
│  ┌─────────────────────────────────────────────┐    │
│  │ SWA 레이어 (50개)                             │    │
│  │ - 슬라이딩 윈도우 = 768 cells/slot            │    │
│  │ - 컨텍스트 크기 무관 (고정)                    │    │
│  │ - KV 캐시: ~1,275 MiB (q8_0)                 │    │
│  └─────────────────────────────────────────────┘    │
│                                                     │
│  ┌─────────────────────────────────────────────┐    │
│  │ Global 레이어 (10개)                          │    │
│  │ - 전체 컨텍스트 참조                           │    │
│  │ - 컨텍스트 크기에 비례                         │    │
│  │ - KV 캐시: ~10,880 MiB (262K, q8_0)          │    │
│  └─────────────────────────────────────────────┘    │
│                                                     │
│  총 KV 캐시: ~12 GiB (q8_0) vs ~24 GiB (f16)       │
└─────────────────────────────────────────────────────┘
```

이 구조 덕분에 컨텍스트를 크게 늘려도 **글로벌 10레이어의 KV만 증가**하여 메모리 효율이 매우 좋습니다.

---

## 4. 기타 옵션

### 성능 관련

| 옵션 | 현재 값 | 설명 |
|------|---------|------|
| `-ngl 999` | 999 | 모든 레이어를 GPU에 오프로드 |
| `-fa on` | on | Flash Attention 활성화 (메모리 효율↑, 속도↑) |
| `--no-mmap` | - | 메모리 맵 비활성 (GPU 메모리로 직접 로드) |
| `-t 32` | 32 | CPU 스레드 수 (프롬프트 처리 시 사용) |
| `GPU_MAX_HW_QUEUES` | 2 | ROCm GPU 하드웨어 큐 제한 (안정성) |

### 메모리 제한 (`deploy.resources.limits.memory`)

```yaml
deploy:
  resources:
    limits:
      memory: ${LLM_MEMORY_LIMIT:-96g}
```

Docker 컨테이너 메모리 상한입니다. `.env`에서 `LLM_MEMORY_LIMIT=48g`으로 설정되어 있으며, 현재 ~30 GiB를 사용하므로 여유가 있습니다.

---

## 5. 트러블슈팅

### 셸 환경변수가 .env를 덮어씀

Docker Compose는 셸 환경변수 > `.env` 파일 순으로 우선합니다.

```bash
# 문제: 셸에 이전 값이 남아있음
echo $CONTEXT_LENGTH   # 32768 (이전 값)

# 해결: 셸 변수 제거 후 재시작
unset CONTEXT_LENGTH PARALLEL_SLOTS
docker compose down llm && docker compose up -d llm

# 확인: docker compose config로 실제 적용될 값 확인
docker compose config | grep -A2 "\-c"
```

### 컨텍스트 초과 (request exceeds available context)

```
error: request (9539 tokens) exceeds the available context size (8192 tokens)
```

**원인:** 슬롯당 컨텍스트가 요청보다 작음. `-c ÷ -np`가 충분한지 확인:

```bash
# 런타임 확인
curl -s http://localhost:8000/props | python3 -c "
import sys, json
d = json.load(sys.stdin)
g = d.get('default_generation_settings', {})
print(f'총 컨텍스트: {g[\"n_ctx\"]}')
print(f'슬롯 수: {d[\"total_slots\"]}')
print(f'슬롯당: {g[\"n_ctx\"] // d[\"total_slots\"]}')
"
```

### reasoning_content는 있는데 content가 비어있음

**원인:** `--reasoning-format auto`일 때 Gemma 4의 thinking 토큰이 분리되어 content가 비어짐.

```bash
# 확인
curl -s http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"gemma-4-31b-it","messages":[{"role":"user","content":"2+2?"}],"max_tokens":50}' \
  | python3 -c "
import sys, json
c = json.load(sys.stdin)['choices'][0]['message']
print('content:', repr(c.get('content','')))
print('reasoning:', repr(c.get('reasoning_content',''))[:100])
"

# 해결: --reasoning-format deepseek 사용
```

---

## 6. 변경 이력

| 날짜 | 변경 사항 |
|------|-----------|
| 2026-04-05 | 컨텍스트 8K → 32K → 262K (훈련 최대), 슬롯 4 → 2, KV q8_0 양자화 적용 |
| 2026-04-05 | `--reasoning-format deepseek` 설정, `--reasoning off` 제거 (thinking 활성화) |
| 2026-04-05 | 셸 환경변수 우선순위 이슈 발견 및 문서화 |
