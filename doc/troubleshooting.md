# 문제 해결 기록

## 1. 셸 환경변수가 .env 파일을 덮어쓰는 문제

**발생일**: 2026-04-05

### 증상

- OpenCode에서 토큰이 생성되다가 다시 시작되는 현상 (토큰 수 제한 시 나타나는 동작)
- 응답이 비정상적으로 짧게 잘림
- "인터넷이 끊겼다"는 메시지가 반복적으로 표시
- llama UI (`localhost:8000`, `localhost:8001`)에서는 정상 동작

### 원인

Docker Compose의 환경변수 우선순위:

```
셸 환경변수 (export) > .env 파일
```

이전 세션에서 직접 `export`한 환경변수가 셸에 잔존하여 `.env` 파일의 값을 덮어쓰고 있었다.

```bash
# 셸에 남아있던 값 (이전 세션에서 export)
CONTEXT_LENGTH=8192        # .env에는 131072
LLM_MEMORY_LIMIT=48g       # .env에는 64g
MAX_MODEL_LEN=8192
```

이로 인해 31B 서버가 `-c 8192` (슬롯당 4,096 토큰)로 실행됨.
OpenCode의 시스템 프롬프트 + 사용자 입력이 4,096 토큰을 초과하면 HTTP 400 에러 발생 → 클라이언트에서 "연결 끊김"으로 표시.

### 진단 과정

**1단계: 서버 로그 확인**

```bash
docker logs llm-server 2>&1 | grep "n_ctx"
```

출력:
```
llama_context: n_ctx         = 8192
llama_context: n_ctx_seq     = 4096
llama_context: n_ctx_seq (4096) < n_ctx_train (262144)
```

→ 컨텍스트가 4,096으로 축소된 것 확인. `.env`에는 `131072`로 설정되어 있음.

**2단계: 실제 실행 커맨드 확인**

```bash
docker inspect llm-server --format '{{.Config.Cmd}}'
```

출력에서 `-c 8192` 확인 → `.env`의 `131072`가 아닌 다른 값이 적용됨.

**3단계: 셸 환경변수 확인**

```bash
echo "CONTEXT_LENGTH: ${CONTEXT_LENGTH}"
```

출력: `CONTEXT_LENGTH: 8192` → 셸 변수가 `.env`를 덮어쓰고 있었음.

**4단계: 전체 관련 환경변수 탐색**

```bash
env | grep -E "CONTEXT|LLM_MEMORY|PARALLEL|MODEL|OMP_NUM|FAST_" | sort
```

출력:
```
CONTEXT_LENGTH=8192
LLM_MEMORY_LIMIT=48g
MAX_MODEL_LEN=8192
MODEL_DIR=./models
MODEL_FILENAME=gemma-4-31B-it-Q4_K_M.gguf
OMP_NUM_THREADS=32
SERVED_MODEL_NAME=gemma-4-31b-it
VLLM_MEMORY_LIMIT=48g
```

### 해결

```bash
# 1. 셸 환경변수 해제
unset CONTEXT_LENGTH LLM_MEMORY_LIMIT MAX_MODEL_LEN \
      MODEL_DIR MODEL_FILENAME OMP_NUM_THREADS \
      SERVED_MODEL_NAME VLLM_MEMORY_LIMIT

# 2. 컨테이너 재생성
docker compose up -d --force-recreate llm llm-fast opencode
```

### 해결 후 상태

| 서버 | 슬롯당 컨텍스트 | 슬롯 수 | 총 컨텍스트 |
|------|----------------|---------|------------|
| 31B (이전) | 4,096 | 2 | 8,192 |
| **31B (수정 후)** | **65,536** | **2** | **131,072** |
| E4B | 32,768 | 4 | 131,072 |

### .env 변경 사항

```diff
- CONTEXT_LENGTH=262144
+ CONTEXT_LENGTH=131072

- LLM_MEMORY_LIMIT=48g
+ LLM_MEMORY_LIMIT=64g
```

- `262144` → `131072`: 두 서버 동시 운영 시 GPU 메모리 안정성 확보
- `48g` → `64g`: 131K 컨텍스트의 KV 캐시를 충분히 수용

### 재발 방지

