# LLM 추론 서버 + OpenCode + PaperClip 스택

**GMKtec EVO-X2 | AMD Ryzen AI MAX+ 395 | Ubuntu 24.04**

이 레포의 현재 기본 운영 조합은 `Gemma 4 26B-A4B + Qwen3.6 35B-A3B` 입니다.

- `Gemma 4 26B-A4B`: 짧거나 중간 길이 문맥, 빠른 일반 작업
- `Qwen3.6 35B-A3B`: 긴 문맥, 기술 리서치, PaperClip 핵심 전문가 에이전트
- `Gemma 4 31B`: 기본 경로에서는 사용하지 않으며 필요 시 legacy profile로만 실행

`Gemma 26B`는 이 레포에서 의도적으로 `65,536` 토큰으로 제한합니다. 100K급 장문 작업은 `Qwen3.6`으로 라우팅하는 구성이 기본값입니다.

## 아키텍처

```text
Docker Compose
  llm-moe     -> Gemma 4 26B-A4B      -> http://localhost:8002/v1
  llm-qwen    -> Qwen3.6 35B-A3B      -> http://localhost:8003/v1
  opencode    -> Web UI               -> http://localhost:3000
  comfyui     -> Image UI/API         -> http://localhost:3001

Host (npm local)
  paperclipai -> AI orchestrator      -> http://localhost:3100
```

| 구성요소 | 포트 | 역할 |
|----------|------|------|
| `llm-moe` | 8002 | Gemma 4 26B-A4B, shorter/mid-context |
| `llm-qwen` | 8003 | Qwen3.6 35B-A3B, long-context/technical |
| `opencode` | 3000 | AI 코딩 어시스턴트 웹 UI |
| `comfyui` | 3001 | ComfyUI Web UI + Local API |
| `paperclipai` | 3100 | 로컬 오케스트레이션 플랫폼 |

## 빠른 시작

### 1. 모델 다운로드

```bash
cd /home/aa/vllm
chmod +x download_model.sh
./download_model.sh
```

권장 메뉴:

```text
1) Gemma 4 26B-A4B IT (MoE, ~16GB)
2) Qwen3.6 35B-A3B UD-Q4_K_M (~22.1GB)
3) Gemma 4 26B + Qwen3.6 (hybrid)  <- 기본 추천
```

수동 다운로드:

```bash
hf download bartowski/google_gemma-4-26B-A4B-it-GGUF \
  google_gemma-4-26B-A4B-it-Q4_K_M.gguf --local-dir ./models

hf download unsloth/Qwen3.6-35B-A3B-GGUF \
  Qwen3.6-35B-A3B-UD-Q4_K_M.gguf --local-dir ./models
```

모델 소스:

- Gemma 26B GGUF: `bartowski/google_gemma-4-26B-A4B-it-GGUF`
- Qwen3.6 GGUF: `unsloth/Qwen3.6-35B-A3B-GGUF`
- optional legacy 31B: `unsloth/gemma-4-31B-it-GGUF`

### 2. 서비스 실행

```bash
docker compose up -d llm-moe llm-qwen opencode comfyui
docker compose ps
```

legacy 모델이 필요할 때만:

```bash
docker compose --profile legacy up -d llm llm-fast
```

### 3. 헬스체크

```bash
curl http://localhost:8002/health
curl http://localhost:8003/health
curl http://localhost:3000
curl http://localhost:3001
```

### 4. 간단한 추론 테스트

```bash
curl http://localhost:8002/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gemma-4-26b-a4b-it",
    "messages": [{"role": "user", "content": "짧은 문맥용 모델인지 한 줄로 설명해줘."}],
    "max_tokens": 64
  }' | jq

curl http://localhost:8003/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "qwen3-6-35b-a3b",
    "messages": [{"role": "user", "content": "120k 문맥이 필요한 기술 작업에 적합한지 한 줄로 설명해줘."}],
    "max_tokens": 64
  }' | jq
```

## 모델 운영 정책

| 모델 | 양자화 | 포트 | 컨텍스트 | 용도 |
|------|--------|------|----------|------|
| Gemma 4 26B-A4B | `Q4_K_M` | 8002 | `65,536` | 일반 작업, shorter/mid-context |
| Qwen3.6 35B-A3B | `UD-Q4_K_M` | 8003 | `131,072` | long-context, 기술 작업 |
| Gemma 4 31B | `Q4_K_M` | 8000 | `65,536` | optional legacy |
| Gemma 4 E4B | `Q4_K_M` | 8001 | `32,768` per-slot | optional legacy |

핵심 포인트:

