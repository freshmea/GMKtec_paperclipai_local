# 로컬 LLM 등록 및 PaperClip 연동 가이드

이 문서는 현재 레포의 기본 조합인 `Gemma 4 26B-A4B + Qwen3.6 35B-A3B`를 기준으로
llama-server, OpenCode, PaperClip을 한 번에 맞추는 방법을 정리합니다.

## 목표 구성

| 역할 | 모델 | 엔드포인트 |
|------|------|-----------|
| shorter/mid-context | `gemma-4-26b-a4b-it` | `http://localhost:8002/v1` |
| long-context/technical | `qwen3-6-35b-a3b` | `http://localhost:8003/v1` |
| optional legacy | `gemma-4-31b-it` | `http://localhost:8000/v1` |

## 1. 모델 다운로드

```bash
cd /home/aa/vllm

hf download bartowski/google_gemma-4-26B-A4B-it-GGUF \
  google_gemma-4-26B-A4B-it-Q4_K_M.gguf --local-dir ./models

hf download unsloth/Qwen3.6-35B-A3B-GGUF \
  Qwen3.6-35B-A3B-UD-Q4_K_M.gguf --local-dir ./models
```

Qwen 양자화 기준:

- 기본 선택: `Qwen3.6-35B-A3B-UD-Q4_K_M.gguf`
- 이유: 현재 스택이 `llama-server + GGUF` 기반이며, 128GB 공유 메모리 환경에서 품질과 메모리 균형이 가장 무난함

## 2. Docker Compose 기준값

현재 레포의 기본 서비스는 다음 셋입니다.

```bash
docker compose up -d llm-moe llm-qwen opencode
```

핵심 포트:

- `llm-moe` -> `localhost:8002`
- `llm-qwen` -> `localhost:8003`
- `opencode` -> `localhost:3000`

`.env` 핵심값:

```env
MOE_MODEL_FILENAME=google_gemma-4-26B-A4B-it-Q4_K_M.gguf
MOE_SERVED_MODEL_NAME=gemma-4-26b-a4b-it
MOE_CONTEXT_LENGTH=65536
MOE_PARALLEL_SLOTS=1

QWEN_MODEL_FILENAME=Qwen3.6-35B-A3B-UD-Q4_K_M.gguf
QWEN_SERVED_MODEL_NAME=qwen3-6-35b-a3b
QWEN_CONTEXT_LENGTH=131072
QWEN_PARALLEL_SLOTS=1
```

운영 원칙:

- `Gemma 26B`는 짧거나 중간 길이 문맥 위주
- `Qwen3.6`은 긴 문맥과 기술 작업 전담
- `Qwen3.6` 서버는 `--cache-ram 0`으로 prompt cache를 끄는 구성을 사용

## 3. OpenCode 설정

### 3-1. Docker 내부 설정

파일: `opencode-config/opencode.jsonc`

```jsonc
{
  "$schema": "https://opencode.ai/config.json",
  "disabled_providers": ["local_llm", "local_llm_fast"],
  "provider": {
    "local_llm_moe": {
      "name": "Local LLM — Gemma 4 26B-A4B",
      "npm": "@ai-sdk/openai-compatible",
      "models": {
        "gemma-4-26b-a4b-it": {
          "name": "Gemma 4 26B-A4B (shorter/mid-context)",
          "limit": { "context": 65536, "output": 8192 }
        }
      },
      "options": {
        "baseURL": "http://llm-moe:8000/v1"
      }
    },
    "local_llm_qwen": {
      "name": "Local LLM — Qwen3.6 35B-A3B",
      "npm": "@ai-sdk/openai-compatible",
      "models": {
        "qwen3-6-35b-a3b": {
          "name": "Qwen3.6 35B-A3B (long-context, technical)",
          "limit": { "context": 131072, "output": 8192 }
        }
      },
      "options": {
        "baseURL": "http://llm-qwen:8000/v1"
      }
    }
  }
}
```

### 3-2. 호스트 CLI 설정

파일: `~/.config/opencode/opencode.jsonc`

```jsonc
{
  "$schema": "https://opencode.ai/config.json",
  "disabled_providers": ["local_llm", "local_llm_fast"],
  "provider": {
    "local_llm_moe": {
      "name": "Local LLM (llama.cpp) — Gemma 4 26B-A4B",
      "npm": "@ai-sdk/openai-compatible",
      "models": {
        "gemma-4-26b-a4b-it": {
          "name": "Gemma 4 26B-A4B (shorter/mid-context)",
          "limit": { "context": 65536, "output": 8192 }
        }
      },
      "options": {
        "baseURL": "http://localhost:8002/v1"
      }
    },
    "local_llm_qwen": {
      "name": "Local LLM (llama.cpp) — Qwen3.6 35B-A3B",
      "npm": "@ai-sdk/openai-compatible",
      "models": {
        "qwen3-6-35b-a3b": {
          "name": "Qwen3.6 35B-A3B (long-context, technical)",
          "limit": { "context": 131072, "output": 8192 }
        }
      },
      "options": {
        "baseURL": "http://localhost:8003/v1"
      }
    }
  }
}
```

