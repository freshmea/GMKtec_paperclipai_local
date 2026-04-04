#!/bin/bash
# =============================================================================
# Gemma 4 31B IT GGUF 모델 다운로드 스크립트
# Q4_K_M 양자화 버전 (~18GB)
# =============================================================================
set -e

MODEL_DIR="${MODEL_DIR:-./models}"
REPO_ID="${REPO_ID:-unsloth/gemma-4-31B-it-GGUF}"
FILENAME="${FILENAME:-gemma-4-31B-it-Q4_K_M.gguf}"

echo "============================================"
echo "  Gemma 4 31B IT GGUF 모델 다운로드"
echo "============================================"
echo ""
echo "  저장소:  ${REPO_ID}"
echo "  파일명:  ${FILENAME}"
echo "  저장위치: ${MODEL_DIR}/"
echo ""

# 모델 디렉토리 생성
mkdir -p "${MODEL_DIR}"

# HF CLI 명령어 결정 (huggingface_hub >=1.0 은 'hf', 이전은 'huggingface-cli')
HF_CMD=""
if command -v hf &> /dev/null; then
    HF_CMD="hf"
elif command -v huggingface-cli &> /dev/null; then
    HF_CMD="huggingface-cli"
else
    # pipx 설치 확인
    if ! command -v pipx &> /dev/null; then
        echo "[INFO] pipx 설치 중..."
        sudo apt-get update && sudo apt-get install -y pipx
        pipx ensurepath
    fi
    export PATH="$HOME/.local/bin:$PATH"
    echo "[INFO] huggingface_hub 설치 중..."
    pipx install "huggingface_hub[cli]"
    # 설치 후 명령어 재확인
    if command -v hf &> /dev/null; then
        HF_CMD="hf"
    elif command -v huggingface-cli &> /dev/null; then
        HF_CMD="huggingface-cli"
    else
        echo "[ERROR] huggingface CLI를 찾을 수 없습니다."
        exit 1
    fi
fi

# 이미 다운로드된 파일 확인
if [ -f "${MODEL_DIR}/${FILENAME}" ]; then
    echo "[INFO] 모델 파일이 이미 존재합니다: ${MODEL_DIR}/${FILENAME}"
    ls -lh "${MODEL_DIR}/${FILENAME}"
    echo ""
    read -p "다시 다운로드하시겠습니까? (y/N): " REDOWNLOAD
    if [ "$REDOWNLOAD" != "y" ] && [ "$REDOWNLOAD" != "Y" ]; then
        echo "다운로드를 건너뜁니다."
        exit 0
    fi
fi

echo ""
echo "[INFO] 다운로드 시작... (약 18GB, 네트워크 속도에 따라 시간이 걸립니다)"
echo ""

# HuggingFace에서 GGUF 파일 다운로드
"${HF_CMD}" download \
    "${REPO_ID}" \
    "${FILENAME}" \
    --local-dir "${MODEL_DIR}"

echo ""
echo "============================================"
echo "  다운로드 완료!"
echo "============================================"
echo ""
ls -lh "${MODEL_DIR}/${FILENAME}"
echo ""
echo "모델을 사용하려면:"
echo "  docker compose up -d"
echo ""
