#!/bin/bash
set -euo pipefail

export PATH="/home/aa/.local/bin:/home/aa/.npm-global/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
export HOME="/home/aa"

PAPERCLIP_PID=""
SOCAT_PID=""

cleanup_listener_port() {
    local port="$1"
    local pids
    pids="$(
        ss -ltnp "( sport = :${port} )" 2>/dev/null \
        | grep -o 'pid=[0-9]\+' \
        | cut -d= -f2 \
        | sort -u || true
    )"

    if [[ -n "${pids}" ]]; then
        echo "Cleaning existing listener(s) on port ${port}: ${pids}"
        for pid in ${pids}; do
            kill "${pid}" 2>/dev/null || true
        done
        sleep 1
    fi
}

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

# Clean up stale/manual listeners before starting the managed service instance.
cleanup_listener_port 3101
cleanup_listener_port 3100

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
