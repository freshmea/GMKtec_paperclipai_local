# Paperclip Agent Memory Runtime Guide

## Runtime Rule

All agents should treat memory retrieval as a three-step flow:

1. Read the current task and local daily note context.
2. Query shared context with `qmd` before broad file reads.
3. Persist durable new facts back into local PARA files, not arbitrary folders.

## Retrieval Order

When the task is about recent operational state, incidents, current routing, approvals, blocked work, or agent health:

1. Query `qmd://paperclip-shared-memory`
2. Read `paperclip-data/instances/default/shared-memory/facts/operational_summary.md` if needed
3. Open only the specific local PARA files needed for agent-level detail

When the task is about historical local work, drafts, project notes, or agent-private context:

1. Read `$AGENT_HOME/memory/YYYY-MM-DD.md`
2. Read relevant `$AGENT_HOME/life/` entities
3. Use `qmd` on `paperclip-workspaces` only if broader recall is needed

## Noise Rules

Do not promote these to shared memory unless they imply a durable issue:

- empty inbox heartbeat notes
- normal no-op timer exits
- repeated "no assigned work" lines

Promote these to shared memory:

- agent health changes (`error`, `running`, `idle`) when operationally relevant
- approvals and approval failures
- blocked parent/child issue relationships
- recurring execution defects
- instruction drift or routing drift
- incidents, outages, auth failures, or board-access failures

## Preferred Commands

```bash
export PATH=/home/aa/.npm-global/bin:$PATH
qmd search "Robot Research Agent error" -c paperclip-workspaces
qmd search "instruction drift" -c paperclip-workspaces
qmd search "blocked follow-up" -c paperclip-shared-memory
```

Use `qmd query` when deeper hybrid recall is worth the latency.

## Automatic Application

The live runtime now reapplies the shared-memory instruction patch during every shared-memory cycle.

- Scheduler: `/home/aa/vllm/scripts/paperclip_shared_memory/scheduled_shared_memory_cycle.sh`
- Manual run: `/home/aa/vllm/scripts/paperclip_shared_memory/run_shared_memory_cycle.sh`
- Patcher: `/home/aa/vllm/scripts/paperclip_shared_memory/patch_agent_instructions.py`

This means:

- existing managed instruction files are rechecked regularly
- the CEO workspace copies of `AGENTS.md` and `HEARTBEAT.md` are also rechecked
- new agents added under the same company tree will receive the shared-memory runtime block on the next cycle