1. **`.env` 관련 변수를 셸에서 `export`하지 않는다** — Docker Compose가 `.env`를 자동으로 읽음
2. **설정 변경 후 반드시 검증한다**:
   ```bash
   # docker compose가 해석하는 실제 값 확인
   docker compose config 2>&1 | grep -A1 "\-c "

   # 실행 중인 컨테이너의 실제 커맨드 확인
   docker inspect llm-server --format '{{.Config.Cmd}}'

   # 서버의 실제 컨텍스트 크기 확인
   curl -s http://localhost:8000/props | python3 -c \
     "import sys,json;d=json.load(sys.stdin);g=d['default_generation_settings'];print('n_ctx:', g['n_ctx'], 'slots:', d['total_slots'])"
   ```
3. **`docker compose config`로 최종 해석 값을 먼저 확인**한 후 `up`한다

---

## 2. PaperClip 새 회사 에이전트 — memory 파일 누락 에러

**발생일**: 2026-04-05

### 증상

```
File not found: .../companies/0443d02d-.../agents/248f4ed9-.../memory/2026-04-05.md
```

PaperClip에서 새 회사를 설립하고 에이전트를 생성하면, 에이전트가 heartbeat 실행 시 `Read` 도구로 `$AGENT_HOME/memory/YYYY-MM-DD.md`를 읽으려 하지만 해당 파일/디렉토리가 존재하지 않아 에러 발생.

### 원인

에이전트의 `HEARTBEAT.md` 2단계에서 매일 메모리 파일을 읽도록 설정되어 있음:

```markdown
## 2. Local Planning Check
1. Read today's plan from `$AGENT_HOME/memory/YYYY-MM-DD.md` under "## Today's Plan".
```

하지만 PaperClip이 새 회사/에이전트를 생성할 때 `instructions/` 디렉토리만 만들고, `memory/`와 `life/` 디렉토리는 자동 생성하지 않음.

### 해결

```bash
# 에이전트 홈 경로 (workspaces 디렉토리가 실제 $AGENT_HOME)
AGENT_HOME="paperclip-data/instances/default/workspaces/{agent-id}"

# 1. 누락된 디렉토리 생성
mkdir -p "$AGENT_HOME/memory" "$AGENT_HOME/life"

# 2. 오늘 날짜의 초기 메모리 파일 생성
cat > "$AGENT_HOME/memory/$(date +%Y-%m-%d).md" << 'EOF'
# Daily Notes — $(date +%Y-%m-%d)

## Today's Plan

- Initial setup complete. Ready for assignments.

## Timeline

- Company initialized.
EOF
```

### 주의: $AGENT_HOME 경로

PaperClip에서 `$AGENT_HOME`은 `companies/.../agents/{id}/` 가 **아니라** `workspaces/{agent-id}/` 이다:

```
✗ paperclip-data/instances/default/companies/{company-id}/agents/{agent-id}/  ← instructions만 저장
✓ paperclip-data/instances/default/workspaces/{agent-id}/                     ← 실제 $AGENT_HOME
```

`instructions/` 폴더의 파일들(HEARTBEAT.md, SOUL.md 등)도 `$AGENT_HOME`에 복사해야 에이전트가 참조할 때 찾을 수 있다.

### 재발 방지

새 회사를 설립할 때마다:
1. `workspaces/{agent-id}/`에 `memory/`, `life/` 디렉토리를 수동 생성
2. `instructions/` 폴더의 `.md` 파일들을 `workspaces/{agent-id}/`에 복사
3. 오늘 날짜의 초기 메모리 파일 생성

---

## 3. PaperClip CEO 에이전트가 작업을 수행하지 않는 문제

**발생일**: 2026-04-05

### 증상

- CEO에게 "Hire your first engineer" 태스크를 할당했지만 "I have loaded the Paperclip skill... I am ready" 만 출력하고 종료
- 실제 API 호출, 도구 사용, 에이전트 고용 등의 작업을 수행하지 않음

### 원인

**1. E4B 모델 (4.5B) 의 능력 한계**

CEO 에이전트가 `local_llm_fast/gemma-4-e4b-it` (4.5B 파라미터)로 설정되어 있었음. CEO의 heartbeat 절차는:
- 여러 API 엔드포인트 호출 (GET /api/agents/me, GET /api/issues, POST checkout 등)
- 복잡한 멀티스텝 추론 (태스크 분석 → 위임 결정 → 에이전트 고용 → 서브태스크 생성)
- 도구 연쇄 호출 (fetch_url, read_file 등)

