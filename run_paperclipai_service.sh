#!/bin/bash
set -euo pipefail

PAPERCLIP_PID=""
SOCAT_PID=""

cleanup() {
    if [[ -n "${SOCAT_PID}" ]] && kill -0 "${SOCAT_PID}" 2>/dev/null; then
        kill "${SOCAT_PID}" 2>/dev/null || true
        wait "${SOCAT_PID}" 2>/dev/null || true
    fi
    if [[ -n "${PAPERCLIP_PID}" ]] && kill -0 "${PAPERCLIP_PID}" 2>/dev/null; then
        kill "${PAPERCLIP_PID}" 2>/dev/null || true
        wait "${PAPERCLIP_PID}" 2>/dev/null || true
    fi
}

trap cleanup TERM INT EXIT

/home/aa/.npm-global/bin/paperclipai run --data-dir ./paperclip-data &
PAPERCLIP_PID=$!

for _ in $(seq 1 60); do
    if ! kill -0 "${PAPERCLIP_PID}" 2>/dev/null; then
        wait "${PAPERCLIP_PID}"
        exit 1
    fi
    if ss -ltnH '( sport = :3100 )' | grep -q ':3100'; then
        break
    fi
    sleep 1
done

if ! ss -ltnH '( sport = :3100 )' | grep -q ':3100'; then
    echo "PaperclipAI did not open port 3100 in time"
    exit 1
fi

/usr/bin/socat TCP-LISTEN:3101,bind=0.0.0.0,reuseaddr,fork TCP:127.0.0.1:3100 &
SOCAT_PID=$!

set +e
wait -n "${PAPERCLIP_PID}" "${SOCAT_PID}"
EXIT_CODE=$?
set -e

cleanup
trap - TERM INT EXIT
exit "${EXIT_CODE}"