- `Gemma 26B`는 이 스택에서 long-context 주력으로 쓰지 않습니다.
- `Qwen3.6-35B-A3B-UD-Q4_K_M.gguf`를 기본 Qwen 양자화로 사용합니다.
- `Qwen`은 안정성 우선으로 `-np 1`과 `--cache-ram 0`을 적용합니다.
- `Gemma 26B`도 컨텍스트를 `65,536`으로 제한해 역할을 분리합니다.

## OpenCode

웹 UI:

- `http://localhost:3000`

Docker 내부 OpenCode 설정은 `./opencode-config/opencode.jsonc`를 사용합니다.
기본 프로바이더는 다음 둘입니다.

- `local_llm_moe/gemma-4-26b-a4b-it`
- `local_llm_qwen/qwen3-6-35b-a3b`

legacy provider:

- `local_llm/gemma-4-31b-it`
- `local_llm_fast/gemma-4-e4b-it`

## ComfyUI

웹 UI / API:

- `http://localhost:3001`

현재 설정:

- 호스트 포트 `3001` -> 컨테이너 `8188`
- listen 주소 `0.0.0.0`
- 외부 공유 시에는 라우터에서 `30006 -> 호스트 3001`로 포트포워딩

실행:

```bash
docker compose up -d comfyui
docker compose logs -f comfyui
```

ComfyUI 모델/입출력 경로:

- `./comfyui/models`
- `./comfyui/input`
- `./comfyui/output`
- `./comfyui/custom_nodes`
- `./comfyui/user`

API 확인 예시:

```bash
curl http://localhost:3001/system_stats
curl http://localhost:3001/object_info
```

워크플로우 실행은 ComfyUI에서 `Save (API Format)`으로 저장한 JSON을 `POST /prompt`로 보내면 됩니다.

## PaperClip

PaperClip은 Docker가 아니라 호스트에서 `npm` 로컬 설치로 운영합니다.

```bash
paperclipai onboard --yes --data-dir ./paperclip-data
paperclipai run --data-dir ./paperclip-data
```

권장 라우팅:

| 에이전트 | 모델 |
|----------|------|
| `CMO`, `UXDesigner` | `local_llm_moe/gemma-4-26b-a4b-it` |
| `ROS2 Expert`, `Robot Research Agent`, `Rust Expert` | `local_llm_qwen/qwen3-6-35b-a3b` |
| `CEO`, `CTO` | 기존 `codex_local / gpt-5.4` 유지 |

실제 DB 반영 방법은 [doc/opencode-local-llm-setup.md](doc/opencode-local-llm-setup.md)에 정리되어 있습니다.

## 주요 파일

```text
/home/aa/vllm/
├── docker-compose.yml
├── .env
├── download_model.sh
├── opencode-config/opencode.jsonc
├── models/
├── workspace/
├── paperclip-data/
└── doc/
```

## 트러블슈팅

### Qwen 컨테이너가 뜨지 않을 때

대부분 모델 파일이 아직 없거나 이름이 맞지 않을 때입니다.

```bash
ls -lh ./models
docker compose logs -f llm-qwen
```

### OpenCode에서 모델이 안 보일 때

```bash
./verify_local_llm_stack.sh
docker compose restart opencode
cat opencode-config/opencode.jsonc
```

이 스크립트는 다음을 한 번에 점검합니다.

- `llm-moe`, `llm-qwen`의 실제 `n_ctx`
- repo/host/container OpenCode config 정합성
- `opencode models`에 Gemma/Qwen provider가 모두 노출되는지
- PaperClip agent 라우팅이 Gemma/Qwen 정책과 일치하는지
- 현재 셸의 `MOE_CONTEXT_LENGTH` 같은 override 환경변수가 `.env`를 덮는지

### PaperClip에서 긴 문맥 작업이 끊길 때

- 해당 에이전트가 `local_llm_qwen/qwen3-6-35b-a3b`를 쓰는지 확인
- `~/.config/opencode/opencode.jsonc`와 repo `opencode-config/opencode.jsonc`가 같은 정책인지 확인
- `curl http://localhost:8003/props`로 실제 `n_ctx` 확인
- `./verify_local_llm_stack.sh`로 stale config/provider 누락 여부를 먼저 확인
- `docker compose config`에서 기대값과 다르면 `unset MOE_CONTEXT_LENGTH QWEN_CONTEXT_LENGTH` 후 재생성

### legacy 31B가 필요할 때

```bash
docker compose --profile legacy up -d llm
```

기본 경로에서는 사용하지 않으므로 OpenCode에서 disabled 상태로 등록되어 있습니다.
