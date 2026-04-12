# Paperclip AI 에이전트 점검 보고서

**작성일:** 2026-04-12
**회사 ID:** `facae2e1-4110-4373-b4f2-3cbf7bd666ac`
**인스턴스:** `/home/aa/vllm/paperclip-data/instances/default`

---

## 1. 에이전트 구성 현황

| 에이전트 | ID (앞 8자리) | 어댑터 | 모델 | 역할 |
|----------|---------------|--------|------|------|
| CEO | `c73df5fb` | `codex_local` | (기본) | 전략, 위임, 경영 |
| CMO | `0b9b2780` | `opencode_local` | gemma-4-26b | 마케팅, 콘텐츠 |
| CTO | `1a708d66` | `opencode_local` | gemma-4-26b | 기술, 아키텍처 |
| UXDesigner | `ea406069` | `opencode_local` | gemma-4-26b | UX 설계, HRI |
| ROS2 Expert | `b1805ca8` | `opencode_local` | gemma-4-26b | ROS2, 로보틱스 |
| Robot Research Agent | `4d34f025` | `opencode_local` | gemma-4-26b | 기술 리서치 |

---

## 2. 수정 사항 요약

### 2.1 cwd 경로 수정 (이전 세션)

5개 에이전트의 `adapterConfig.cwd`가 CEO 워크스페이스를 가리키고 있었음.

| 에이전트 | 수정 전 cwd | 수정 후 cwd |
|----------|-------------|-------------|
| CMO | CEO 워크스페이스 (`c73df5fb`) | 자체 워크스페이스 (`0b9b2780`) |
| CTO | CEO 워크스페이스 (`c73df5fb`) | 자체 워크스페이스 (`1a708d66`) |
| UXDesigner | CEO 워크스페이스 (`c73df5fb`) | 자체 워크스페이스 (`ea406069`) |
| ROS2 Expert | CTO 워크스페이스 (`1a708d66`) | 자체 워크스페이스 (`b1805ca8`) |
| Robot Research | CTO 워크스페이스 (`1a708d66`) | 자체 워크스페이스 (`4d34f025`) |

### 2.2 instruction 파일 통합

**문제:** 에이전트별 `HEARTBEAT.md`, `SOUL.md`를 워크스페이스에 별도 파일로 생성했으나, `opencode_local` 어댑터는 `instructionsFilePath`에 지정된 단일 파일만 읽음. 워크스페이스 md 파일은 UI와 연결 안 됨.

**해결:** 5개 에이전트의 `/instructions/AGENTS.md`에 HEARTBEAT + SOUL 내용을 인라인 통합.
워크스페이스에서 `AGENTS.md`, `HEARTBEAT.md`, `SOUL.md` 삭제.

| 에이전트 | 통합 후 AGENTS.md 크기 |
|----------|----------------------|
| CMO | 5,477 bytes |
| CTO | 5,188 bytes |
| UXDesigner | 5,164 bytes |
| ROS2 Expert | 5,490 bytes |
| Robot Research | 5,390 bytes |
| CEO | 3,253 bytes (기존 유지 — codex_local은 워크스페이스 파일 자동 발견) |

**CEO 예외:** `codex_local` 어댑터는 워크스페이스의 `AGENTS.md`를 자동 발견하므로 기존 분리 구조(AGENTS.md + HEARTBEAT.md + SOUL.md + TOOLS.md) 유지.

### 2.3 API 체크아웃 에러 수정

**문제:** `{"error":"API route not found"}` — 에이전트가 `GET /api/issues/{id}/checkout`으로 호출. 이 라우트는 `POST` 전용.

**원인:** AGENTS.md에 `POST /api/issues/{id}/checkout`이라고 텍스트로만 기술. Gemma 26B 모델이 curl 변환 시 `-X POST`와 JSON body를 누락.

**해결:** 5개 에이전트 AGENTS.md의 하트비트 섹션을 복사-붙여넣기 가능한 curl 코드 블록으로 교체:
```bash
curl -s -X POST "$PAPERCLIP_API_URL/api/issues/{issueId}/checkout" \
  -H "Authorization: Bearer $PAPERCLIP_API_KEY" \
  -H "Content-Type: application/json" \
  -H "X-Paperclip-Run-Id: $PAPERCLIP_RUN_ID" \
  -d "{\"agentId\": \"$PAPERCLIP_AGENT_ID\", \"expectedStatuses\": [\"todo\", \"backlog\", \"blocked\"]}"
```

### 2.4 스킬 설치