4.5B 모델로는 이 수준의 작업을 수행할 수 없어 "ready" 상태에서 멈춤.

**2. $AGENT_HOME이 빈 디렉토리**

`workspaces/{agent-id}/` 디렉토리가 비어있어 HEARTBEAT.md, SOUL.md 등의 참조 파일을 찾을 수 없었음.

### 해결

```bash
# 1. CEO 모델을 31B로 변경
PGPASSWORD=paperclip psql -h localhost -p 54329 -U paperclip -d paperclip -c "
  UPDATE agents
  SET adapter_config = jsonb_set(adapter_config, '{model}', '\"local_llm/gemma-4-31b-it\"'),
      status = 'idle'
  WHERE role = 'ceo';
"

# 2. $AGENT_HOME에 instructions 복사 + memory/life 생성
AGENT_HOME=paperclip-data/instances/default/workspaces/{agent-id}
mkdir -p \$AGENT_HOME/memory \$AGENT_HOME/life
cp companies/{company-id}/agents/{agent-id}/instructions/*.md \$AGENT_HOME/
```

### 모델 역할 가이드

| 역할 | 권장 모델 | 이유 |
|------|----------|------|
| CEO | 31B (고품질) | 멀티스텝 추론, API 호출, 전략적 판단 필요 |
| CTO | 31B (고품질) | 코드 생성, 기술 설계, 복잡한 도구 사용 |
| IC (개별 기여자) | E4B (경량) 또는 31B | 단순 작업은 E4B, 복잡한 작업은 31B |


## 토큰 초과 문제

Error: request (33121 tokens) exceeds the available context size (32768 tokens), try increasing it

---

## 3. PaperClip AI 포트 3100 외부 접근 불가

**발생일**: 2026-04-09

### 증상

- `localhost:3100`으로 접속하면 정상 작동
- 외부에서 포트포워딩으로 접근 시 **접근 거부** (ERR_CONNECTION_REFUSED)
- `ss -tlnp | grep 3100` 결과: `127.0.0.1:3100` (loopback만 바인딩)

### 원인

[config.json](../paperclip-data/instances/default/config.json)의 기본 설정이 로컬 전용:

```json
"server": {
  "deploymentMode": "local_trusted",   // ← loopback 강제
  "exposure": "private",
  "host": "127.0.0.1",                 // ← localhost만 수신
  ...
}
```

PaperClip의 배포 모드 제약:
- `local_trusted`: host가 **반드시** `127.0.0.1` (loopback) 필수, exposure는 `private` 필수
- `authenticated`: host `0.0.0.0` 허용, 로그인 인증 필요

### 에러 로그 (시도 과정)

**시도 1** — `exposure: "local_network"` 사용:
```
✗ Config file: Invalid config: server.exposure: Invalid enum value. Expected 'private' | 'public', received 'local_network'
```
→ `exposure`는 `'private' | 'public'`만 허용

**시도 2** — `deploymentMode: "self_hosted"` + `exposure: "public"` 사용:
```
✗ Config file: Invalid config: server.deploymentMode: Invalid enum value. Expected 'local_trusted' | 'authenticated', received 'self_hosted'
```
→ `deploymentMode`는 `'local_trusted' | 'authenticated'`만 허용

**시도 3** — `deploymentMode: "local_trusted"` + `exposure: "private"` + `host: "0.0.0.0"`:
```
✗ Deployment/auth mode: local_trusted requires loopback host binding (found 0.0.0.0)
```
→ `local_trusted`는 loopback 바인딩 강제

### 해결

파일: `paperclip-data/instances/default/config.json`

```diff
  "server": {
-   "deploymentMode": "local_trusted",
+   "deploymentMode": "authenticated",
    "exposure": "private",
-   "host": "127.0.0.1",
+   "host": "0.0.0.0",
    "port": 3100,
    "allowedHostnames": [
-     "localhost"
+     "localhost",
+     "0.0.0.0"
    ],
    "serveUi": true
  },
  "auth": {
    "baseUrlMode": "explicit",
    "disableSignUp": false,
-   "publicBaseUrl": "http://localhost:3100"
+   "publicBaseUrl": "http://0.0.0.0:3100"
  },
```

