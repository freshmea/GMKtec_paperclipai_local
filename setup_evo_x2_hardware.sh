#!/usr/bin/env bash
set -euo pipefail

if [[ "${EUID}" -ne 0 ]]; then
  echo "[ERROR] root 권한이 필요합니다. sudo로 실행하세요."
  exit 1
fi

echo "[1/7] 패키지 인덱스 갱신"
apt-get update

echo "[2/7] 기본 하드웨어 진단/가속 패키지 설치"
apt-get install -y \
  pciutils \
  mesa-vulkan-drivers \
  vulkan-tools \
  clinfo \
  rocminfo \
  rocm-smi \
  libhsa-runtime64-1 \
  libamdhip64-5 \
  numactl \
  hwloc \
  linux-tools-common \
  linux-tools-$(uname -r) \
  cpufrequtils \
  jq \
  git \
  python3-venv \
  python3-pip

echo "[3/7] AMD GPU 성능 관련 커널 파라미터 적용"
cat >/etc/modprobe.d/amdgpu-performance.conf <<'EOF'
options amdgpu ppfeaturemask=0xffffffff
EOF

echo "[4/7] 부팅 커널 파라미터 최적화"
GRUB_FILE="/etc/default/grub"
# deprecated amdgpu.gttsize 제거
sed -i 's/ amdgpu.gttsize=[0-9]*//' "${GRUB_FILE}"
# ttm.pages_limit (108GB GTT) + amd_pstate + mitigations=off 추가
if ! grep -q "ttm.pages_limit" "${GRUB_FILE}"; then
  sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="\([^"]*\)"/GRUB_CMDLINE_LINUX_DEFAULT="\1 ttm.pages_limit=27648000 ttm.page_pool_size=27648000 amd_pstate=active mitigations=off"/' "${GRUB_FILE}"
fi
update-grub

echo "[5/7] CPU governor를 performance로 설정"
if systemctl list-unit-files | grep -q cpupower.service; then
  systemctl enable --now cpupower.service || true
fi
for gov in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
  [[ -f "${gov}" ]] && echo performance >"${gov}" || true
done

echo "[6/7] 사용자 권한: render/video 그룹 추가(현재 실행 사용자 대상)"
if [[ -n "${SUDO_USER:-}" ]]; then
  usermod -aG render,video "${SUDO_USER}" || true
fi

echo "[7/7] 상태 출력"
echo "===== GPU ====="
lspci -nnk | grep -Ei 'vga|3d|display|amdgpu' -A4 -B1 || true
echo "===== ROCm ====="
rocminfo | sed -n '1,80p' || true
echo "===== OpenCL ====="
clinfo | sed -n '1,120p' || true

cat <<'EOF'

[완료] 재부팅 후 아래를 확인하세요.
  1) groups <user> 에 render,video 포함 여부
  2) rocminfo 에 Agent 정보 노출 여부
  3) vLLM ROCm 실행 테스트

EOF