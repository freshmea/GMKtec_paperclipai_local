# 로컬 LLM 모델 추가 가이드

새로운 GGUF 모델을 다운로드하고, llama-server · OpenCode · PaperClip 전체 스택에
등록하는 방법을 정리합니다.

> **현재 구성:** Gemma 4 31B IT (Q4_K_M) — `gemma-4-31b-it`
>
> **스택:** llama-server (`:8000`) → OpenCode Docker (`:3000`) / OpenCode 호스트 CLI → PaperClip (`:3100`)

---

## 목차

1. [모델 다운로드](#1-모델-다운로드)
2. [llama-server 설정](#2-llama-server-설정)
3. [OpenCode 설정 (Docker)](#3-opencode-설정-docker)
4. [OpenCode 설정 (호스트 CLI)](#4-opencode-설정-호스트-cli)
5. [PaperClip 에이전트 설정](#5-paperclip-에이전트-설정)
6. [전체 검증](#6-전체-검증)
7. [참고 정보](#7-참고-정보)

---

## 1. 모델 다운로드

### 1-1. GGUF 모델 파일 다운로드

```bash
cd /home/aa/vllm

# HuggingFace CLI로 다운로드 (예: Gemma 4 31B IT Q4_K_M)
hf download \
    unsloth/gemma-4-31B-it-GGUF \
    gemma-4-31B-it-Q4_K_M.gguf \
    --local-dir ./models

# 또는 download_model.sh 스크립트 사용
./download_model.sh
```

### 1-2. 다른 모델로 교체할 경우

```bash
# 예: Qwen3 32B
hf download \
    Qwen/Qwen3-32B-GGUF \
    qwen3-32b-q4_k_m.gguf \
    --local-dir ./models
```

> **양자화 선택 기준:** 128GB 공유 메모리 시스템에서는 30B급 모델 기준
> Q4_K_M (~18GB)이 품질–메모리 최적 균형점입니다.

---

## 2. llama-server 설정

### 2-1. `.env` 파일 수정

```env
# --- 모델 설정 ---
MODEL_FILENAME=gemma-4-31B-it-Q4_K_M.gguf   # ← GGUF 파일명
SERVED_MODEL_NAME=gemma-4-31b-it              # ← API에서 사용할 모델 이름 (alias)
CONTEXT_LENGTH=262144                          # ← 모델의 훈련 컨텍스트에 맞춰 설정
PARALLEL_SLOTS=2                               # ← 동시 요청 수 (슬롯당 = CONTEXT/SLOTS)

# --- 서버 설정 ---
LLM_MEMORY_LIMIT=48g
OMP_NUM_THREADS=32
GPU_MAX_HW_QUEUES=2
```

**핵심 변수:**

| 변수 | 설명 | 예시 |
|------|------|------|
| `MODEL_FILENAME` | `models/` 디렉토리 내 GGUF 파일명 | `gemma-4-31B-it-Q4_K_M.gguf` |
| `SERVED_MODEL_NAME` | API 요청 시 `model` 파라미터로 사용할 이름 | `gemma-4-31b-it` |
| `CONTEXT_LENGTH` | 총 KV 캐시 크기 (토큰) | `262144` |
| `PARALLEL_SLOTS` | 동시 요청 슬롯 수. 슬롯당 컨텍스트 = `CONTEXT_LENGTH ÷ PARALLEL_SLOTS` | `2` → 슬롯당 128K |

### 2-2. docker-compose.yml (보통 수정 불필요)

`.env` 변수를 참조하므로 모델 교체 시 `docker-compose.yml` 수정은 불필요합니다.
현재 command 섹션:

```yaml
command: >
  llama-server
  --host 0.0.0.0
  --port 8000
  -m /models/${MODEL_FILENAME:-gemma-4-31B-it-Q4_K_M.gguf}
  --alias ${SERVED_MODEL_NAME:-gemma-4-31b-it}
  -t ${OMP_NUM_THREADS:-32}
  -ngl 999
  -fa on
  --no-mmap
  -c ${CONTEXT_LENGTH:-131072}
  -np ${PARALLEL_SLOTS:-2}
  --cache-type-k q8_0
  --cache-type-v q8_0
  --reasoning-format deepseek
```

> **`--reasoning-format deepseek`:** Gemma 4, Qwen3 등 thinking 모델용 설정입니다.
> thinking이 없는 모델에서는 이 옵션을 제거하거나 `none`으로 변경하세요.

### 2-3. LLM 서버 재시작

```bash
cd /home/aa/vllm

# 셸 환경변수가 .env를 덮어쓰지 않도록 정리
unset MODEL_FILENAME SERVED_MODEL_NAME CONTEXT_LENGTH PARALLEL_SLOTS

# 서버 재시작 (force-recreate로 변경사항 적용)
docker compose up -d --force-recreate llm

# 로딩 대기 (31B 모델 기준 2~3분 소요)
echo "LLM 로딩 대기..." && \
until curl -sf http://localhost:8000/health >/dev/null 2>&1; do sleep 5; done && \
echo "준비 완료"
```

### 2-4. LLM 서버 검증

```bash
# 모델 정보 확인
curl -s http://localhost:8000/v1/models | python3 -m json.tool

# 실제 설정 확인 (슬롯당 컨텍스트 등)
curl -s http://localhost:8000/props | python3 -c "
import sys, json
d = json.load(sys.stdin)
g = d.get('default_generation_settings', {})
print(f'모델:       {g.get(\"model\")}')
print(f'총 컨텍스트: {g.get(\"n_ctx\"):,}')
print(f'슬롯 수:    {d.get(\"total_slots\")}')
print(f'슬롯당:     {g.get(\"n_ctx\",0) // max(d.get(\"total_slots\",1),1):,}')
"

# 간단한 응답 테스트
curl -s http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"gemma-4-31b-it","messages":[{"role":"user","content":"2+2는?"}],"max_tokens":50}' \
  | python3 -c "
import sys, json
c = json.load(sys.stdin)['choices'][0]['message']
print('content:', c.get('content',''))
print('reasoning:', (c.get('reasoning_content','') or '')[:100])
"
```

---

## 3. OpenCode 설정 (Docker)

Docker 컨테이너 내부의 OpenCode는 환경변수로 LLM 서버에 자동 연결됩니다.

### 3-1. 환경변수 (docker-compose.yml)

```yaml
opencode:
  environment:
    - OPENAI_API_BASE=http://llm:8000/v1     # 컨테이너 간 통신
    - OPENAI_API_KEY=sk-no-key-required       # 로컬 서버이므로 더미 키
    - LOCAL_ENDPOINT=http://llm:8000/v1
```

> 모델 이름 변경 시 OpenCode Docker는 별도 설정 없이 llama-server의 `--alias`를
> 자동 감지합니다. 컨테이너 재시작만 하면 됩니다:
>
> ```bash
> docker compose restart opencode
> ```

### 3-2. 컨테이너 내부 설정 파일 (선택)

영속적 프로바이더 등록이 필요하면 볼륨 마운트를 추가하세요:

```yaml
# docker-compose.yml — opencode 서비스에 볼륨 추가
volumes:
  - ${WORKSPACE_DIR:-./workspace}:/workspace
  - ./opencode-config:/root/.config/opencode    # ← 추가
```

`opencode-config/opencode.jsonc` 파일 생성:

```jsonc
{
  "$schema": "https://opencode.ai/config.json",
  "provider": {
    "local_llm": {
      "name": "Local LLM (llama.cpp)",
      "npm": "@ai-sdk/openai-compatible",
      "models": {
        "gemma-4-31b-it": { "name": "gemma-4-31b-it" }
      },
      "options": {
        "baseURL": "http://llm:8000/v1"       // 컨테이너 내부 주소
      }
    }
  }
}
```

---

## 4. OpenCode 설정 (호스트 CLI)

PaperClip의 `opencode_local` 어댑터는 호스트의 `opencode` CLI를 직접 호출합니다.
이 설정은 PaperClip 연동에 필수입니다.

### 4-1. OpenCode CLI 설치

```bash
# Docker 이미지에서 바이너리 추출
docker cp opencode-web:/usr/local/bin/opencode /tmp/opencode
chmod +x /tmp/opencode

# 호스트에 설치
mkdir -p ~/.local/bin
mv /tmp/opencode ~/.local/bin/opencode

# PATH 확인
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# 설치 확인
opencode --version
```

### 4-2. 호스트 설정 파일

`~/.config/opencode/opencode.jsonc`:

```jsonc
{
  "$schema": "https://opencode.ai/config.json",
  "disabled_providers": [],
  "provider": {
    "local_llm": {
      "name": "Local LLM (llama.cpp)",
      "npm": "@ai-sdk/openai-compatible",
      "models": {
        "gemma-4-31b-it": {                     // ← 모델 ID (API alias와 일치)
          "name": "gemma-4-31b-it"
        }
      },
      "options": {
        "baseURL": "http://localhost:8000/v1"   // 호스트에서 직접 접근
      }
    }
  }
}
```

**다른 모델 추가 시** `models` 객체에 항목 추가:

```jsonc
"models": {
  "gemma-4-31b-it": { "name": "gemma-4-31b-it" },
  "qwen3-32b": { "name": "Qwen3 32B" }         // ← 새 모델 추가
}
```

### 4-3. 호스트 CLI 테스트

```bash
OPENAI_API_KEY=sk-no-key-required \
opencode run --model local_llm/gemma-4-31b-it "안녕하세요. 2+2는?"
```

> 정상이면 `4` 등 간단한 답변이 출력됩니다.

---

## 5. PaperClip 에이전트 설정

PaperClip은 에이전트별로 어댑터와 모델을 DB에 저장합니다.
모델 변경은 **내장 PostgreSQL의 `agents` 테이블**을 직접 수정합니다.

### 5-1. 어댑터 타입 선택

| 어댑터 | CLI | API 방식 | llama-server 호환 |
|--------|-----|----------|------------------|
| `opencode_local` | `opencode` CLI | Chat Completions (`/v1/chat/completions`) | **호환** |
| `codex_local` | `codex` CLI | Responses API (`/v1/responses` WebSocket) | **비호환** |

> **중요:** `codex_local`(Codex CLI)은 OpenAI 전용 Responses API를 사용하므로
> llama-server와 호환되지 않습니다. 로컬 LLM에는 반드시 `opencode_local`을 사용하세요.

### 5-2. DB에서 에이전트 모델 변경

```bash
# PaperClip의 내장 pg 모듈을 사용하여 DB 접근
NODE_PATH="$HOME/.npm-global/lib/node_modules/paperclipai/node_modules" \
node -e "
const { Client } = require('pg');
const c = new Client({
  host: '127.0.0.1',
  port: 54329,
  user: 'paperclip',
  password: 'paperclip',
  database: 'paperclip'
});

async function main() {
  await c.connect();

  // 현재 에이전트 설정 확인
  const res = await c.query('SELECT name, adapter_type, adapter_config FROM agents');
  console.log('=== 현재 에이전트 설정 ===');
  res.rows.forEach(a => {
    console.log(a.name + ': type=' + a.adapter_type + ' model=' + a.adapter_config.model);
  });

  await c.end();
}
main();
"
```

### 5-3. 모델 변경 SQL

```bash
NODE_PATH="$HOME/.npm-global/lib/node_modules/paperclipai/node_modules" \
node -e "
const { Client } = require('pg');
const c = new Client({
  host: '127.0.0.1', port: 54329,
  user: 'paperclip', password: 'paperclip', database: 'paperclip'
});

const NEW_MODEL = 'local_llm/gemma-4-31b-it';   // ← 변경할 모델 (provider/model 형식)

async function main() {
  await c.connect();

  // 어댑터 타입을 opencode_local로 변경하고 모델 업데이트
  await c.query(\`
    UPDATE agents
    SET adapter_type = 'opencode_local',
        adapter_config = jsonb_set(
          jsonb_set(adapter_config, '{model}', '\"' || \\\$1 || '\"'),
          '{dangerouslySkipPermissions}', 'true'
        )
  \`, [NEW_MODEL]);

  // 결과 확인
  const res = await c.query('SELECT name, adapter_type, adapter_config->>$$model$$ as model FROM agents');
  res.rows.forEach(a => console.log(a.name + ' → ' + a.adapter_type + ' / ' + a.model));

  await c.end();
}
main();
"
```

### 5-4. adapter_config 주요 필드

```json
{
  "model": "local_llm/gemma-4-31b-it",
  "graceSec": 15,
  "timeoutSec": 0,
  "dangerouslySkipPermissions": true,
  "instructionsFilePath": "...AGENTS.md",
  "instructionsBundleMode": "managed"
}
```

| 필드 | 설명 |
|------|------|
| `model` | `{provider}/{model_id}` 형식. OpenCode 설정의 provider 키와 models 키 조합 |
| `dangerouslySkipPermissions` | `true`로 설정해야 PaperClip이 승인 없이 자동 실행 |
| `graceSec` | CLI 종료 대기 시간 (초) |
| `timeoutSec` | 작업 타임아웃 (0 = 무제한) |

---

## 6. 전체 검증

### 6-1. llama-server

```bash
curl -s http://localhost:8000/health
# → {"status":"ok"}
```

### 6-2. OpenCode Docker

```bash
docker exec opencode-web opencode run "안녕하세요. 2+2는?" 2>&1 | head -5
```

### 6-3. OpenCode 호스트 CLI

```bash
OPENAI_API_KEY=sk-no-key-required \
opencode run --model local_llm/gemma-4-31b-it "3+3은?"
```

### 6-4. PaperClip 에이전트

```bash
# DB에서 설정 확인
NODE_PATH="$HOME/.npm-global/lib/node_modules/paperclipai/node_modules" \
node -e "
const {Client}=require('pg');
const c=new Client({host:'127.0.0.1',port:54329,user:'paperclip',password:'paperclip',database:'paperclip'});
c.connect().then(()=>c.query('SELECT name,adapter_type,adapter_config->>$$model$$ as model FROM agents'))
.then(r=>{r.rows.forEach(a=>console.log(a.name+': '+a.adapter_type+' / '+a.model));c.end();});
"
```

### 6-5. PaperClip 웹 UI

브라우저에서 `http://localhost:3100` 접속 → 에이전트에게 작업 할당 → 로컬 LLM으로 실행되는지 확인

---

## 7. 참고 정보

### 엔드포인트 요약

| 컨텍스트 | 엔드포인트 | 용도 |
|----------|-----------|------|
| Docker 컨테이너 간 | `http://llm:8000/v1` | OpenCode Docker → llama-server |
| 호스트 | `http://localhost:8000/v1` | PaperClip, 호스트 CLI → llama-server |

### 모델 이름 흐름

```
.env (MODEL_FILENAME)
  → llama-server (--alias = SERVED_MODEL_NAME)
    → OpenCode (opencode.jsonc → models 키)
      → PaperClip (adapter_config.model = "provider/model_id")
```

예시: `gemma-4-31B-it-Q4_K_M.gguf` → `gemma-4-31b-it` → `local_llm/gemma-4-31b-it`

### 파일 위치 요약

| 파일 | 위치 | 역할 |
|------|------|------|
| GGUF 모델 | `./models/*.gguf` | llama-server가 로딩하는 모델 파일 |
| `.env` | `./` | 모델 파일명, alias, 컨텍스트 등 환경변수 |
| `docker-compose.yml` | `./` | llama-server + OpenCode Docker 정의 |
| OpenCode 호스트 설정 | `~/.config/opencode/opencode.jsonc` | 호스트 CLI용 프로바이더 등록 |
| PaperClip 설정 | `./paperclip-data/instances/default/config.json` | PaperClip 서버 설정 |
| PaperClip DB | `./paperclip-data/instances/default/db/` | 에이전트 설정 (PostgreSQL) |

### 관련 문서

- [llama-server 최적화 가이드](llama-server-tuning.md) — 컨텍스트, KV 캐시, reasoning 설정 상세
- [OpenCode 사용 가이드](opencode-guide.md) — 웹 UI 사용법, 도구, 단축키
- [LLM 서버 실행 가이드](vllm-guide.md) — 하드웨어 설정, ROCm, 트러블슈팅
