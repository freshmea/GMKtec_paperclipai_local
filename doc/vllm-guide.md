# LLM 추론 서버 실행 가이드

## 개요

이 프로젝트는 Docker Compose를 사용하여 **llama-server** (OpenAI 호환 LLM 추론 서버)를 실행합니다.
기본 모델은 `gemma-4-31B-it-Q4_K_M.gguf`이며, **ROCm GPU 가속** 모드로 동작합니다.

- **하드웨어:** GMKtec EVO-X2 / AMD Ryzen AI MAX+ 395 / 128GB LPDDR5
- **GPU:** Radeon 8060S (gfx1151, RDNA 4) — iGPU, VRAM은 시스템 메모리에서 할당
- **추론 엔진:** llama.cpp + ROCm (HIP) — [kyuz0/amd-strix-halo-toolboxes](https://github.com/kyuz0/amd-strix-halo-toolboxes) 기반
- **API:** OpenAI 호환 (`/v1/chat/completions`, `/v1/models`)

> **GPU 추론 백엔드 비교 (2026-04-05 기준):**
>
> | 백엔드 | 상태 | 비고 |
> |--------|------|------|
> | **llama.cpp + ROCm 7.2** | **동작 확인 (권장)** | gfx1151 네이티브 지원, GPU 가속 |
> | llama.cpp + Vulkan | 동작 확인 | 대안 (ROCm 불가 시) |
> | vLLM + ROCm | **미지원** | PR #38455 진행중, segfault/hang 이슈 미해결 |
> | vLLM CPU | 동작 확인 | 매우 느림 (비권장) |
>
> **⚠️ 커널 주의:** 커널 6.17.x에서는 gfx1151 안정성 버그가 있습니다.
> 안정적 운영을 위해 **커널 6.18.4 이상**을 권장합니다 (kyuz0 권고).
>
> **메모리 구성:** 128GB LPDDR5는 GPU가 GTT(Graphics Translation Table)를 통해 동적으로 사용합니다.
> 커널 파라미터 `ttm.pages_limit`으로 GPU 접근 가능 메모리 한도를 설정합니다 (최대 108GB 권장).
> BIOS UMA 설정 변경은 불필요합니다 — 자세한 내용은 [7. iGPU VRAM 할당 늘리기](#7-igpu-vram-할당-늘리기-gtt-메모리-설정) 참조.

---

## 1. 사전 준비

### Docker 설치

```bash
./install_docker.sh
```

설치 후 재로그인하여 docker 그룹 적용:

```bash
newgrp docker
```

### 하드웨어 최적화 (선택)

```bash
sudo ./setup_evo_x2_hardware.sh
sudo reboot
```

CPU governor를 `performance`로 설정하고, AMD GPU 커널 파라미터를 최적화합니다.

---

## 2. 모델 다운로드

```bash
./download_model.sh
```

Hugging Face에서 `gemma-4-31B-it-Q4_K_M.gguf` (~18GB)를 `models/` 디렉토리로 다운로드합니다.

---

## 3. 서비스 실행

### 사전 빌드 이미지 다운로드 (최초 1회)

```bash
docker pull kyuz0/amd-strix-halo-toolboxes:rocm-7.2
```

### 전체 스택 시작 (LLM + OpenCode + PaperClip)

```bash
docker compose up -d
```

### LLM 서버만 실행

```bash
docker compose up -d llm
```

### 로그 확인

```bash
docker compose logs -f llm
```

### 이미지 재빌드가 필요한 경우 (커스텀 빌드 사용 시)

`Dockerfile.llama-rocm`을 사용하여 직접 빌드하려면:

```bash
# docker-compose.yml에서 image: 줄을 주석처리하고 build: 블록 주석 해제 후:
docker compose up -d --build
```

### 서비스 중지

```bash
docker compose down
```

---

## 4. API 사용

vLLM은 OpenAI 호환 API를 제공합니다. 기본 포트는 `8000`입니다.

### 헬스 체크

```bash
curl http://localhost:8000/health
```

### Chat Completion 요청

```bash
curl http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gemma-4-31b-it",
    "messages": [{"role": "user", "content": "안녕하세요"}],
    "max_tokens": 256
  }'
```

### 모델 목록 확인

```bash
curl http://localhost:8000/v1/models
```

---

## 5. 주요 설정 (환경변수)

| 환경변수 | 기본값 | 설명 |
|----------|--------|------|
| `LLM_PORT` | `8000` | LLM API 포트 |
| `MODEL_DIR` | `./models` | 모델 파일 디렉토리 |
| `MODEL_FILENAME` | `gemma-4-31B-it-Q4_K_M.gguf` | 모델 파일명 |
| `SERVED_MODEL_NAME` | `gemma-4-31b-it` | API에서 사용할 모델 이름 (alias) |
| `CONTEXT_LENGTH` | `8192` | 최대 컨텍스트 길이 |
| `LLM_MEMORY_LIMIT` | `96g` | 컨테이너 메모리 제한 |
| `GPU_MAX_HW_QUEUES` | `2` | GPU 하드웨어 큐 수 (안정성용) |

환경변수는 `.env` 파일을 생성하여 설정할 수 있습니다:

```bash
# .env
LLM_PORT=8000
CONTEXT_LENGTH=4096
SERVED_MODEL_NAME=gemma-4-31b-it
```

---

## 6. 서비스 포트 요약

| 서비스 | 포트 | 설명 |
|--------|------|------|
| LLM (llama-server) | `8000` | OpenAI 호환 API 서버 (ROCm GPU) |
| OpenCode | `3000` | AI 코딩 어시스턴트 웹 UI |
| PaperClip | `3100` | AI 오케스트레이션 플랫폼 |

---

## 7. iGPU VRAM 할당 늘리기 (GTT 메모리 설정)

> 참고: [Jeff Geerling — Increasing the VRAM allocation on AMD AI APUs under Linux](https://www.jeffgeerling.com/blog/2025/increasing-vram-allocation-on-amd-ai-apus-under-linux)

AMD AI APU (Ryzen AI MAX+ 395 등)는 iGPU이므로 VRAM이 시스템 메모리에서 동적으로 할당됩니다.
기본 설정으로는 GPU에 할당 가능한 GTT(Graphics Translation Table) 메모리가 제한되어 있어,
LLM처럼 큰 모델을 GPU에 올리려면 이 값을 늘려야 합니다.

### 7-1. 기존 방식 (deprecated)

현재 시스템에는 아래 방식이 적용되어 있습니다:

```
amdgpu.gttsize=131072
```

이 방식은 커널 로그에 다음과 같은 경고가 출력됩니다:

```
amdgpu: [drm] Configuring gttsize via module parameter is deprecated, please use ttm.pages_limit
```

### 7-2. 새로운 방식 (ttm 커널 파라미터)

`ttm.pages_limit`과 `ttm.page_pool_size` 파라미터를 사용합니다.

> **커널 버전별 파라미터 이름:**
> - 커널 6.17+ (Ubuntu 24.04 등): `ttm.pages_limit`, `ttm.page_pool_size`
> - 일부 ROCm/AMDGPU 전용 커널: `amdttm.pages_limit`, `amdttm.page_pool_size`
>
> 시스템에서 `ls /sys/module/ttm/parameters/` 명령으로 확인할 수 있습니다.

**계산 공식:**

```
pages = (원하는 GB × 1024 × 1024) / 4.096
```

| 원하는 GTT | pages 값 |
|------------|----------|
| 96 GB | 24576000 |
| 108 GB (권장 최대) | 27648000 |

> **주의:** AI MAX+ 395에서 128GB RAM 기준 약 108GB가 안정적 최대치입니다.
> 110GB 이상은 모델 로딩 시 segfault가 발생할 수 있습니다.

**Ubuntu (GRUB) 설정:**

```bash
# 1. 기존 deprecated 파라미터 제거
sudo sed -i 's/ amdgpu.gttsize=[0-9]*//' /etc/default/grub

# 2. 새 파라미터 추가 (108GB 예시)
sudo sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="\([^"]*\)"/GRUB_CMDLINE_LINUX_DEFAULT="\1 ttm.pages_limit=27648000 ttm.page_pool_size=27648000"/' /etc/default/grub

# 3. GRUB 업데이트 및 재부팅
sudo update-grub
sudo reboot
```

**Fedora (grubby) 설정:**

```bash
sudo grubby --update-kernel=ALL --args='ttm.pages_limit=27648000'
sudo grubby --update-kernel=ALL --args='ttm.page_pool_size=27648000'
sudo reboot
```

### 7-3. 적용 확인

재부팅 후 다음 명령으로 GTT 메모리 할당을 확인합니다:

```bash
sudo dmesg | grep "amdgpu.*memory"
```

기대 출력 (108GB 설정 시):

```
[drm] amdgpu: 512M of VRAM memory ready
[drm] amdgpu: 108000M of GTT memory ready.
```

- **VRAM**: 고정 할당된 전용 비디오 메모리 (512MB, BIOS UMA 설정)
- **GTT**: 시스템 메모리에서 GPU가 접근 가능한 추가 메모리 (동적 할당)

> **참고:** GPU 드라이버는 GTT 메모리를 필요에 따라 동적으로 할당합니다.
> `nvtop`, `btop` 등 모니터링 도구가 512MB만 표시할 수 있지만,
> 실제로는 설정된 GTT 한도까지 사용 가능합니다.
> `vulkaninfo`로 확인하면 더 정확한 값을 볼 수 있습니다.

### 7-4. BIOS UMA 설정에 대하여

BIOS의 UMA (Unified Memory Architecture) 설정은 iGPU에 **고정 할당**되는 전용 VRAM 크기를 결정합니다.
GMKtec EVO-X2 기본 BIOS에서는 보통 Auto 또는 특정 값(예: 512MB~16GB)으로 설정되어 있습니다.

**GTT 방식과의 관계:**

| 구분 | BIOS UMA (VRAM) | GTT 메모리 |
|------|----------------|------------|
| 설정 위치 | BIOS 설정 화면 | 커널 부트 파라미터 |
| 할당 방식 | **고정** (부팅 시 예약) | **동적** (필요 시 할당) |
| dmesg 표시 | `VRAM memory ready` | `GTT memory ready` |
| 용도 | 디스플레이 출력 등 기본 GPU 작업 | **LLM 추론용 대용량 메모리** |

**결론: BIOS UMA 설정을 변경할 필요 없습니다.**

LLM 추론에 필요한 대용량 GPU 메모리는 GTT로 동적 할당되므로,
BIOS UMA는 기본값(Auto 또는 512MB)을 유지하면 됩니다.
위의 `ttm.pages_limit` 커널 파라미터로 GTT 한도를 108GB로 설정하면
GPU가 필요에 따라 시스템 메모리에서 최대 108GB까지 사용할 수 있습니다.

> **참고:** BIOS에서 UMA를 96GB 등으로 크게 설정하면 해당 메모리가 고정 예약되어
> 시스템 RAM이 줄어들고, OS 및 다른 프로세스에 영향을 줄 수 있습니다.
> GTT 동적 할당이 훨씬 유연하므로 BIOS UMA는 최소로 두는 것을 권장합니다.

### 7-5. 참고 자료

- [Jeff Geerling — Increasing the VRAM allocation on AMD AI APUs under Linux](https://www.jeffgeerling.com/blog/2025/increasing-vram-allocation-on-amd-ai-apus-under-linux)
- [AMD Instinct MI300A system optimization](https://instinct.docs.amd.com/projects/amdgpu-docs/en/latest/system-optimization/mi300a.html)
- [Preparing AMD APUs for LLM usage](https://blog.linux-ng.de/2025/07/13/getting-information-about-amd-apus/)
- [Framework Community Forum — iGPU VRAM allocation](https://community.frame.work/t/igpu-vram-how-much-can-be-assigned/73081/7)

---

## 8. 문제 해결

### `opencode`가 unhealthy 라고 나오지만 실제 원인이 `llm`인 경우

`opencode`는 `llm`이 healthy 상태가 된 뒤에 시작됩니다. 따라서 아래처럼 보이면:

```text
ERROR: for opencode Container "..." is unhealthy
```

실제로는 `llm`이 먼저 실패했을 가능성이 큽니다. 다음 순서로 확인하세요:

```bash
docker compose ps
docker compose logs --tail 200 llm
```

만약 `HIP error` 또는 `no device found` 오류가 보이면 GPU 장치 접근 문제입니다:

```bash
# GPU 장치 확인
ls -la /dev/kfd /dev/dri/render*

# 사용자가 video/render 그룹에 포함되어 있는지 확인
groups

# rocminfo 테스트 (컨테이너 내부)
docker run --rm --device=/dev/kfd --device=/dev/dri \
  --group-add video --security-opt seccomp=unconfined \
  kyuz0/amd-strix-halo-toolboxes:rocm-7.2 rocminfo

# render 그룹 오류 시 group_add에서 render 제거
# docker-compose.yml의 group_add 섹션에서 "- render" 줄 삭제
```
