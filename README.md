# vLLM 하드웨어 세팅 기록 (GMKtec EVO-X2)

작성일: 2026-04-05

## 1) 현재 장비/OS 점검 결과

- OS: Ubuntu 24.04.4 LTS
- 커널: 6.17.0-20-generic
- CPU: AMD RYZEN AI MAX+ 395 w/ Radeon 8060S (16C/32T)
- 메모리: 총 62GiB (실메모리 64GB 구성)
- GPU 드라이버: `amdgpu` 커널 드라이버 로드 확인
- 현재 sudo 정책: 비밀번호 입력 필요 (`SUDO_PASSWORD_REQUIRED`)

## 2) 목표별 세팅 전략

### A. 최대 GPU 가속

- 커널 레벨: `amdgpu` 성능 플래그 및 `amdgpu.gttsize=131072` 적용
- 유저 공간: ROCm/OpenCL 도구 설치 (`rocminfo`, `rocm-smi`, `clinfo`)
- Vulkan 진단 설치 (`vulkan-tools`)

### B. 최대 CPU 활용

- `performance` governor 적용
- `cpufrequtils`, `linux-tools-$(uname -r)` 설치
- vLLM 실행 시 CPU 바인딩/스레드 제어 (`numactl`, OMP 변수)

### C. VRAM 128G 사용 이슈

- 본 장비는 iGPU(공유 메모리) 구조이며, 현재 시스템 메모리가 64GB로 확인됨
- 따라서 "VRAM 128G"를 실사용하려면 물리 RAM이 128GB여야 하며 BIOS UMA 설정도 충분히 크게 설정되어야 함
- 현재 구성(64GB)에서는 128GB VRAM 할당은 물리적으로 불가능

## 3) 자동 설치 스크립트

아래 스크립트를 생성해 두었음:

- `./setup_evo_x2_hardware.sh`

실행 방법:

```bash
cd /home/aa/vllm
sudo ./setup_evo_x2_hardware.sh
sudo reboot
```

## 4) 스크립트가 수행하는 작업

1. 패키지 설치
2. `/etc/modprobe.d/amdgpu-performance.conf` 생성
3. `/etc/default/grub`에 성능 커널 파라미터 추가
4. CPU governor를 `performance`로 설정
5. 사용자에 `render`, `video` 그룹 추가
6. `rocminfo`, `clinfo` 진단 출력

## 5) 재부팅 후 검증 명령

```bash
groups $USER
lsmod | grep -E 'amdgpu|kfd'
rocminfo | head -n 80
clinfo | head -n 120
vulkaninfo --summary
```

## 6) vLLM 권장 실행 예시 (ROCm 환경)

```bash
export HSA_OVERRIDE_GFX_VERSION=11.5.0
export VLLM_USE_TRITON_FLASH_ATTN=0
export OMP_NUM_THREADS=32

numactl --cpunodebind=0 --membind=0 \
python3 -m vllm.entrypoints.openai.api_server \
  --model <모델경로 또는 허브ID> \
  --dtype auto \
  --gpu-memory-utilization 0.92 \
  --max-model-len 8192
```

참고:

- `HSA_OVERRIDE_GFX_VERSION` 값은 ROCm 인식 상태에 따라 조정 필요
- 최신 ROCm이 필요하면 Ubuntu 기본 저장소보다 AMD 공식 저장소 버전이 유리할 수 있음

## 7) 이번 작업에서 실제 수행된 항목

- 하드웨어/드라이버/OS 상태 진단 완료
- 설치 스크립트 작성 완료
- sudo 비밀번호가 필요한 환경이라 시스템 패키지 설치는 실행 대기 상태
