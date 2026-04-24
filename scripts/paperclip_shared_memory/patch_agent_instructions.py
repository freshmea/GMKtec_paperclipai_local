from __future__ import annotations

from pathlib import Path
import re


INSTANCE_ROOT = Path("/home/aa/vllm/paperclip-data/instances/default")
COMPANY_ID = "facae2e1-4110-4373-b4f2-3cbf7bd666ac"
CEO_ID = "c73df5fb-9c81-4be2-9e3c-81696ae46ac9"

COMPANY_ROOT = INSTANCE_ROOT / "companies" / COMPANY_ID / "agents"
CEO_WORKSPACE = INSTANCE_ROOT / "workspaces" / CEO_ID

MEMORY_BLOCK = """
## Shared Memory Runtime

Shared memory is now part of the default recall path.
Use the following order before broad file reads:

1. Read the current local daily note first.
2. If the task is about recent operating status, approvals, blocked follow-up, agent health, incidents, or instruction drift, query shared memory with `qmd` first.
3. Only open broader files under `$AGENT_HOME/memory/` or `$AGENT_HOME/life/` when the `qmd` results are not enough.

Default commands:

```bash
export PATH=/home/aa/.npm-global/bin:$PATH
qmd search "<query>" -c paperclip-shared-memory
qmd search "<query>" -c paperclip-workspaces
```

Recall rules:

* Recent operating status questions: prefer `paperclip-shared-memory`
* Recent work context from other agents: prefer `paperclip-workspaces`
* Personal drafts, local working memory, and tacit notes: prefer `$AGENT_HOME/memory/` and `$AGENT_HOME/life/`

Promote only durable operational facts into shared memory:

* agent state changes such as `error`, `running`, or `idle` when they affect operations
* approvals, board-access failures, auth failures
* blocked parent or child relationships and required follow-up
* instruction drift, routing drift, and recurring defects

Do not promote heartbeat noise:

* empty inbox checks
* repeated no-op or normal-exit logs
* intermediate private drafting notes
""".strip()

HEARTBEAT_INSERT = """
## 5.5. Shared Memory Recall

- Before broad file reads, run the shared-memory recall path first.
- `export PATH=/home/aa/.npm-global/bin:$PATH`
- For recent operational status, approvals, blocked work, incidents, or agent health, run `qmd search "<query>" -c paperclip-shared-memory`
- For recent work context from other agents, run `qmd search "<query>" -c paperclip-workspaces`
- Only open individual memory or life files when the `qmd` result is not enough
""".strip()

OLD_HEARTBEAT_INSERT = """
5.5. Shared memory recall

- broad file read 전에 다음을 우선 수행합니다.
- `export PATH=/home/aa/.npm-global/bin:$PATH`
- 최근 운영 상태/차단/승인/agent health 확인이 필요하면 `qmd search "<query>" -c paperclip-shared-memory`
- 다른 agent의 최근 작업 맥락이 필요하면 `qmd search "<query>" -c paperclip-workspaces`
- 검색 결과로 충분하지 않을 때만 개별 memory/life 파일을 엽니다.
""".strip()

FACT_EXTRACTION_REPLACEMENT = """
## 7. Fact Extraction

1. Check for new conversations since last extraction.
2. Before broad local file reads, query shared operational context with `qmd` when the task concerns recent status, approvals, blocked work, incidents, or agent health.
3. Extract durable local facts to the relevant entity in `$AGENT_HOME/life/` (PARA).
4. Update `$AGENT_HOME/memory/YYYY-MM-DD.md` with timeline entries.
5. Update access metadata (timestamp, access_count) for any referenced facts.
6. Promote only durable operational facts into shared memory; do not promote empty-inbox or no-op heartbeat noise.
""".strip()


def patch_agent_file(path: Path) -> bool:
    text = path.read_text(encoding="utf-8")
    marker = "## Safety Considerations"
    if "## Shared Memory Runtime" in text and marker in text:
        start = text.index("## Shared Memory Runtime")
        end = text.index(marker)
        updated = text[:start] + MEMORY_BLOCK + "\n\n" + text[end:]
    elif marker in text:
        updated = text.replace(marker, MEMORY_BLOCK + "\n\n" + marker, 1)
    else:
        updated = text.rstrip() + "\n\n" + MEMORY_BLOCK + "\n"
    if updated == text:
        return False
    path.write_text(updated, encoding="utf-8")
    return True


def patch_heartbeat_file(path: Path) -> bool:
    text = path.read_text(encoding="utf-8")
    original = text
    text = text.replace(OLD_HEARTBEAT_INSERT + "\n\n", "")
    text = text.replace(OLD_HEARTBEAT_INSERT + "\r\n\r\n", "")
    if "## 6. Delegation" in text:
        text = re.sub(
            r"(?:##\s*)?5\.5\.\s*Shared [Mm]emory [Rr]ecall.*?(?=\n## 6\. Delegation)",
            "",
            text,
            flags=re.S,
        )
        text = text.replace("## 6. Delegation", HEARTBEAT_INSERT + "\n\n## 6. Delegation", 1)
    if "## 7. Fact Extraction" in text and "## 8. Exit" in text:
        start = text.index("## 7. Fact Extraction")
        end = text.index("## 8. Exit")
        text = text[:start] + FACT_EXTRACTION_REPLACEMENT + "\n\n" + text[end:]
    if text == original:
        return False
    path.write_text(text, encoding="utf-8")
    return True


def patch_company_instructions() -> list[Path]:
    changed: list[Path] = []
    for path in COMPANY_ROOT.glob("*/instructions/AGENTS.md"):
        if patch_agent_file(path):
            changed.append(path)
    for path in COMPANY_ROOT.glob("*/instructions/HEARTBEAT.md"):
        if patch_heartbeat_file(path):
            changed.append(path)
    return changed


def patch_ceo_workspace() -> list[Path]:
    changed: list[Path] = []
    agents_path = CEO_WORKSPACE / "AGENTS.md"
    heartbeat_path = CEO_WORKSPACE / "HEARTBEAT.md"
    if agents_path.exists() and patch_agent_file(agents_path):
        changed.append(agents_path)
    if heartbeat_path.exists() and patch_heartbeat_file(heartbeat_path):
        changed.append(heartbeat_path)
    return changed


def main() -> None:
    changed = patch_company_instructions() + patch_ceo_workspace()
    for path in changed:
        print(path)


if __name__ == "__main__":
    main()