핵심 변경 사항:
| 항목 | 변경 전 | 변경 후 |
|------|---------|---------|
| `server.deploymentMode` | `local_trusted` | `authenticated` |
| `server.host` | `127.0.0.1` | `0.0.0.0` |
| `server.allowedHostnames` | `["localhost"]` | `["localhost", "0.0.0.0"]` |
| `auth.publicBaseUrl` | `http://localhost:3100` | `http://0.0.0.0:3100` |

서비스 재시작:
```bash
sudo systemctl restart paperclipai.service
```

### 검증

```bash
# 바인딩 주소 확인 — 0.0.0.0이면 성공
ss -tlnp | grep 3100
# 기대 결과: LISTEN 0  511  0.0.0.0:3100  0.0.0.0:*

# 서비스 상태 확인
sudo systemctl status paperclipai.service
```

### 허용 값 정리

| 설정 | 허용 값 | 비고 |
|------|---------|------|
| `server.deploymentMode` | `local_trusted`, `authenticated` | |
| `server.exposure` | `private`, `public` | `local_trusted`는 `private` 필수 |
| `server.host` | IP 문자열 | `local_trusted`는 `127.0.0.1` 필수 |

> **결론**: `local_trusted`에서는 직접 외부 바인딩이 불가능. 5번의 socat 프록시 방식으로 해결.

---

## 4. PaperClip AI 외부 IP 접속 시 "호스트 이름 허용되지 않음" 에러

**발생일**: 2026-04-09

### 증상

외부 IP(예: `182.229.102.180`)로 포트포워딩하여 접속하면 다음 메시지 표시:

```
182.229.102.180 호스트 이름은 이 paperclip 인스턴스에서 허용되지 않습니다.
이 호스트 이름을 허용하려면 pnpm paperclipai allowed-hostname 182.229.102.180 명령을 실행하십시오
```

### 원인

`config.json`의 `server.allowedHostnames`에 접속에 사용된 호스트네임/IP가 포함되어 있지 않으면 차단됨.

기존 설정:
```json
"allowedHostnames": ["localhost", "0.0.0.0"]
```
→ `localhost`와 `0.0.0.0`만 허용, 외부 IP 접속은 모두 거부

### 해결

와일드카드 `*`를 추가하여 **모든 호스트네임/IP에서 접속 허용**:

```bash
cd /home/aa/vllm
paperclipai allowed-hostname '*' --data-dir ./paperclip-data
sudo systemctl restart paperclipai.service
```

변경 후 `config.json`:
```json
"allowedHostnames": [
  "*",
  "0.0.0.0",
  "localhost"
]
```

### 개별 IP만 허용하려면

특정 IP만 추가할 수도 있음:
```bash
paperclipai allowed-hostname 182.229.102.180 --data-dir ./paperclip-data
```

> **주의**: `allowedHostnames`의 `*` 와일드카드는 실제로는 **동작하지 않음** (Set.has()로 정확한 문자열 매칭만 수행).
> 또한 `deploymentMode`를 `authenticated`로 변경하면 기존 회사 데이터가 보이지 않고 `Instance admin required` 에러가 발생함.
> 아래 5번 항목의 socat 프록시 방식으로 해결.

---

## 5. PaperClip AI 외부 접속 — 최종 해결 (socat 프록시)

**발생일**: 2026-04-09

### 배경 (시도했으나 실패한 방법들)

PaperClip의 `local_trusted` 모드 제약:
- `host`는 반드시 `127.0.0.1` (loopback)
- `exposure`는 반드시 `private`
- `allowedHostnames`의 `*` 와일드카드는 작동하지 않음 (정확 매칭만 지원)

`authenticated` 모드로 변경 시:
- 외부 접속은 가능하지만 **기존 회사 데이터가 보이지 않음**
- `Instance admin required` 에러 발생 (DB에 admin 역할 미설정)
- `local_trusted`는 모든 요청을 `isInstanceAdmin: true`로 자동 처리하지만, `authenticated`는 DB 기반 인증 필요

### 해결 — socat 리버스 프록시

