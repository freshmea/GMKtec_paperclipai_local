# vLLM + OpenCode + PaperClip Docker Compose 스택

**GMKtec EVO-X2 | AMD Ryzen AI MAX+ 395 | Ubuntu 24.04**

---

## 목차

1. [시스템 정보](#1-시스템-정보)
2. [아키텍처 개요](#2-아키텍처-개요)
3. [사전 준비](#3-사전-준비)
4. [모델 다운로드](#4-모델-다운로드)
5. [Docker Compose 실행](#5-docker-compose-실행)
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
| RAM | 64GB (iGPU와 공유) |
| GPU | AMD Radeon 8060S (iGPU, gfx1150, RDNA 4) |
| GPU 드라이버 | `amdgpu` 커널 모듈 |

> **참고:** 본 시스템은 iGPU(통합 그래픽)를 사용하며, VRAM은 시스템 메모리에서 할당됩니다.
> 128GB VRAM 사용을 위해서는 물리 RAM 128GB + BIOS UMA 설정이 필요합니다.

## 2) 아키텍처 개요

```
┌─────────────────────────────────────────────────────────────┐
│                    Docker Compose Stack                       │
│                                                               │
│  ┌─────────────┐   ┌──────────────┐   ┌──────────────────┐  │
│  │   vLLM       │   │  OpenCode    │   │   PaperClip      │  │
│  │  :8000       │   │  :3000       │   │   :3100           │  │
│  │  OpenAI API  │◄──│  AI Coding   │   │   AI Orchestrator │  │
│  │  Gemma 4 31B │   │  Assistant   │   │                   │  │
│  │  Q4_K_M GGUF │   │              │   │  ┌─────────────┐ │  │
│  └─────────────┘   └──────────────┘   │  │ PostgreSQL  │ │  │
│                                        │  │ :5432       │ │  │
│                                        │  └─────────────┘ │  │
│                                        └──────────────────┘  │
│                                                               │
│                     [llm-network bridge]                      │
└─────────────────────────────────────────────────────────────┘
```

| 서비스 | 이미지 | 포트 | 역할 |
|--------|--------|------|------|
| vllm | 커스텀 빌드 (Dockerfile.vllm) | 8000 | Gemma 4 31B 추론 서버 (OpenAI 호환 API) |
| opencode | `smanx/opencode:latest` | 3000 | AI 코딩 어시스턴트 웹 UI |
| paperclip | `ghcr.io/paperclipai/paperclip:latest` | 3100 | AI 오케스트레이션 플랫폼 |
| paperclip-db | `postgres:17-alpine` | (내부) | PaperClip 데이터베이스 |

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

검증:

```bash
docker --version
docker compose version
docker run hello-world
```

### 3-3. 작업 디렉토리 생성

```bash
mkdir -p models workspace
```

## 4) 모델 다운로드

Gemma 4 31B IT GGUF (Q4_K_M, 약 18GB) 다운로드:

```bash
chmod +x download_model.sh
./download_model.sh
```

또는 수동 다운로드:

```bash
pip install huggingface_hub[cli]
huggingface-cli download \
    unsloth/gemma-4-31B-it-GGUF \
    gemma-4-31B-it-Q4_K_M.gguf \
    --local-dir ./models \
    --local-dir-use-symlinks False
```

> **모델 소스:** [`unsloth/gemma-4-31B-it-GGUF`](https://huggingface.co/unsloth/gemma-4-31B-it-GGUF) (274k+ 다운로드)
>
> 대안: `bartowski/google_gemma-4-31B-it-GGUF`, `lmstudio-community/gemma-4-31B-it-GGUF`

## 5) Docker Compose 실행

### 환경변수 설정

`.env` 파일이 프로젝트 루트에 포함되어 있습니다. 필요에 따라 수정하세요:

```bash
# 주요 설정 확인/수정
vi .env
```

### 전체 스택 시작

```bash
# vLLM Docker 이미지 빌드 + 전체 서비스 시작
docker compose up -d --build
```

### 개별 서비스 시작

```bash
# vLLM만 시작
docker compose up -d vllm

# OpenCode만 시작 (vLLM 의존)
docker compose up -d opencode

# PaperClip만 시작
docker compose up -d paperclip
```

### 상태 확인

```bash
docker compose ps
docker compose logs -f vllm
docker compose logs -f opencode
docker compose logs -f paperclip
```

### 중지

```bash
docker compose down
# 볼륨 포함 완전 삭제:
# docker compose down -v
```

## 6) 서비스 상세

### 6-1. vLLM (LLM 추론 서버)

- **엔드포인트:** `http://localhost:8000`
- **API:** OpenAI Chat Completions 호환
- **모델:** Gemma 4 31B IT (Q4_K_M GGUF)
- **모드:** CPU 추론 (32 스레드)

API 테스트:

```bash
# 모델 목록 확인
curl http://localhost:8000/v1/models | jq

# 채팅 요청
curl http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gemma-4-31b-it",
    "messages": [{"role": "user", "content": "안녕하세요"}],
    "max_tokens": 128
  }' | jq
```

### 6-2. OpenCode (AI 코딩 어시스턴트)

- **웹 UI:** `http://localhost:3000`
- **LLM 연동:** vLLM 서버 자동 연결 (`http://vllm:8000/v1`)
- **작업 디렉토리:** `./workspace` 마운트

> **참고:** OpenCode는 현재 아카이브 상태이며, 후속 프로젝트는 [Crush](https://github.com/charmbracelet/crush)입니다.
> Docker 이미지 `smanx/opencode`는 100K+ 다운로드로 안정적입니다.

### 6-3. PaperClip (AI 오케스트레이션)

- **웹 UI:** `http://localhost:3100`
- **데이터베이스:** PostgreSQL 17 (자동 구성)
- **인증:** BETTER_AUTH_SECRET 환경 변수로 설정

> **참고:** `.env`의 `BETTER_AUTH_SECRET` 값을 프로덕션 환경에서는 반드시 변경하세요.

## 7) 양자화 및 KV 캐시 설정

### TurboQuant 상태

TurboQuant는 현재 vLLM PR 단계입니다 (PR #38662, #38479, #38280). 아직 메인 브랜치에
머지되지 않아 사용할 수 없습니다.

### FP8 KV 캐시

FP8 KV 캐시(`--kv-cache-dtype fp8`)는 NVIDIA Ada Lovelace/Hopper GPU에서만 지원됩니다.
AMD GPU 및 CPU 모드에서는 사용할 수 없습니다.

### 현재 적용된 최적화

| 항목 | 설정 | 비고 |
|------|------|------|
| 모델 양자화 | Q4_K_M (GGUF) | 31B 모델을 ~18GB로 압축 |
| KV 캐시 | `--kv-cache-dtype auto` | 자동 최적 선택 |
| KV 캐시 공간 | `VLLM_CPU_KVCACHE_SPACE=40` (GB) | CPU 모드 KV 캐시 크기 |
| 컨텍스트 길이 | `--max-model-len 8192` | 메모리에 맞게 조정 가능 |
| CPU 스레드 | `OMP_NUM_THREADS=32` | 16C/32T 풀 활용 |

> **GGUF Q4_K_M 선택 이유:** 31B 모델을 64GB RAM 시스템에서 실행 가능하게 하면서
> 품질 손실을 최소화하는 최적의 균형점입니다. Q5_K_M(~21GB)도 가능하지만
> KV 캐시 공간이 줄어듭니다.

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
# 대용량 페이지 활성화 (32B 모델 로딩 최적화)
echo 'vm.nr_hugepages=1024' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p

# swap 비활성화 (성능 향상)
sudo swapoff -a
```

### vLLM 네이티브 실행 (Docker 없이, ROCm GPU 가속)

Docker 대신 호스트에서 직접 실행하면 ROCm GPU 가속을 활용할 수 있습니다:

```bash
export HSA_OVERRIDE_GFX_VERSION=11.5.0
export VLLM_USE_TRITON_FLASH_ATTN=0
export OMP_NUM_THREADS=32

numactl --cpunodebind=0 --membind=0 \
python3 -m vllm.entrypoints.openai.api_server \
  --model ./models/gemma-4-31B-it-Q4_K_M.gguf \
  --served-model-name gemma-4-31b-it \
  --dtype auto \
  --gpu-memory-utilization 0.92 \
  --max-model-len 8192
```

## 9) ROCm GPU 가속 (선택)

현재 `vllm/vllm-openai` Docker Hub에 ROCm 태그가 없습니다.
ROCm GPU 가속을 Docker에서 사용하려면 아래 두 가지 방법이 있습니다:

### 방법 A: vLLM ROCm 소스 빌드

```bash
# vLLM 소스 클론
git clone --depth 1 --branch v0.19.0 \
    https://github.com/vllm-project/vllm.git /tmp/vllm-src

# ROCm Dockerfile로 빌드 (1~2시간 소요)
cd /tmp/vllm-src
DOCKER_BUILDKIT=1 docker build \
    --target vllm-openai \
    --tag vllm-rocm:v0.19.0 \
    --file docker/Dockerfile.rocm .
```

빌드 완료 후 `docker-compose.yml`에서 vllm 서비스의 `build` 섹션을 변경:

```yaml
vllm:
  image: vllm-rocm:v0.19.0
  # build: 섹션 제거
  devices:
    - /dev/kfd
    - /dev/dri
  group_add:
    - video
    - render
  command: >
    --host 0.0.0.0
    --port 8000
    --model /models/gemma-4-31B-it-Q4_K_M.gguf
    --served-model-name gemma-4-31b-it
    --max-model-len 8192
    --kv-cache-dtype auto
    --dtype auto
    --gpu-memory-utilization 0.92
```

### 방법 B: Dockerfile.vllm-rocm 사용

```bash
# docker-compose.yml의 vllm 서비스 dockerfile을 변경
# dockerfile: Dockerfile.vllm → dockerfile: Dockerfile.vllm-rocm
# + devices 추가
docker compose up -d --build vllm
```

> **참고:** Radeon 8060S (gfx1150, RDNA 4)의 ROCm 지원은 ROCm 버전에 따라 실험적입니다.
> `HSA_OVERRIDE_GFX_VERSION=11.5.0` 환경변수가 필요할 수 있습니다.

## 10) 트러블슈팅

### vLLM 시작이 느림

CPU 모드에서 31B 모델 로딩에 2~5분 소요될 수 있습니다.
`start_period: 120s` 헬스체크 설정으로 충분한 시간을 부여하고 있습니다.

```bash
# 로그 확인
docker compose logs -f vllm
```

### 메모리 부족 (OOM)

`.env`에서 설정 조정:

```bash
# KV 캐시 크기 줄이기
VLLM_CPU_KVCACHE_SPACE=20

# 컨텍스트 길이 줄이기
MAX_MODEL_LEN=4096

# 컨테이너 메모리 제한 줄이기
VLLM_MEMORY_LIMIT=40g
```

### OpenCode가 vLLM에 연결 실패

vLLM의 헬스체크가 통과할 때까지 기다립니다:

```bash
# vLLM 상태 확인
curl http://localhost:8000/health

# vLLM이 정상이면 OpenCode 재시작
docker compose restart opencode
```

### PaperClip 인증 오류

`.env`의 `BETTER_AUTH_SECRET`를 안전한 값으로 변경:

```bash
# 랜덤 시크릿 생성
openssl rand -hex 32
```

---

## 파일 구조

```
/home/aa/vllm/
├── docker-compose.yml          # Docker Compose 정의 (전체 스택)
├── Dockerfile.vllm             # vLLM CPU 모드 Docker 이미지
├── Dockerfile.vllm-rocm        # vLLM ROCm GPU 모드 Docker 이미지 (선택)
├── .env                        # 환경 변수 설정
├── download_model.sh           # GGUF 모델 다운로드 스크립트
├── install_docker.sh           # Docker Engine 설치 스크립트
├── setup_evo_x2_hardware.sh    # 하드웨어 최적화 스크립트
├── models/                     # GGUF 모델 파일 저장 디렉토리
│   └── gemma-4-31B-it-Q4_K_M.gguf
├── workspace/                  # OpenCode 작업 디렉토리
└── README.md                   # 이 문서
```

## 버전 정보

| 구성요소 | 버전 | 비고 |
|----------|------|------|
| vLLM | 0.19.0 (latest) | CPU 모드, GGUF 지원 |
| Gemma 4 31B IT | Q4_K_M GGUF | ~18GB, unsloth 제공 |
| OpenCode | latest | `smanx/opencode` Docker 이미지 |
| PaperClip | latest | `ghcr.io/paperclipai/paperclip` |
| PostgreSQL | 17-alpine | PaperClip 데이터베이스 |
| Docker Compose | v2 | `docker compose` 명령 사용 |
