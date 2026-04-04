#!/bin/bash
# =============================================================================
# Docker Engine 설치 스크립트 (Ubuntu 24.04)
# GMKtec EVO-X2 / AMD Ryzen AI MAX+ 395
# =============================================================================
set -e

echo "============================================"
echo "  Docker Engine 설치 시작"
echo "============================================"

# 기존 Docker 패키지 제거 (충돌 방지)
echo "[1/6] 기존 Docker 패키지 정리..."
for pkg in docker.io docker-doc docker-compose docker-compose-v2 podman-docker containerd runc; do
    sudo apt-get remove -y "$pkg" 2>/dev/null || true
done

# Docker 공식 GPG 키 추가
echo "[2/6] Docker GPG 키 추가..."
sudo apt-get update
sudo apt-get install -y ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

# Docker 저장소 추가
echo "[3/6] Docker APT 저장소 추가..."
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update

# Docker 설치
echo "[4/6] Docker Engine 설치..."
sudo apt-get install -y \
    docker-ce \
    docker-ce-cli \
    containerd.io \
    docker-buildx-plugin \
    docker-compose-plugin

# 사용자를 docker 그룹에 추가
echo "[5/6] 사용자 '$USER'를 docker 그룹에 추가..."
sudo usermod -aG docker "$USER"

# Docker 서비스 시작 및 활성화
echo "[6/6] Docker 서비스 활성화..."
sudo systemctl enable docker
sudo systemctl start docker

echo ""
echo "============================================"
echo "  Docker 설치 완료!"
echo "============================================"
echo ""
echo "Docker 버전:"
docker --version
echo ""
echo "Docker Compose 버전:"
docker compose version
echo ""
echo "※ docker 그룹 반영을 위해 로그아웃 후 다시 로그인하거나"
echo "  'newgrp docker' 명령을 실행하세요."
echo ""