PaperClip은 `local_trusted` + `127.0.0.1:3100`으로 유지하고, `socat`으로 외부 포트를 내부로 포워딩:

```
외부 → 0.0.0.0:3101 (socat) → 127.0.0.1:3100 (PaperClip)
```

**1) socat 설치:**
```bash
sudo apt-get install -y socat
```

**2) systemd 서비스 생성:**
```bash
sudo tee /etc/systemd/system/paperclipai-proxy.service > /dev/null <<'EOF'
[Unit]
Description=PaperclipAI External Proxy (socat 0.0.0.0:3101 -> 127.0.0.1:3100)
After=paperclipai.service
Requires=paperclipai.service

[Service]
Type=simple
ExecStart=/usr/bin/socat TCP-LISTEN:3101,bind=0.0.0.0,reuseaddr,fork TCP:127.0.0.1:3100
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
EOF
```

**3) 서비스 활성화:**
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now paperclipai-proxy.service
```

**4) config.json은 원본 유지:**
```json
"server": {
  "deploymentMode": "local_trusted",
  "exposure": "private",
  "host": "127.0.0.1",
  "port": 3100
}
```

### 포트 구조

| 포트 | 바인딩 | 담당 | 용도 |
|------|--------|------|------|
| `127.0.0.1:3100` | PaperClip | node | 로컬 접속 (localhost) |
| `0.0.0.0:3101` | socat 프록시 | socat | 외부/LAN 접속 |

### 공유기 포트포워딩 설정

| 외부 포트 | 내부 IP | 내부 포트 | 프로토콜 |
|-----------|---------|-----------|----------|
| 30005 | 192.168.219.45 | **3101** | TCP |

### 검증

```bash
# 로컬 접속
curl -s -o /dev/null -w "%{http_code}\n" http://localhost:3100

# LAN 접속 (socat 경유)
curl -s -o /dev/null -w "%{http_code}\n" http://192.168.219.45:3101

# 서비스 상태
sudo systemctl status paperclipai.service
sudo systemctl status paperclipai-proxy.service

# 포트 바인딩
ss -tlnp | grep -E '3100|3101'
```

---

## 6. 에이전트 heartbeat 시 HEARTBEAT.md / instructions 파일 누락 에러 (반복 발생)

**발생일**: 2026-04-09, 2026-04-10 (에이전트 고용할 때마다 반복)

### 증상

```
File not found: /home/aa/vllm/paperclip-data/instances/default/workspaces/{agent-id}/HEARTBEAT.md
```

CEO 에이전트가 heartbeat 실행 시 `$AGENT_HOME/HEARTBEAT.md`를 읽으려 하지만 파일이 없어서 실패.

### 근본 원인 — PaperClip의 설계 구조

**이것은 설정 실수가 아니라 PaperClip의 의도된 아키텍처입니다.**

PaperClip은 instructions 파일과 workspace를 **별도의 경로로 관리**합니다:

```
instructions 파일 (HEARTBEAT.md, SOUL.md 등)
  → companies/{companyId}/agents/{agentId}/instructions/
  → adapter_config.instructionsRootPath로 참조
  → 에이전트 생성 시 자동으로 materializeManagedBundle()이 여기에 기록

$AGENT_HOME (에이전트 워크스페이스)
  → workspaces/{agentId}/
  → heartbeat 시 자동으로 mkdir만 수행 (빈 디렉토리)
  → instructions 파일은 복사되지 않음 ← 여기가 문제
