from __future__ import annotations

import json
import re
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterable


ROOT = Path(__file__).resolve().parents[2]
INSTANCE_DIR = ROOT / "paperclip-data" / "instances" / "default"
WORKSPACES_DIR = INSTANCE_DIR / "workspaces"
SHARED_DIR = INSTANCE_DIR / "shared-memory"
CANDIDATES_DIR = SHARED_DIR / "candidates"
LOGS_DIR = SHARED_DIR / "logs"

NOISE_PATTERNS = (
    "exiting cleanly",
    "inbox is empty",
    "no tasks assigned",
    "no issue is currently assigned",
    "no ceo-assigned issues",
    "할당된 작업 없음",
    "정상 종료",
    "정상 heartbeat",
    "inbox-lite 응답 빈 배열",
    "pAPERCLIP_TASK_ID 미설정".lower(),
    "checkout 생략 후 종료",
    "대기 상태 유지",
    "현재 할당된 작업 없음",
)

STATUS_PATTERNS = (
    "remains in `error`",
    "is now `running`",
    "is now `idle`",
    "available for routing",
    "delegated it to",
    "closed ",
    "created child task",
    "blockedByIssueIds",
)

NOTE_PATTERNS = (
    "approval",
    "board access required",
    "incident",
    "outage",
    "follow-up",
    "remediation",
    "복구",
    "승인",
    "장애",
    "후속",
    "실패",
)


@dataclass
class Candidate:
    candidate_id: str
    workspace_id: str
    source_path: str
    source_date: str
    agent_hint: str | None
    fact_type: str
    text: str
    timestamp: str
    confidence: float


def iter_memory_files() -> Iterable[Path]:
    for workspace in sorted(WORKSPACES_DIR.iterdir()):
        memory_dir = workspace / "memory"
        if not memory_dir.exists():
            continue
        files = sorted(memory_dir.glob("*.md"))
        for path in files[-3:]:
            yield path


def infer_agent_hint(workspace_id: str) -> str | None:
    agents_path = WORKSPACES_DIR / workspace_id / "AGENTS.md"
    if not agents_path.exists():
        return None
    for line in agents_path.read_text(encoding="utf-8", errors="ignore").splitlines()[:6]:
        line = line.strip()
        if line.startswith("You are "):
            return line.removeprefix("You are ").rstrip(".")
    return None


def classify_line(text: str) -> tuple[str, float] | None:
    lowered = text.lower()
    if any(pattern in lowered for pattern in NOISE_PATTERNS):
        return None
    if any(pattern in lowered for pattern in STATUS_PATTERNS):
        return ("operational_status", 0.85)
    if "heartbeat run" in lowered or "wake reason" in lowered:
        return None
    if "heartbeat" in lowered or "하트비트" in lowered:
        return None
    if "delegated" in lowered or "blocked" in lowered or "closed" in lowered:
        return ("task_flow", 0.75)
    if "error" in lowered or "running" in lowered or "idle" in lowered:
        return ("agent_state", 0.8)
    if any(pattern in lowered for pattern in NOTE_PATTERNS):
        return ("note", 0.55)
    return None


def collect() -> list[Candidate]:
    timestamp = datetime.now(timezone.utc).isoformat()
    candidates: list[Candidate] = []
    bullet_re = re.compile(r"^\s*-\s+(.*)$")

    for path in iter_memory_files():
        workspace_id = path.parents[1].name
        agent_hint = infer_agent_hint(workspace_id)
        lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
        for index, line in enumerate(lines, start=1):
            match = bullet_re.match(line)
            if not match:
                continue
            text = match.group(1).strip()
            classified = classify_line(text)
            if not classified:
                continue
            fact_type, confidence = classified
            candidates.append(
                Candidate(
                    candidate_id=f"{workspace_id}:{path.stem}:{index}",
                    workspace_id=workspace_id,
                    source_path=str(path.relative_to(ROOT)).replace("\\", "/"),
                    source_date=path.stem,
                    agent_hint=agent_hint,
                    fact_type=fact_type,
                    text=text,
                    timestamp=timestamp,
                    confidence=confidence,
                )
            )
    return candidates


def main() -> None:
    CANDIDATES_DIR.mkdir(parents=True, exist_ok=True)
    LOGS_DIR.mkdir(parents=True, exist_ok=True)
    candidates = collect()
    output = CANDIDATES_DIR / f"candidates_{datetime.now().strftime('%Y%m%dT%H%M%SZ')}.jsonl"
    with output.open("w", encoding="utf-8") as handle:
        for item in candidates:
            handle.write(json.dumps(item.__dict__, ensure_ascii=False) + "\n")
    log_path = LOGS_DIR / "collect_candidates.log"
    log_path.write_text(
        f"{datetime.now(timezone.utc).isoformat()} collected {len(candidates)} candidates into {output.name}\n",
        encoding="utf-8",
    )
    print(json.dumps({"status": "ok", "candidates": len(candidates), "output": str(output)}, ensure_ascii=False))


if __name__ == "__main__":
    main()