모든 에이전트에 `desiredSkills: []`이었음. `paperclip` + `para-memory-files` 스킬을 sync 호출로 설치.

> **참고:** 스킬은 `~/.claude/skills/`에 물리적으로 설치되어 있고 번들 스킬로 자동 포함됨. `desiredSkills`는 관리 추적용.

### 2.5 파일 마이그레이션 (이전 세션)

CEO 워크스페이스에 잘못 생성된 `robot/`, `reports/`, `research/`, `sources/` 디렉토리를 Robot Research Agent 워크스페이스로 이동.

---

## 3. 현재 파일 구조

### instruction 파일 (Paperclip이 읽는 파일)

```
companies/{companyId}/agents/
├── {CEO-id}/instructions/
│   ├── AGENTS.md          # 3,253 bytes — codex_local의 진입점
│   ├── HEARTBEAT.md       # 별도 파일 (codex_local 전용)
│   ├── SOUL.md            # 별도 파일 (codex_local 전용)
│   └── TOOLS.md           # 별도 파일 (codex_local 전용)
├── {CMO-id}/instructions/
│   └── AGENTS.md          # 5,477 bytes — 역할 + 하트비트 + 페르소나 통합
├── {CTO-id}/instructions/
│   └── AGENTS.md          # 5,188 bytes — 역할 + 하트비트 + 페르소나 통합
├── {UXDesigner-id}/instructions/
│   └── AGENTS.md          # 5,164 bytes — 역할 + 하트비트 + 페르소나 통합
├── {ROS2-id}/instructions/
│   └── AGENTS.md          # 5,490 bytes — 역할 + 하트비트 + 페르소나 통합
└── {Research-id}/instructions/
    └── AGENTS.md          # 5,390 bytes — 역할 + 하트비트 + 페르소나 통합
```

### 워크스페이스 (에이전트 런타임 작업 공간)

```
workspaces/
├── {CEO-id}/              # codex_local — AGENTS.md 자동 발견
│   ├── AGENTS.md, HEARTBEAT.md, SOUL.md, TOOLS.md
│   ├── life/, memory/
│   └── (기타 작업 파일)
├── {CMO-id}/
│   ├── life/, memory/     # 런타임 전용
├── {CTO-id}/
│   ├── life/, memory/
├── {UXDesigner-id}/
│   ├── life/, memory/
├── {ROS2-id}/
│   ├── life/, memory/
└── {Research-id}/
    ├── life/, memory/
    └── robot/, reports/, research/, sources/  # 리서치 결과물
```

---

## 4. instruction 자동 수정 가능 여부

### 4.1 Paperclip 서버의 자동 수정

| 항목 | 동작 |
|------|------|
| `instructionsBundleMode: managed` | Paperclip 서버가 instruction 파일의 **경로**를 관리함. 내용은 수정하지 않음 |
| `PATCH /api/agents/{id}/instructions-path` | instruction 파일 **경로만** 변경 가능. 파일 내용 수정 API 없음 |
| 스킬 sync | `~/.claude/skills/`에 스킬 파일을 설치/제거. 에이전트 instruction과는 별개 |
| 서버 업그레이드 | Paperclip 서버 업데이트 시 스킬 파일(`paperclip`, `para-memory-files` 등)은 자동 갱신 가능. instruction은 아님 |

**결론: Paperclip 서버는 instruction 파일의 내용을 자동으로 수정하지 않습니다.**

### 4.2 에이전트 자체 수정 가능 여부

| 어댑터 | 파일 시스템 접근 | instruction 자체 수정 가능 |
|--------|-----------------|--------------------------|
| `codex_local` (CEO) | 워크스페이스 cwd 내 읽기/쓰기 가능 | ⚠️ 워크스페이스의 AGENTS.md는 수정 가능 (codex가 자동 발견하는 파일). `instructions/` 디렉토리가 cwd 외부에 있으면 접근 불가 |
| `opencode_local` (나머지) | 워크스페이스 cwd 내 읽기/쓰기 가능 | ❌ `instructionsFilePath`가 `companies/.../instructions/AGENTS.md`를 가리키며 이는 cwd 외부. 에이전트가 자체 instruction을 수정할 수 없음 |

**결론:**
- **opencode_local 에이전트 5개:** instruction은 외부에서만 수정 가능 (보드 사용자 또는 CEO의 파일 시스템 접근)
- **CEO (codex_local):** 워크스페이스 내 AGENTS.md는 자체 수정 가능하지만, `companies/.../instructions/` 경로의 파일은 cwd가 워크스페이스를 가리키므로 접근 불가