```

**경로 비교:**
| 항목 | 경로 |
|------|------|
| instructions 저장 | `.../companies/.../agents/{id}/instructions/HEARTBEAT.md` |
| `$AGENT_HOME` | `.../workspaces/{id}/` ← **여기에는 파일 없음** |

### 에이전트가 HEARTBEAT.md를 왜 `$AGENT_HOME`에서 찾는가?

HEARTBEAT.md의 내용에 `$AGENT_HOME/memory/YYYY-MM-DD.md` 같은 경로 참조가 있어서,
에이전트(LLM)가 heartbeat 실행 시 `$AGENT_HOME`에서 HEARTBEAT.md를 `Read`하려고 시도합니다.
하지만 PaperClip 서버의 `instructionsRootPath`는 instructions 폴더를 가리키고,
어댑터는 `$AGENT_HOME`만 환경변수로 전달합니다.

**instructions는 어댑터의 시스템 프롬프트로 주입되므로** LLM이 파일을 직접 읽을 필요가 없지만,
LLM이 파일 내용을 보고 "이 파일이 있으니 Read로 다시 읽어보자"라고 판단하는 것입니다.

### 왜 에이전트를 고용할 때마다 반복되는가?

1. CEO가 `paperclip-create-agent` 스킬로 새 에이전트 생성
2. PaperClip이 instructions를 `companies/.../instructions/`에 기록 (정상)
3. 새 에이전트의 첫 heartbeat에서 `$AGENT_HOME` = `workspaces/{new-id}/` (빈 디렉토리)
4. LLM이 instructions 내용(HEARTBEAT.md)을 보고 해당 파일을 `$AGENT_HOME`에서 `Read` 시도
5. 파일 없음 → 에러

### 해결 — 수동 복사 (현재 유일한 방법)

```bash
# 특정 에이전트의 instructions를 workspace에 복사
COMPANY=facae2e1-4110-4373-b4f2-3cbf7bd666ac
AGENT_ID={에이전트-id}
BASE=/home/aa/vllm/paperclip-data/instances/default

cp "$BASE/companies/$COMPANY/agents/$AGENT_ID/instructions/"*.md \
   "$BASE/workspaces/$AGENT_ID/"

# memory, life 디렉토리도 생성 (HEARTBEAT.md에서 참조)
mkdir -p "$BASE/workspaces/$AGENT_ID/memory" \
         "$BASE/workspaces/$AGENT_ID/life"
```

### 해결 — 전체 에이전트 일괄 처리 스크립트

```bash
#!/bin/bash
# fix-agent-workspaces.sh — 모든 에이전트의 instructions를 workspace에 동기화
BASE=/home/aa/vllm/paperclip-data/instances/default

for company_dir in "$BASE"/companies/*/; do
  company_id=$(basename "$company_dir")
  for agent_dir in "$company_dir"agents/*/; do
    agent_id=$(basename "$agent_dir")
    inst="$agent_dir/instructions"
    ws="$BASE/workspaces/$agent_id"

    [[ ! -d "$inst" ]] && continue
    mkdir -p "$ws/memory" "$ws/life"

    for f in "$inst"/*.md; do
      [[ -f "$f" ]] || continue
      fname=$(basename "$f")
      if [[ ! -f "$ws/$fname" ]] || [[ "$f" -nt "$ws/$fname" ]]; then
        cp "$f" "$ws/"
        echo "Synced: $fname → $agent_id"
      fi
    done
  done
done
echo "Done."
```

### 요약

| 질문 | 답변 |
|------|------|
| 내 설정이 잘못된 건가? | **아님** — PaperClip이 원래 이렇게 동작함 |
| instructions 자동 복사 기능이 있는가? | **없음** — instructions는 시스템 프롬프트로만 주입, 파일 복사 없음 |
| 왜 에러가 나는가? | LLM이 HEARTBEAT.md 내용을 보고 `$AGENT_HOME`에서 직접 Read 시도 |
| `provisionCommand`로 자동화 가능한가? | 가능 — `runtimeConfig.provisionCommand`에 복사 스크립트 설정 |

### 향후 자동화 (provisionCommand 활용)

에이전트의 `runtimeConfig`에 provision 명령어를 설정하면 workspace 생성 시 자동 실행됨:

```sql
-- CEO 에이전트에 자동 프로비저닝 설정
UPDATE agents SET runtime_config = jsonb_set(
  COALESCE(runtime_config, '{}'),
  '{provisionCommand}',
  '"cp $PAPERCLIP_WORKSPACE_REPO_ROOT/../companies/*/agents/$PAPERCLIP_AGENT_ID/instructions/*.md $AGENT_HOME/ 2>/dev/null; mkdir -p $AGENT_HOME/memory $AGENT_HOME/life"'
) WHERE id = 'c73df5fb-9c81-4be2-9e3c-81696ae46ac9';
```

> 주의: `provisionCommand`는 execution workspace(git worktree 기반)에서만 실행됨.
> agent_home 폴백 모드에서는 실행되지 않으므로 수동 복사가 더 안전함.