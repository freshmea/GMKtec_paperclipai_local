#!/bin/bash
# =============================================================================
# GGUF 모델 다운로드 스크립트
# 기본 운영 모델: Gemma 4 26B-A4B + Qwen3.6 35B-A3B
# 선택 레거시 모델: Gemma 4 31B Dense, Gemma 4 E4B
# =============================================================================
set -e

MODEL_DIR="${MODEL_DIR:-./models}"

# 모델 선택 메뉴
echo "============================================"
echo "  GGUF 모델 다운로드"
echo "============================================"
echo ""
echo "  다운로드할 모델을 선택하세요:"
echo ""
echo "  1) Gemma 4 26B-A4B IT (MoE, ~16GB)       — shorter/mid-context"
echo "  2) Qwen3.6 35B-A3B Q4_K_M (~22.1GB)      — 긴 컨텍스트/기술 작업 ← 추천"
echo "  3) Gemma 4 26B + Qwen3.6 (hybrid)        — PaperClip 혼합 운영 ← 기본"
echo "  4) Gemma 4 31B IT (Dense, ~18GB)         — legacy 옵션"
echo "  5) Gemma 4 E4B IT (~5GB)                 — legacy 고속"
echo "  6) 전체 다운로드"
echo ""

if [ -n "$1" ]; then
    CHOICE="$1"
else
    read -p "선택 (1-6) [3]: " CHOICE
    CHOICE="${CHOICE:-3}"
fi

# 모델 정보 설정
download_model() {
    local repo_id="$1"
    local filename="$2"
    local desc="$3"

    echo ""
    echo "--------------------------------------------"
    echo "  ${desc}"
    echo "  저장소:  ${repo_id}"
    echo "  파일명:  ${filename}"
    echo "  저장위치: ${MODEL_DIR}/"
    echo "--------------------------------------------"

    # 이미 다운로드된 파일 확인
    if [ -f "${MODEL_DIR}/${filename}" ]; then
        echo "[INFO] 모델 파일이 이미 존재합니다: ${MODEL_DIR}/${filename}"
        ls -lh "${MODEL_DIR}/${filename}"
        echo ""
        read -p "다시 다운로드하시겠습니까? (y/N): " REDOWNLOAD
        if [ "$REDOWNLOAD" != "y" ] && [ "$REDOWNLOAD" != "Y" ]; then
            echo "다운로드를 건너뜁니다."
            return 0
        fi
    fi

    echo ""
    echo "[INFO] 다운로드 시작..."
    echo ""
    "${HF_CMD}" download "${repo_id}" "${filename}" --local-dir "${MODEL_DIR}"
    echo ""
    echo "[OK] 다운로드 완료: ${MODEL_DIR}/${filename}"
    ls -lh "${MODEL_DIR}/${filename}"
}

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

# 모델 디렉토리 생성
mkdir -p "${MODEL_DIR}"

case "${CHOICE}" in
    1)
        download_model "bartowski/google_gemma-4-26B-A4B-it-GGUF" \
            "google_gemma-4-26B-A4B-it-Q4_K_M.gguf" \
            "Gemma 4 26B-A4B IT (MoE, Q4_K_M, ~16GB)"
        ;;
    2)
        download_model "unsloth/Qwen3.6-35B-A3B-GGUF" \
            "Qwen3.6-35B-A3B-UD-Q4_K_M.gguf" \
            "Qwen3.6 35B-A3B (UD-Q4_K_M, ~22.1GB)"
        ;;
    3)
        download_model "bartowski/google_gemma-4-26B-A4B-it-GGUF" \
            "google_gemma-4-26B-A4B-it-Q4_K_M.gguf" \
            "Gemma 4 26B-A4B IT (MoE, Q4_K_M, ~16GB)"
        download_model "unsloth/Qwen3.6-35B-A3B-GGUF" \
            "Qwen3.6-35B-A3B-UD-Q4_K_M.gguf" \
            "Qwen3.6 35B-A3B (UD-Q4_K_M, ~22.1GB)"
        ;;
    4)
        download_model "unsloth/gemma-4-31B-it-GGUF" \
            "gemma-4-31B-it-Q4_K_M.gguf" \
            "Gemma 4 31B IT (Dense, Q4_K_M, ~18GB, legacy optional)"
        ;;
    5)
        download_model "unsloth/gemma-4-E4B-it-GGUF" \
            "gemma-4-E4B-it-Q4_K_M.gguf" \
            "Gemma 4 E4B IT (Q4_K_M, ~5GB, legacy)"
        ;;
    6)
        download_model "bartowski/google_gemma-4-26B-A4B-it-GGUF" \
            "google_gemma-4-26B-A4B-it-Q4_K_M.gguf" \
            "Gemma 4 26B-A4B IT (MoE, Q4_K_M, ~16GB)"
        download_model "unsloth/gemma-4-31B-it-GGUF" \
            "gemma-4-31B-it-Q4_K_M.gguf" \
            "Gemma 4 31B IT (Dense, Q4_K_M, ~18GB, legacy optional)"
        download_model "unsloth/Qwen3.6-35B-A3B-GGUF" \
            "Qwen3.6-35B-A3B-UD-Q4_K_M.gguf" \
            "Qwen3.6 35B-A3B (UD-Q4_K_M, ~22.1GB)"
        download_model "unsloth/gemma-4-E4B-it-GGUF" \
            "gemma-4-E4B-it-Q4_K_M.gguf" \
            "Gemma 4 E4B IT (Q4_K_M, ~5GB, legacy)"
        ;;
    *)
        echo "[ERROR] 잘못된 선택: ${CHOICE} (1~6)"
        exit 1
        ;;
esac

echo ""
echo "============================================"
echo "  다운로드 완료!"
echo "============================================"
echo ""
ls -lh "${MODEL_DIR}/"*.gguf 2>/dev/null || true
echo ""
echo "모델을 사용하려면:"
echo "  docker compose up -d"
echo ""
