#!/bin/bash
set -e

SERVICE_NAME="paperclipai"
SERVICE_FILE="$(dirname "$(realpath "$0")")/${SERVICE_NAME}.service"
RUNNER_FILE="$(dirname "$(realpath "$0")")/run_paperclipai_service.sh"

if [[ ! -f "$SERVICE_FILE" ]]; then
    echo "ERROR: ${SERVICE_FILE} 파일을 찾을 수 없습니다."
    exit 1
fi

if [[ ! -f "$RUNNER_FILE" ]]; then
    echo "ERROR: ${RUNNER_FILE} 파일을 찾을 수 없습니다."
    exit 1
fi

if [[ ! -x /home/aa/.local/bin/opencode ]]; then
    echo "ERROR: /home/aa/.local/bin/opencode 실행 파일을 찾을 수 없습니다."
    echo "먼저 호스트 OpenCode CLI를 설치하세요."
    exit 1
fi

echo "=== PaperclipAI systemd 서비스 설치 ==="

# 서비스 파일 복사
sudo cp "$SERVICE_FILE" /etc/systemd/system/${SERVICE_NAME}.service
sudo install -m 755 "$RUNNER_FILE" /home/aa/vllm/run_paperclipai_service.sh
# 비대화형/비로그인 PATH에서도 opencode를 찾을 수 있도록 전역 경로에 링크
sudo ln -sf /home/aa/.local/bin/opencode /usr/local/bin/opencode
echo "[1/4] 서비스 파일 복사 완료"

# 이전 외부 프록시 분리 서비스 정리
if sudo test -f /etc/systemd/system/paperclipai-proxy.service; then
    sudo systemctl disable --now paperclipai-proxy.service || true
    sudo rm -f /etc/systemd/system/paperclipai-proxy.service
fi

# sleep/hibernate 복귀 시 자동 재시작 hook
sudo mkdir -p /etc/systemd/system/${SERVICE_NAME}.service.d
sudo tee /etc/systemd/system/${SERVICE_NAME}.service.d/restart-on-wake.conf > /dev/null <<'EOF'
# sleep/hibernate 복귀 후 서비스 재시작 보장
[Unit]
After=suspend.target hibernate.target hybrid-sleep.target suspend-then-hibernate.target
EOF

sudo tee /etc/systemd/system/paperclipai-wake.service > /dev/null <<EOF
[Unit]
Description=Restart PaperclipAI after sleep/hibernate
After=suspend.target hibernate.target hybrid-sleep.target suspend-then-hibernate.target

[Service]
Type=oneshot
ExecStart=/bin/systemctl restart ${SERVICE_NAME}.service

[Install]
WantedBy=suspend.target hibernate.target hybrid-sleep.target suspend-then-hibernate.target
EOF
echo "[2/4] sleep/hibernate 복귀 hook 설치 완료"

# systemd 데몬 리로드 및 서비스 활성화
sudo systemctl daemon-reload
sudo systemctl enable ${SERVICE_NAME}.service
sudo systemctl enable paperclipai-wake.service
echo "[3/4] 서비스 활성화 완료"

# 서비스 시작
sudo systemctl restart ${SERVICE_NAME}.service
echo "[4/4] 서비스 시작 완료"

echo ""
echo "=== 설치 완료 ==="
echo "상태 확인:  sudo systemctl status ${SERVICE_NAME}"
echo "로그 확인:  sudo journalctl -u ${SERVICE_NAME} -f"
echo "중지:       sudo systemctl stop ${SERVICE_NAME}"
echo "제거:       sudo systemctl disable ${SERVICE_NAME} paperclipai-wake && sudo rm -f /etc/systemd/system/${SERVICE_NAME}.service /etc/systemd/system/paperclipai-wake.service && sudo rm -rf /etc/systemd/system/${SERVICE_NAME}.service.d"
