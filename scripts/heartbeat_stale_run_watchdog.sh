#!/usr/bin/env bash
set -euo pipefail

PAPERCLIP_DATA_DIR="${PAPERCLIP_DATA_DIR:-/home/aa/vllm/paperclip-data}"
PAPERCLIP_INSTANCE_ID="${PAPERCLIP_INSTANCE_ID:-default}"
RUN_LOG_ROOT="${PAPERCLIP_RUN_LOG_ROOT:-$PAPERCLIP_DATA_DIR/instances/$PAPERCLIP_INSTANCE_ID/data/run-logs}"

DB_HOST="${PAPERCLIP_DB_HOST:-127.0.0.1}"
DB_PORT="${PAPERCLIP_DB_PORT:-54329}"
DB_NAME="${PAPERCLIP_DB_NAME:-paperclip}"
DB_USER="${PAPERCLIP_DB_USER:-paperclip}"
DB_PASSWORD="${PAPERCLIP_DB_PASSWORD:-paperclip}"

POLL_SECONDS="${PAPERCLIP_STALE_RUN_WATCHDOG_POLL_SECONDS:-60}"
MIN_AGE_SECONDS="${PAPERCLIP_STALE_RUN_MIN_AGE_SECONDS:-300}"
MAX_IDLE_SECONDS="${PAPERCLIP_STALE_RUN_MAX_IDLE_SECONDS:-300}"
TERM_GRACE_SECONDS="${PAPERCLIP_STALE_RUN_TERM_GRACE_SECONDS:-15}"
LOCK_FILE="${PAPERCLIP_STALE_RUN_WATCHDOG_LOCK_FILE:-/tmp/paperclip-stale-run-watchdog.lock}"

MODE="loop"
DRY_RUN=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --once)
            MODE="once"
            ;;
        --dry-run)
            DRY_RUN=1
            ;;
        *)
            echo "[stale-run-watchdog] unknown argument: $1" >&2
            exit 1
            ;;
    esac
    shift
done

log() {
    printf '[stale-run-watchdog] %s %s\n' "$(date -Iseconds)" "$*"
}

exec 9>"$LOCK_FILE"
if ! flock -n 9; then
    log "another watchdog instance already holds $LOCK_FILE; exiting"
    exit 0
fi

psql_query() {
    PGPASSWORD="$DB_PASSWORD" \
        psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" \
        -v ON_ERROR_STOP=1 -At -F $'\t' -c "$1"
}

terminate_process_group() {
    local pid="$1"
    local pgid="$2"
    local target=""

    if [[ -n "$pgid" && "$pgid" != "0" ]]; then
        target="-$pgid"
    else
        target="$pid"
    fi

    if [[ "$DRY_RUN" -eq 1 ]]; then
        log "dry-run: would send SIGTERM to $target"
        return 0
    fi

    if [[ "$target" == -* ]]; then
        kill -TERM "$target" 2>/dev/null || true
    else
        kill -TERM "$target" 2>/dev/null || true
    fi

    local deadline=$(( $(date +%s) + TERM_GRACE_SECONDS ))
    while kill -0 "$pid" 2>/dev/null; do
        if (( $(date +%s) >= deadline )); then
            break
        fi
        sleep 1
    done

    if kill -0 "$pid" 2>/dev/null; then
        log "run process pid=$pid did not exit after ${TERM_GRACE_SECONDS}s; escalating to SIGKILL"
        if [[ "$target" == -* ]]; then
            kill -KILL "$target" 2>/dev/null || true
        else
            kill -KILL "$target" 2>/dev/null || true
        fi
    fi
}

check_once() {
    local now_epoch
    now_epoch="$(date +%s)"
    local query
    query="
        select
            hr.id,
            hr.agent_id,
            hr.process_pid,
            coalesce(hr.process_group_id, hr.process_pid),
            coalesce(hr.log_store, ''),
            coalesce(hr.log_ref, ''),
            extract(epoch from hr.started_at)::bigint
        from heartbeat_runs hr
        where hr.status = 'running'
          and hr.process_pid is not null
          and hr.started_at is not null
        order by hr.started_at asc;
    "

    local rows
    rows="$(psql_query "$query")"
    if [[ -z "$rows" ]]; then
        return 0
    fi

    while IFS=$'\t' read -r run_id agent_id pid pgid log_store log_ref started_epoch; do
        [[ -n "${run_id:-}" ]] || continue

        local run_age=$(( now_epoch - started_epoch ))
        if (( run_age < MIN_AGE_SECONDS )); then
            continue
        fi

        if ! kill -0 "$pid" 2>/dev/null; then
            log "run=$run_id agent=$agent_id pid=$pid is still marked running but process is gone; skipping kill"
            continue
        fi

        local last_activity_epoch="$started_epoch"
        if [[ "$log_store" == "local_file" && -n "$log_ref" ]]; then
            local log_path="$RUN_LOG_ROOT/$log_ref"
            if [[ -f "$log_path" ]]; then
                last_activity_epoch="$(stat -c %Y "$log_path")"
            fi
        fi

        local idle_seconds=$(( now_epoch - last_activity_epoch ))
        if (( idle_seconds < MAX_IDLE_SECONDS )); then
            continue
        fi

        log "stale run detected: run=$run_id agent=$agent_id pid=$pid pgid=$pgid age=${run_age}s idle=${idle_seconds}s"
        terminate_process_group "$pid" "$pgid"
    done <<< "$rows"
}

log "started with poll=${POLL_SECONDS}s min_age=${MIN_AGE_SECONDS}s max_idle=${MAX_IDLE_SECONDS}s dry_run=${DRY_RUN}"

if [[ "$MODE" == "once" ]]; then
    check_once
    exit 0
fi

while true; do
    check_once || log "iteration failed"
    sleep "$POLL_SECONDS"
done