### 4.3 추가 instruction 파일

현재 `instructionsBundleMode: managed` + `instructionsEntryFile: AGENTS.md` 설정에서:
- CEO (`codex_local`): `instructions/` 디렉토리 내 여러 파일을 자동 발견하고 번들링 (현재 AGENTS.md, HEARTBEAT.md, SOUL.md, TOOLS.md)
- 나머지 (`opencode_local`): `instructionsFilePath`가 가리키는 **단일 파일만** 읽음. 추가 파일은 무시됨

---

## 5. CEO를 통한 instruction 관리 방안

### 방안 A: CEO에게 instruction 관리 태스크 위임 (수동)

Paperclip UI에서 CEO에게 이슈를 생성:

```
제목: "CMO instruction 업데이트 — 마케팅 KPI 지표 추가"
설명: "CMO의 AGENTS.md에 마케팅 KPI 추적 지침을 추가해주세요."
```

**한계:** CEO(`codex_local`)의 cwd가 워크스페이스를 가리키므로, `companies/.../instructions/` 경로에 직접 파일 쓰기를 할 수 있는지는 codex 어댑터의 파일 시스템 접근 범위에 따라 다름.

### 방안 B: Routine(루틴)으로 자동화

CEO에게 주기적 루틴을 생성하여 instruction 점검 태스크를 자동 생성:

```bash
# 1. 프로젝트 생성 (이미 있으면 기존 프로젝트 ID 사용)
POST /api/companies/{companyId}/projects
{ "name": "Agent Governance", "description": "에이전트 instruction 관리 및 점검" }

# 2. CEO에게 루틴 생성
POST /api/companies/{companyId}/routines
{
  "title": "주간 에이전트 instruction 점검",
  "description": "각 에이전트의 AGENTS.md instruction이 최신 상태인지 점검하고, 필요 시 업데이트 요청을 보드에 에스컬레이션",
  "assigneeAgentId": "{CEO-agent-id}",
  "projectId": "{project-id}",
  "priority": "low",
  "status": "active",
  "concurrencyPolicy": "skip_if_active",
  "catchUpPolicy": "skip_missed"
}

# 3. 주간 스케줄 트리거 추가 (매주 월요일 오전 9시)
POST /api/routines/{routineId}/triggers
{
  "kind": "schedule",
  "cronExpression": "0 9 * * 1",
  "timezone": "Asia/Seoul"
}
```

**한계:** CEO가 instruction 파일을 직접 수정할 수 있는지는 cwd 접근 범위에 따라 다름. 수정이 필요하면 보드에 에스컬레이션하는 것이 안전함.

### 방안 C: instruction 경로를 워크스페이스 내부로 변경 (권장)

각 에이전트의 `instructionsFilePath`를 워크스페이스 내부로 변경하면, 에이전트 자신이 instruction을 수정할 수 있음:

```bash
# CMO 예시
PATCH /api/agents/{CMO-id}/instructions-path
{ "path": "AGENTS.md" }
# → cwd 기준 상대경로로 해석: workspaces/{CMO-id}/AGENTS.md
```

**장점:**
- 에이전트가 자체 instruction을 업데이트 가능
- CEO가 각 에이전트 워크스페이스에 접근하여 instruction 수정 가능

**단점:**
- 에이전트가 자신의 instruction을 임의로 변경할 위험
- 현재 `companies/.../instructions/` 구조와 일관성 깨짐

### 방안 D: CEO의 cwd에 전체 instruction을 포함 (고급)

CEO의 워크스페이스에 모든 에이전트의 instruction 사본을 두고, CEO가 관리:

```
workspaces/{CEO-id}/
├── agent-instructions/
│   ├── cmo/AGENTS.md
│   ├── cto/AGENTS.md
│   └── ...
```

그 후 각 에이전트의 `instructionsFilePath`를 CEO 워크스페이스 경로로 지정.

**장점:** 중앙 집중 관리
**단점:** 경로 의존성, CEO 워크스페이스 크기 증가

---

## 6. 권장 사항

1. **현재 구조 유지** (`companies/.../instructions/AGENTS.md`) — 가장 안전하고 Paperclip 기본 설계에 부합
2. **instruction 수정은 보드 사용자가 직접 수행** — 파일 시스템 접근으로 AGENTS.md 편집
3. **CEO에게 점검 루틴 추가** — instruction 변경이 필요한 상황을 자동 감지하고 보드에 에스컬레이션
4. **에이전트 자체 instruction 수정은 비권장** — 안정성/거버넌스 위험
