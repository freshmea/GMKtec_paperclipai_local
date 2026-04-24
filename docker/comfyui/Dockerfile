ARG ROCM_PYTORCH_IMAGE=rocm/pytorch:rocm7.2_ubuntu24.04_py3.12_pytorch_release_2.8.0
FROM ${ROCM_PYTORCH_IMAGE}

ARG COMFYUI_REF=v0.19.3

ENV DEBIAN_FRONTEND=noninteractive \
    PYTHONUNBUFFERED=1 \
    PIP_NO_CACHE_DIR=1 \
    COMFYUI_HOME=/opt/comfyui

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    ffmpeg \
    git \
    libgl1 \
    libglib2.0-0 \
    && rm -rf /var/lib/apt/lists/*

RUN git clone --depth 1 --branch ${COMFYUI_REF} https://github.com/Comfy-Org/ComfyUI.git ${COMFYUI_HOME}

WORKDIR ${COMFYUI_HOME}

RUN python -m pip install --upgrade pip setuptools wheel

RUN python - <<'PY'
from pathlib import Path
req = Path("requirements.txt").read_text().splitlines()
filtered = [line for line in req if line.strip() not in {"torch", "torchvision", "torchaudio"}]
Path("requirements-rocm.txt").write_text("\n".join(filtered) + "\n")
print("Wrote requirements-rocm.txt")
PY

RUN python -m pip install -r requirements-rocm.txt
RUN python -m pip install -U --pre comfyui-manager

COPY start.sh /start.sh
RUN chmod +x /start.sh && \
    mkdir -p /opt/comfyui/models /opt/comfyui/input /opt/comfyui/output /opt/comfyui/user /opt/comfyui/custom_nodes

EXPOSE 8188

ENTRYPOINT ["/start.sh"]
