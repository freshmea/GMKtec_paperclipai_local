# Paperclip Shared Memory Migration Plan

## Goal

Add a shared knowledge and operational memory layer to the existing Paperclip deployment without replacing the current per-agent PARA memory workflow.

Keep the current split:

- Per-agent local memory: `$AGENT_HOME/memory/`, `$AGENT_HOME/life/`, `$AGENT_HOME/MEMORY.md`
- Central orchestration state: embedded Postgres + Paperclip API
- New shared layer: normalized operational facts and shared wiki summaries

## Why This Design

The current deployment already uses agent-local PARA files successfully, but shared recall is weak:

- durable facts are duplicated across agent workspaces
- heartbeat timelines accumulate repeated "no work" notes
- current PARA recall depends on `qmd`, which is not installed yet
- the central Paperclip DB stores runs and issues, but not a normalized shared fact layer

The dxAx stack is useful here because it separates:

- static/shared knowledge
- change-aware operational facts
- graph/semantic retrieval

The full dxAx stack should not be copied as-is because local embedded Qdrant is a poor fit for concurrent heartbeat workers.

## Target Architecture

### Layer 1: Agent-local memory

Keep the existing PARA pattern unchanged.

Use for:

- personal work notes
- daily timeline
- private working context
- draft observations before promotion

### Layer 2: Shared operational memory

Instance-scoped directory:

`paperclip-data/instances/default/shared-memory/`

Contents:

- `README.md`: operator notes
- `candidates/`: raw candidate facts collected from agent-local memory
- `facts/operational_facts.jsonl`: deduplicated shared facts
- `facts/operational_summary.md`: human-readable summary
- `logs/`: run logs from collection/consolidation jobs
- `wiki/`: optional promoted summaries and reports

Use for:

- current agent health facts
- recent decisions
- durable operational incidents
- stable routing facts
- recurring blockers

Do not use for:

- every heartbeat event
- full chat transcript storage
- large documents already tracked elsewhere

### Layer 3: Shared wiki / recall

Use `qmd` as the first retrieval fix because the current PARA instructions already depend on it.

Collections:

- `paperclip-workspaces` -> all workspace markdown under `paperclip-data/instances/default/workspaces`
- `paperclip-shared-memory` -> the new shared-memory directory

## Data Flow

1. Agents continue writing local PARA files.
2. A collector scans recent workspace memory files and emits candidate JSONL entries.
3. A consolidator deduplicates repeated candidates and promotes only durable facts.
4. Operators or future automation can query both local PARA files and shared-memory summaries with `qmd`.

## Candidate Fact Schema

Each candidate record should contain:

- `candidate_id`
- `workspace_id`
- `source_path`
- `source_date`
- `agent_hint`
- `fact_type`
- `text`
- `timestamp`
- `confidence`

## Shared Fact Schema

Each promoted shared fact should contain:

- `fact_id`
- `scope`
- `fact_type`
- `text`
- `sources`
- `first_seen`
- `last_seen`
- `occurrences`
- `status`

Recommended scope values:

- `global`
- `agent`
- `task`

## Rollout Priority

### Priority 1

- Install `qmd`
- Create `shared-memory/` scaffold
- Register `qmd` collections
- Verify keyword recall works against workspace markdown

Acceptance:

- `qmd` exists on host
- collections are listed
- `qmd search` returns workspace hits

### Priority 2

- Add collector and consolidator scripts
- Run one manual collection cycle
- Generate initial `operational_facts.jsonl` and `operational_summary.md`

Acceptance:

- candidate JSONL exists
- shared facts file exists
- summary markdown is readable and useful

### Priority 3

- Add scheduled execution for collector/consolidator
- promote selected shared facts into a richer wiki/report flow
- later, consider replacing flat JSONL facts with server Qdrant or pgvector if concurrency or retrieval depth demands it

Acceptance:

- regular refresh without manual intervention
- duplicate/noise rate stays acceptable

## What Not To Do

- Do not replace per-agent PARA files.
- Do not use local embedded Qdrant for multi-agent shared writes.
- Do not ingest every heartbeat line into shared memory.
- Do not let agents write directly into shared facts without a consolidation step.

## Immediate Operator Commands

Install and bootstrap `qmd`:

```bash
./scripts/paperclip_shared_memory/bootstrap_qmd.sh
```

Run one shared-memory cycle:

```bash
./scripts/paperclip_shared_memory/run_shared_memory_cycle.sh
```

## Next Recommended Upgrade

Once the shared-memory summary proves useful, move the promoted facts into a server-backed store with explicit namespaces and timestamps. Until then, JSONL + markdown keeps the rollout simple and auditable.
