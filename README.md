# LLM 추론 서버 + OpenCode + PaperClip 스택

**GMKtec EVO-X2 | AMD Ryzen AI MAX+ 395 | Ubuntu 24.04**

---

## 목차

1. [시스템 정보](#1-시스템-정보)
2. [아키텍처 개요](#2-아키텍처-개요)
3. [사전 준비](#3-사전-준비)
4. [모델 다운로드](#4-모델-다운로드)
5. [서비스 실행](#5-서비스-실행)
6. [서비스 상세](#6-서비스-상세)
7. [양자화 및 KV 캐시 설정](#7-양자화-및-kv-캐시-설정)
8. [하드웨어 최적화](#8-하드웨어-최적화)
9. [ROCm GPU 가속 (선택)](#9-rocm-gpu-가속-선택)
10. [트러블슈팅](#10-트러블슈팅)

---

## 1) 시스템 정보

| 항목 | 사양 |
|------|------|
| OS | Ubuntu 24.04.4 LTS |
| 커널 | 6.17.0-20-generic |
| CPU | AMD Ryzen AI MAX+ 395 w/ Radeon 8060S (16C/32T, 최대 5187 MHz) |
| RAM | 128GB LPDDR5 (iGPU와 공유) |
| GPU | AMD Radeon 8060S (iGPU, gfx1151, RDNA 4) |
| GPU 드라이버 | `amdgpu` 커널 모듈 + ROCm 7.2 |

> **참고:** 본 시스템은 iGPU(통합 그래픽)를 사용하며, VRAM은 시스템 메모리에서 할당됩니다.
> BIOS UMA 64GB 고정 + GTT ~105GB 동적 할당으로 운용합니다.

## 2) 아키텍처 개요

```
┌──────────────────────────── Docker Compose ──────────────────────────────────┐
│                                                                              │
│  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐  ┌────────────┐    │
│  │ llama-server   │  │ llama-server  │  │ llama-server  │  │  OpenCode  │    │
│  │ (31B, 고품질)  │  │ (26B MoE)     │  │ (E4B, 고속)   │  │  Web UI    │    │
│  │ :8000          │  │ :8002         │  │ :8001         │  │  :3000     │    │
│  │ gemma-4-31B    │  │ gemma-4-26B   │  │ gemma-4-E4B   │  │  AI Coding │    │
│  │ ~10.5 t/s      │  │ ~30+ t/s      │  │ ~100+ t/s     │  │  Assistant │    │
│  └───────┬───────┘  └───────┬───────┘  └───────┬───────┘  └────────────┘    │
│          │                  │                   │                             │
│          │  [llm-network bridge]                │                             │
└──────────┼──────────────────┼───────────────────┼────────────────────────────┘
           │                  │                   │
           │ :8000/v1         │ :8002/v1          │ :8001/v1
           │                  │                   │
┌──────────┼──────────────────┼───────────────────┼──── 호스트 (npm 로컬) ─────┐
│          │                  │                   │                             │
│  ┌───────▼──────────────────▼───────────────────▼──┐  ┌────────────────┐     │
│  │   PaperClip (paperclipai)                       │  │  기타 도구      │     │
│  │   :3100 — AI Orchestrator                       │  │                │     │
│  │   에이전트 → 31B / 26B MoE / E4B 자유 선택       │  │                │     │
│  │   내장 PostgreSQL                               │  │                │     │
│  └─────────────────────────────────────────────────┘  └────────────────┘     │
└──────────────────────────────────────────────────────────────────────────────┘
```

| 구성요소 | 실행 환경 | 포트 | 역할 |
|----------|-----------|------|------|
| llama-server (31B) | Docker (ROCm 7.2) | 8000 | LLM 추론 서버 — 고품질 Dense (OpenAI 호환 API) |
| llama-server (26B MoE) | Docker (ROCm 7.2) | 8002 | LLM 추론 서버 — MoE 균형형 (OpenAI 호환 API) |
| llama-server (E4B) | Docker (ROCm 7.2) | 8001 | LLM 추론 서버 — 고속 (OpenAI 호환 API) |
| OpenCode | Docker | 3000 | AI 코딩 어시스턴트 웹 UI |
| PaperClip | npm 로컬 | 3100 | AI 오케스트레이션 플랫폼 (내장 PostgreSQL) |

## 3) 사전 준비

### 3-1. 하드웨어 최적화 (선택)

```bash
cd /home/aa/vllm
sudo ./setup_evo_x2_hardware.sh
sudo reboot
```

이 스크립트는 다음을 수행합니다:
- ROCm/OpenCL/Vulkan 도구 설치
- `amdgpu` 성능 플래그 적용
- CPU governor를 `performance`로 설정
- `render`, `video` 그룹 추가

### 3-2. Docker 설치

```bash
chmod +x install_docker.sh
sudo ./install_docker.sh
```

설치 후 docker 그룹 적용:

```bash
newgrp docker
# 또는 로그아웃 후 재로그인
```

### 3-3. Node.js 및 npm 도구 설치

```bash
# Node.js 22 설치 (이미 설치되어 있으면 건너뛰기)
node --version  # v22.x 확인

# npm 전역 디렉토리 설정 (sudo 없이 설치)
mkdir -p ~/.npm-global
npm config set prefix '~/.npm-global'
echo 'export PATH="$HOME/.npm-global/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# PaperClip CLI 설치
npm install -g paperclipai

# Codex CLI 설치 (선택)
npm install -g @openai/codex
```

### 3-4. 작업 디렉토리 생성

```bash
mkdir -p models workspace
```

## 4) 모델 다운로드

다운로드 스크립트로 원하는 모델을 선택하여 다운로드:

```bash
chmod +x download_model.sh
./download_model.sh
# 메뉴:
# 1) Gemma 4 31B IT (Dense, ~18GB)     — 고품질, 느림
# 2) Gemma 4 26B-A4B IT (MoE, ~17GB)   — 균형 (빠름 + 고품질) ← 추천
# 3) Gemma 4 E4B IT (~5GB)             — 경량, 최고 속도
# 4) 전체 다운로드
```

또는 수동 다운로드:

```bash
pip install huggingface_hub[cli]

# 31B Dense (고품질)
hf download unsloth/gemma-4-31B-it-GGUF \
    gemma-4-31B-it-Q4_K_M.gguf --local-dir ./models

# 26B-A4B MoE (균형, 추천)
hf download bartowski/google_gemma-4-26B-A4B-it-GGUF \
    google_gemma-4-26B-A4B-it-Q4_K_M.gguf --local-dir ./models

# E4B (고속)
hf download unsloth/gemma-4-E4B-it-GGUF \
    gemma-4-E4B-it-Q4_K_M.gguf --local-dir ./models
```

> **모델 소스:**
> - 31B: [`unsloth/gemma-4-31B-it-GGUF`](https://huggingface.co/unsloth/gemma-4-31B-it-GGUF)
> - 26B-A4B: [`bartowski/google_gemma-4-26B-A4B-it-GGUF`](https://huggingface.co/bartowski/google_gemma-4-26B-A4B-it-GGUF) (MoE, 25.2B total / 3.8B active)
> - E4B: [`unsloth/gemma-4-E4B-it-GGUF`](https://huggingface.co/unsloth/gemma-4-E4B-it-GGUF)

## 5) 서비스 실행

### Docker 서비스 (LLM + OpenCode)

```bash
# LLM 서버 + OpenCode 시작
docker compose up -d

# 상태 확인
docker compose ps
docker compose logs -f llm
```

### PaperClip (npm 로컬)

```bash
# 최초 실행 — 초기화 + 서버 시작
paperclipai onboard --yes --data-dir ./paperclip-data

# 이후 실행
paperclipai run --data-dir ./paperclip-data

# LLM 설정 변경
paperclipai configure --section llm --data-dir ./paperclip-data

# 진단
paperclipai doctor --data-dir ./paperclip-data
```

### 개별 Docker 서비스

```bash
# LLM만 시작 (31B)
docker compose up -d llm

# MoE만 시작 (26B)
docker compose up -d llm-moe

# OpenCode만 시작 (LLM 의존)
docker compose up -d opencode
```

### 중지

```bash
# Docker 서비스 중지
docker compose down

# PaperClip 중지: Ctrl+C 또는 프로세스 종료
```

## 6) 서비스 상세

### 6-1. LLM 추론 서버 (트리플 모델 구성)

세 개의 llama-server가 동시에 실행됩니다:

| 서버 | 모델 | 포트 | 파라미터 | 슬롯 | 용도 |
|------|------|------|---------|------|------|
| `llm` (31B) | Gemma 4 31B IT Q4_K_M | 8000 | 30.7B Dense | 2 (128K/슬롯) | 복잡한 코딩, 긴 추론 |
| `llm-moe` (26B) | Gemma 4 26B-A4B IT Q4_K_M | 8002 | 25.2B MoE (3.8B active) | 4 (32K/슬롯) | 균형형 (128 experts, 8 active) |
| `llm-fast` (E4B) | Gemma 4 E4B IT Q4_K_M | 8001 | 4.5B Effective | 4 (32K/슬롯) | 빠른 응답, 단순 작업 |

- **이미지:** `kyuz0/amd-strix-halo-toolboxes:rocm-7.2`
- **GPU:** ROCm 7.2 가속 (gfx1151 네이티브 지원)

API 테스트:

```bash
# 31B 모델 (고품질)
curl http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gemma-4-31b-it",
    "messages": [{"role": "user", "content": "안녕하세요"}],
    "max_tokens": 128
  }' | jq

# 26B MoE 모델 (균형)
curl http://localhost:8002/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gemma-4-26b-a4b-it",
    "messages": [{"role": "user", "content": "안녕하세요"}],
    "max_tokens": 128
  }' | jq

# E4B 모델 (고속)
curl http://localhost:8001/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gemma-4-e4b-it",
    "messages": [{"role": "user", "content": "안녕하세요"}],
    "max_tokens": 128
  }' | jq
```

### 6-2. OpenCode (AI 코딩 어시스턴트)

- **웹 UI:** `http://localhost:3000`
- **LLM 연동:** 31B (`http://llm:8000/v1`) + 26B MoE (`http://llm-moe:8000/v1`) + E4B (`http://llm-fast:8000/v1`)
- **작업 디렉토리:** `./workspace` 마운트
- **설정 영속화:** `./opencode-config/` 볼륨 마운트

OpenCode UI에서 `Ctrl+O`로 모델 선택 시 세 모델이 모두 표시됩니다.

### 6-3. PaperClip (AI 오케스트레이션 — npm 로컬)

- **웹 UI:** `http://localhost:3100`
- **데이터베이스:** 내장 PostgreSQL (자동 구성, 포트 54329)
- **인증:** `local_trusted` 모드 (로컬 접속 자동 신뢰)
- **데이터 디렉토리:** `./paperclip-data/`
- **설정 파일:** `./paperclip-data/instances/default/config.json`

PaperClip은 Docker가 아닌 npm 로컬로 실행됩니다. Codex CLI, 로컬 LLM 등
호스트 도구와의 연동이 자유롭습니다.

```bash
# LLM 설정 (config.json의 llm 섹션)
paperclipai configure --section llm --data-dir ./paperclip-data

# 모델 변경, codex 어댑터 등 자유롭게 설정 가능
```

#### PaperClip 에이전트의 LLM 모델 변경

에이전트가 사용하는 모델을 26B MoE로 변경하려면 PaperClip 웹 UI에서
에이전트 설정의 `model` 값을 변경합니다:

| 모델 참조 (OpenCode adapter) | 설명 |
|-----|------|
| `local_llm/gemma-4-31b-it` | 31B Dense — 고품질, 느림 |
| `local_llm_moe/gemma-4-26b-a4b-it` | **26B MoE — 균형 (추천)** |
| `local_llm_fast/gemma-4-e4b-it` | E4B — 경량, 빠름 |

```bash
# 또는 CLI로 에이전트 어댑터 설정 변경
paperclipai configure --section llm --data-dir ./paperclip-data
```

### 6-4. Codex CLI (선택)

- **인증:** ChatGPT Plus (Google OAuth)
- **설정:** `~/.codex/config.toml`

```bash
# 인증
codex auth login

# 사용
codex "설명할 내용"
```

## 7) 양자화 및 모델 설정

### 현재 적용된 최적화

| 항목 | 31B (llm) | 26B MoE (llm-moe) | E4B (llm-fast) |
|------|-----------|-------------------|----------------|
| 모델 양자화 | Q4_K_M (~18GB) | Q4_K_M (~17GB) | Q4_K_M (~5GB) |
| 아키텍처 | Dense 30.7B | MoE 25.2B (3.8B active) | Dense 4.5B eff |
| GPU 오프로드 | `-ngl 999` (전 레이어) | `-ngl 999` (전 레이어) | `-ngl 999` (전 레이어) |
| Flash Attention | `-fa on` | `-fa on` | `-fa on` |
| KV 캐시 | q8_0 (K, V) | q8_0 (K, V) | q8_0 (K, V) |
| 컨텍스트/슬롯 | 262K / 2슬롯 (128K/슬롯) | 131K / 4슬롯 (32K/슬롯) | 131K / 4슬롯 (32K/슬롯) |
| CPU 스레드 | 32 | 32 | 16 |
| Reasoning | `--reasoning-format deepseek` | `--reasoning-format deepseek` | `--reasoning-format deepseek` |
| 토큰 생성 속도 | ~10.5 t/s | ~30+ t/s (예상) | ~100+ t/s |
| 메모리 제한 | 96GB | 48GB | 16GB |

> **GGUF Q4_K_M 선택 이유:** 128GB 공유 메모리 시스템에서 세 모델을 동시 실행하면서
> 품질 손실을 최소화하는 최적의 균형점입니다.
>
> **26B MoE 특징:** 총 25.2B 파라미터 중 추론 시 3.8B만 활성화 (128 experts 중 8개 선택).
> 31B급 품질을 유지하면서 E4B에 가까운 속도를 제공합니다.

## 8) 하드웨어 최적화

### CPU 성능 최적화

```bash
# CPU governor 확인
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor

# performance 모드로 변경 (setup_evo_x2_hardware.sh에 포함)
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

### 메모리 최적화

```bash
# 대용량 페이지 활성화
echo 'vm.nr_hugepages=1024' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

### iGPU 메모리 구조

| 메모리 | 크기 | 용도 |
|--------|------|------|
| VRAM (UMA) | 64GB | BIOS 고정 할당, GPU 프레임버퍼 |
| GTT | ~105GB | 동적 할당, 모델 텐서 로딩 |
| 시스템 | 128GB | 전체 물리 메모리 (VRAM+GTT 공유) |

## 9) ROCm GPU 가속

현재 아키텍처는 `kyuz0/amd-strix-halo-toolboxes:rocm-7.2` 이미지로
ROCm 7.2 GPU 가속을 기본 사용합니다. gfx1151 네이티브 지원으로
`HSA_OVERRIDE_GFX_VERSION` 설정이 불필요합니다.

## 10) 트러블슈팅

### LLM 서버 시작이 느림

ROCm GPU 모드에서 31B 모델 로딩에 2~3분 소요될 수 있습니다.
`start_period: 180s` 헬스체크 설정으로 충분한 시간을 부여합니다.

```bash
docker compose logs -f llm
```

### 메모리 부족 (OOM)

```bash
# 컨텍스트 길이 줄이기 (.env)
CONTEXT_LENGTH=4096

# 컨테이너 메모리 제한 조정 (.env)
LLM_MEMORY_LIMIT=64g
```

### OpenCode가 LLM에 연결 실패

```bash
# LLM 상태 확인
curl http://localhost:8000/health

# LLM이 정상이면 OpenCode 재시작
docker compose restart opencode
```

### PaperClip 문제

```bash
# 진단 실행
paperclipai doctor --data-dir ./paperclip-data

# 설정 재구성
paperclipai configure --data-dir ./paperclip-data
```

---

## 파일 구조

```
/home/aa/vllm/
├── docker-compose.yml          # Docker Compose (LLM + OpenCode)
├── Dockerfile.llama-rocm       # llama.cpp ROCm 커스텀 빌드 (선택)
├── Dockerfile.vllm             # vLLM CPU 모드 이미지 (레거시)
├── Dockerfile.vllm-rocm        # vLLM ROCm 이미지 (레거시)
├── download_model.sh           # GGUF 모델 다운로드 스크립트
├── install_docker.sh           # Docker Engine 설치 스크립트
├── setup_evo_x2_hardware.sh    # 하드웨어 최적화 스크립트
├── models/                     # GGUF 모델 파일
│   ├── gemma-4-31B-it-Q4_K_M.gguf            # 31B 고품질 (~18GB)
│   ├── google_gemma-4-26B-A4B-it-Q4_K_M.gguf # 26B MoE 균형형 (~17GB)
│   └── gemma-4-E4B-it-Q4_K_M.gguf            # E4B 고속 (~5GB)
├── opencode-config/            # OpenCode 설정 (Docker 마운트)
│   └── opencode.jsonc          # 프로바이더/모델 등록
├── workspace/                  # OpenCode 작업 디렉토리
├── paperclip-data/             # PaperClip 로컬 데이터 (npm)
│   └── instances/default/
│       ├── config.json         # PaperClip 설정
│       ├── db/                 # 내장 PostgreSQL 데이터
│       └── data/               # 스토리지, 백업
├── doc/                        # 가이드 문서
└── README.md                   # 이 문서
```

## 버전 정보

| 구성요소 | 버전 | 실행 환경 |
|----------|------|-----------|
| llama-server (31B) | ROCm 7.2 | Docker 포트 8000 (`kyuz0/amd-strix-halo-toolboxes`) |
| llama-server (26B MoE) | ROCm 7.2 | Docker 포트 8002 (`kyuz0/amd-strix-halo-toolboxes`) |
| llama-server (E4B) | ROCm 7.2 | Docker 포트 8001 (`kyuz0/amd-strix-halo-toolboxes`) |
| Gemma 4 31B IT | Q4_K_M GGUF (~18GB) | 고품질, 30.7B Dense |
| Gemma 4 26B-A4B IT | Q4_K_M GGUF (~17GB) | MoE 균형형, 25.2B total / 3.8B active |
| Gemma 4 E4B IT | Q4_K_M GGUF (~5GB) | 고속, 4.5B Effective (8B total) |
| OpenCode | latest | Docker (`smanx/opencode`) |
| PaperClip | latest | npm 로컬 (`paperclipai`) |
| Node.js | v22.22.2 | 호스트 |
| Docker Compose | v2 | 호스트 |