## 4. 서비스 재기동

```bash
cd /home/aa/vllm
docker compose up -d --force-recreate llm-moe llm-qwen opencode
```

준비 확인:

```bash
until curl -sf http://localhost:8002/health >/dev/null 2>&1 && \
      curl -sf http://localhost:8003/health >/dev/null 2>&1; do
  sleep 5
done
echo "Gemma 26B + Qwen3.6 ready"
```

## 5. 모델 검증

```bash
curl -s http://localhost:8002/v1/models | python3 -m json.tool
curl -s http://localhost:8003/v1/models | python3 -m json.tool
```

```bash
OPENAI_API_KEY=sk-no-key-required \
opencode run --model local_llm_moe/gemma-4-26b-a4b-it "짧은 문맥 기본 모델인지 한 줄로 설명해줘."

OPENAI_API_KEY=sk-no-key-required \
opencode run --model local_llm_qwen/qwen3-6-35b-a3b "120k 문맥 기술 작업용 기본 모델인지 한 줄로 설명해줘."
```

## 6. PaperClip 에이전트 라우팅

현재 권장 라우팅:

- `CMO`, `UXDesigner` -> `local_llm_moe/gemma-4-26b-a4b-it`
- `ROS2 Expert`, `Robot Research Agent`, `Rust Expert` -> `local_llm_qwen/qwen3-6-35b-a3b`
- `CEO`, `CTO` -> 기존 `codex_local / gpt-5.4` 유지

현재 상태 확인:

```bash
PGPASSWORD=paperclip psql -h 127.0.0.1 -p 54329 -U paperclip -d paperclip \
  -P pager=off -c "SELECT name, adapter_type, adapter_config->>'model' AS model FROM agents ORDER BY name;"
```

Gemma 26B로 보낼 에이전트:

```bash
PGPASSWORD=paperclip psql -h 127.0.0.1 -p 54329 -U paperclip -d paperclip <<'SQL'
UPDATE agents
SET adapter_type = 'opencode_local',
    adapter_config = jsonb_set(
      jsonb_set(adapter_config, '{model}', to_jsonb('local_llm_moe/gemma-4-26b-a4b-it'::text)),
      '{dangerouslySkipPermissions}',
      'true'::jsonb
    )
WHERE name IN ('CMO', 'UXDesigner');
SQL
```

Qwen3.6으로 보낼 에이전트:

```bash
PGPASSWORD=paperclip psql -h 127.0.0.1 -p 54329 -U paperclip -d paperclip <<'SQL'
UPDATE agents
SET adapter_type = 'opencode_local',
    adapter_config = jsonb_set(
      jsonb_set(adapter_config, '{model}', to_jsonb('local_llm_qwen/qwen3-6-35b-a3b'::text)),
      '{dangerouslySkipPermissions}',
      'true'::jsonb
    )
WHERE name IN ('ROS2 Expert', 'Robot Research Agent', 'Rust Expert');
SQL
```

결과 확인:

```bash
PGPASSWORD=paperclip psql -h 127.0.0.1 -p 54329 -U paperclip -d paperclip \
  -P pager=off -c "SELECT name, adapter_type, adapter_config->>'model' AS model FROM agents ORDER BY name;"
```

## 7. 엔드포인트 요약

| 위치 | 엔드포인트 | 용도 |
|------|-----------|------|
| Docker 내부 | `http://llm-moe:8000/v1` | OpenCode -> Gemma 26B |
| Docker 내부 | `http://llm-qwen:8000/v1` | OpenCode -> Qwen3.6 |
| 호스트 | `http://localhost:8002/v1` | PaperClip, 호스트 CLI -> Gemma 26B |
| 호스트 | `http://localhost:8003/v1` | PaperClip, 호스트 CLI -> Qwen3.6 |

## 8. 주의사항

- `codex_local`은 OpenAI Responses API 전용이므로 llama-server와 직접 호환되지 않습니다.
- 긴 문맥 작업은 반드시 `Qwen3.6` 쪽 provider를 사용해야 합니다.
- `Gemma 31B`는 optional legacy로만 남겨두고 기본 라우팅에서는 제외합니다.
