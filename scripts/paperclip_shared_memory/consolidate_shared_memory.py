from __future__ import annotations

import json
from collections import defaultdict
from dataclasses import dataclass, asdict
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
INSTANCE_DIR = ROOT / "paperclip-data" / "instances" / "default"
SHARED_DIR = INSTANCE_DIR / "shared-memory"
CANDIDATES_DIR = SHARED_DIR / "candidates"
FACTS_DIR = SHARED_DIR / "facts"
LOGS_DIR = SHARED_DIR / "logs"


@dataclass
class SharedFact:
    fact_id: str
    scope: str
    fact_type: str
    text: str
    sources: list[str]
    first_seen: str
    last_seen: str
    occurrences: int
    status: str


def load_candidates() -> list[dict]:
    items: list[dict] = []
    seen: set[tuple[str, str]] = set()
    for path in sorted(CANDIDATES_DIR.glob("candidates_*.jsonl")):
        for line in path.read_text(encoding="utf-8").splitlines():
            if line.strip():
                item = json.loads(line)
                key = (item["candidate_id"], item["text"])
                if key in seen:
                    continue
                seen.add(key)
                items.append(item)
    return items


def normalize_text(text: str) -> str:
    return " ".join(text.split()).strip().lower()


def build_facts(candidates: list[dict]) -> list[SharedFact]:
    grouped: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for item in candidates:
        key = (item["fact_type"], normalize_text(item["text"]))
        grouped[key].append(item)

    facts: list[SharedFact] = []
    for index, ((fact_type, _), items) in enumerate(sorted(grouped.items()), start=1):
        items = sorted(items, key=lambda item: item["timestamp"])
        text = items[-1]["text"]
        sources = sorted({item["source_path"] for item in items})
        if fact_type == "note" and len(items) < 2:
            continue
        scope = "agent" if fact_type == "agent_state" else "global"
        facts.append(
            SharedFact(
                fact_id=f"fact-{index:04d}",
                scope=scope,
                fact_type=fact_type,
                text=text,
                sources=sources,
                first_seen=items[0]["timestamp"],
                last_seen=items[-1]["timestamp"],
                occurrences=len(items),
                status="active",
            )
        )
    return facts


def write_outputs(facts: list[SharedFact]) -> None:
    FACTS_DIR.mkdir(parents=True, exist_ok=True)
    LOGS_DIR.mkdir(parents=True, exist_ok=True)

    facts_path = FACTS_DIR / "operational_facts.jsonl"
    with facts_path.open("w", encoding="utf-8") as handle:
        for fact in facts:
            handle.write(json.dumps(asdict(fact), ensure_ascii=False) + "\n")

    summary_path = FACTS_DIR / "operational_summary.md"
    lines = [
        "# Shared Operational Summary",
        "",
        f"Generated: {datetime.now(timezone.utc).isoformat()}",
        "",
        f"Total facts: {len(facts)}",
        "",
    ]
    for fact in facts[:50]:
        lines.append(f"## {fact.fact_id} [{fact.fact_type}]")
        lines.append("")
        lines.append(f"- Scope: `{fact.scope}`")
        lines.append(f"- Occurrences: `{fact.occurrences}`")
        lines.append(f"- Last seen: `{fact.last_seen}`")
        lines.append(f"- Text: {fact.text}")
        lines.append(f"- Sources: {', '.join(fact.sources)}")
        lines.append("")
    summary_path.write_text("\n".join(lines), encoding="utf-8")

    log_path = LOGS_DIR / "consolidate_shared_memory.log"
    log_path.write_text(
        f"{datetime.now(timezone.utc).isoformat()} consolidated {len(facts)} facts\n",
        encoding="utf-8",
    )


def main() -> None:
    candidates = load_candidates()
    facts = build_facts(candidates)
    write_outputs(facts)
    print(json.dumps({"status": "ok", "candidates": len(candidates), "facts": len(facts)}, ensure_ascii=False))


if __name__ == "__main__":
    main()
