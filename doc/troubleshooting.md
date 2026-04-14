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

---

## 3. PaperclipAI Stale PostgreSQL PID 파일로 인한 크래시 루프

**발생일**: 2026-04-13

### 증상

- `http://localhost:3100` 접속 불가
- `systemctl status paperclipai` → 서비스가 `active (running)` 이지만 즉시 `exit-code` FAILURE 로 전환
- 재시작 카운터가 **389회** 이상 누적 (무한 크래시 루프)
- 로그에 반복적으로:
  ```
  Paperclip server failed to start.
  connect ECONNREFUSED 127.0.0.1:54329
  WARN: Embedded PostgreSQL already running; reusing existing process (pid=3035, port=54329)
  ```

### 원인

내장 PostgreSQL의 `postmaster.pid` 파일이 정리되지 않은 상태로 남음 (stale PID).

1. 어떤 이유로(비정상 종료, 시스템 sleep/hibernate 등) 내장 PostgreSQL 프로세스(PID 3035)가 죽음
2. `paperclip-data/instances/default/db/postmaster.pid` 파일은 남아있음
3. paperclipai가 시작할 때 PID 파일을 보고 "PostgreSQL이 이미 실행 중"이라고 판단
4. 죽은 프로세스의 포트(54329)에 연결 시도 → `ECONNREFUSED`
5. `Restart=always`로 인해 5초마다 무한 반복

```
postmaster.pid 존재 → "이미 실행 중" 오판 → 연결 실패 → 크래시 → 재시작 → 반복
```

### 진단

```bash
# 1. 서비스 상태 확인
systemctl status paperclipai
# → restart counter is at 389

# 2. 로그에서 에러 패턴 확인
journalctl -u paperclipai -n 30
# → ECONNREFUSED 127.0.0.1:54329 반복

# 3. PID 파일이 가리키는 프로세스 존재 여부 확인
cat paperclip-data/instances/default/db/postmaster.pid
# → PID 3035
ps -p 3035
# → 프로세스 없음 (stale PID 확인)

# 4. 포트 54329 리스닝 여부 확인
ss -tlnp | grep 54329
# → 없음 (PostgreSQL 실제로 죽어있음)
```

### 해결

```bash
# 1. 서비스 중지
sudo systemctl stop paperclipai

# 2. stale PID 파일 제거
rm -f paperclip-data/instances/default/db/postmaster.pid

# 3. 서비스 재시작
sudo systemctl start paperclipai

# 4. 확인
ss -tlnp | grep 3100
# → LISTEN 0 511 127.0.0.1:3100 ...
```

### 재발 방지 — paperclipai.service 개선

`paperclipai.service`에 두 가지 보호 장치를 추가함:

**1) `ExecStartPre` — 시작 전 stale PID 자동 정리**

```ini
ExecStartPre=/bin/bash -c '\
  PID_FILE="./paperclip-data/instances/default/db/postmaster.pid"; \
  if [ -f "$PID_FILE" ]; then \
    PG_PID=$(head -1 "$PID_FILE"); \
    if ! kill -0 "$PG_PID" 2>/dev/null; then \
      echo "Removing stale postmaster.pid (pid=$PG_PID not running)"; \
      rm -f "$PID_FILE"; \
    fi; \
  fi'
```

서비스가 시작되기 전에 PID 파일이 가리키는 프로세스가 실제로 살아있는지 확인하고, 죽어있으면 PID 파일을 제거한다.

**2) `StartLimitBurst` + `StartLimitIntervalSec` — 크래시 루프 제한**

```ini
StartLimitIntervalSec=60
StartLimitBurst=5
```

60초 내에 5회 이상 실패하면 systemd가 자동 재시작을 중단한다. 389회 크래시 루프 같은 리소스 낭비를 방지.

### 서비스 파일 변경 후 적용

```bash
sudo cp paperclipai.service /etc/systemd/system/paperclipai.service
sudo systemctl daemon-reload
sudo systemctl restart paperclipai
```

### 참고: 수동 복구 (StartLimitBurst로 중단된 경우)

`StartLimitBurst` 에 도달하여 서비스가 중단된 경우:

```bash
# 실패 카운터 초기화
sudo systemctl reset-failed paperclipai

# 필요시 stale PID 파일 수동 제거
rm -f paperclip-data/instances/default/db/postmaster.pid

# 서비스 재시작
sudo systemctl start paperclipai
```
> agent_home 폴백 모드에서는 실행되지 않으므로 수동 복사가 더 안전함.

---

## 4. PaperclipAI 외부 접속용 `3101/tcp` (`socat`) 미오픈 문제

**발생일**: 2026-04-14

### 증상

- `paperclipai` 자체는 동작하지만 외부에서 `3101` 포트로 접속 불가
- `ss -ltnp | grep 3101` 결과가 비어 있음
- `paperclipai.service`는 살아 있거나 재시작되지만 `paperclipai-proxy.service`는 `failed`
- 외부 접속 프록시가 켜져 있어야 하는 구조인데, 재부팅이나 재설치 후 `3101`이 열리지 않음

### 당시 상태

`3100`은 loopback 전용으로 정상 리스닝 중:

```bash
LISTEN 0 511 127.0.0.1:3100 0.0.0.0:* users:(("node",pid=...,fd=...))
```

하지만 `3101`은 열려 있지 않았음:

```bash
ss -ltnp | grep 3101
# 출력 없음
```

그리고 별도 프록시 서비스는 실패 상태:

```bash
systemctl status paperclipai-proxy.service --no-pager
# Active: failed (Result: exit-code)
```

### 원인

기존 구조는 다음과 같았음:

- `paperclipai.service` → `127.0.0.1:3100`에서 본체 실행
- `paperclipai-proxy.service` → `socat 0.0.0.0:3101 -> 127.0.0.1:3100`

문제는 두 서비스가 분리되어 있어서:

1. `paperclipai`가 재시작/실패 루프에 들어가면
2. `paperclipai-proxy.service`도 같이 정지/실패 상태로 남고
3. 최종적으로 `paperclipai`만 살아 있고 `3101` 프록시는 죽은 상태가 됨

즉, 외부 접속 포트 `3101`의 생명주기가 본체 서비스와 안정적으로 묶여 있지 않았음.

### 첫 시도와 실패 원인

처음에는 `paperclipai.service`의 `ExecStart=` 안에 직접 bash wrapper를 길게 넣어서

- `paperclipai` 실행
- `3100` 포트 오픈 대기
- `socat` 실행
- 둘 중 하나가 죽으면 전체 종료

를 처리하려고 했음.

하지만 systemd가 `ExecStart=` 안의 `${PAPERCLIP_PID}`, `${SOCAT_PID}` 같은 문자열을 shell 변수로 넘기기 전에 systemd environment variable처럼 먼저 해석하면서 다음 에러가 발생함:

```text
Referenced but unset environment variable evaluates to an empty string: PAPERCLIP_PID, SOCAT_PID, exit_code
/bin/bash: wait: `': PID가 아니거나 적절한 작업 명세 없음
```

즉, inline shell script 방식은 systemd의 변수 해석과 충돌해서 실패했음.

### 최종 해결

`ExecStart=` 안의 긴 inline bash를 버리고, 별도 실행 스크립트로 분리함.

#### 1. `paperclipai.service`

핵심 실행부를 다음처럼 단순화:

```ini
ExecStart=/bin/bash /home/aa/vllm/run_paperclipai_service.sh
```

또한 크래시 루프 제한을 `Unit` 섹션으로 정리:

```ini
[Unit]
StartLimitIntervalSec=60
StartLimitBurst=5
```

#### 2. `run_paperclipai_service.sh`

이 스크립트가 실제 런타임 오케스트레이션을 담당함:

1. `paperclipai run --data-dir ./paperclip-data` 백그라운드 실행
2. `127.0.0.1:3100`이 실제로 열릴 때까지 최대 60초 대기
3. 포트가 열리면 `socat TCP-LISTEN:3101,bind=0.0.0.0,reuseaddr,fork TCP:127.0.0.1:3100` 실행
4. 둘 중 하나가 죽으면 cleanup 후 전체 프로세스 종료

이렇게 하면 `3101` 프록시가 본체와 항상 같이 올라오고 같이 내려감.

#### 3. `install_paperclipai_service.sh`

설치 스크립트도 함께 수정:

- `run_paperclipai_service.sh`를 실행 가능 파일로 설치
- 기존 `paperclipai-proxy.service`가 남아 있으면 `disable --now` 후 제거
- 이후 `paperclipai.service`만 재시작하도록 변경

### 적용 방법

```bash
cd /home/aa/vllm
sudo bash ./install_paperclipai_service.sh
sudo systemctl daemon-reload
sudo systemctl restart paperclipai.service
```

### 검증 방법

#### 1. 서비스 상태 확인

```bash
systemctl status paperclipai.service --no-pager
```

정상 기대값:

- `Active: active (running)`

#### 2. 포트 확인

```bash
ss -ltnp | grep -E '3100|3101'
```

정상 기대값:

```bash
LISTEN ... 127.0.0.1:3100 ...
LISTEN ... 0.0.0.0:3101 ...
```

#### 3. 프록시 응답 확인

```bash
curl -I http://127.0.0.1:3101
```

정상 기대값:

```text
HTTP/1.1 200 OK
```

### 실제 검증 결과

수정 후 `run_paperclipai_service.sh`를 직접 실행하여 다음을 확인함:

- `paperclipai` 본체 정상 기동
- `127.0.0.1:3100` 리스닝 확인
- `0.0.0.0:3101` 리스닝 확인
- `curl -I http://127.0.0.1:3101` → `HTTP/1.1 200 OK`

예시:

```bash
LISTEN 0 511 127.0.0.1:3100 0.0.0.0:* users:(("node",pid=953199,fd=30))
LISTEN 0 5   0.0.0.0:3101   0.0.0.0:* users:(("socat",pid=953265,fd=5))
```

### 운영 메모

- 앞으로 `3101` 외부 접속 문제는 먼저 `paperclipai.service` 하나만 보면 됨.
- `paperclipai-proxy.service`는 더 이상 운영 기준 서비스가 아님.
- 재부팅 후 외부 접속이 안 되면 가장 먼저 다음 세 가지를 확인:

```bash
systemctl status paperclipai.service --no-pager
ss -ltnp | grep -E '3100|3101'
journalctl -u paperclipai.service -n 80 --no-pager
```

---

## 5. `opencode` 명령을 찾지 못해 에이전트 실행 실패

**발생일**: 2026-04-14

### 증상

- Paperclip run transcript에 다음 메시지가 표시됨:

```text
[system] Command not found in PATH: "opencode"
```

- 일부 heartbeat/run은 fallback workspace까지 생성한 뒤 즉시 실패
- 에이전트 실행이 `opencode_local` 어댑터 단계에서 멈춤

### 원인

호스트에 `opencode` CLI는 설치되어 있었지만, 실제 바이너리 위치가 다음 한 곳뿐이었음:

```bash
/home/aa/.local/bin/opencode
```

즉, 로그인 셸이나 특정 환경에서는 동작할 수 있었지만, Paperclip의 비대화형 실행 경로에서는 `opencode` 문자열만으로 명령을 찾지 못하는 상황이 발생함.

실제 확인 결과:

```bash
opencode --version
# /bin/bash: opencode: command not found

/home/aa/.local/bin/opencode models
# 정상 동작
```

정리하면:

- **절대경로**로는 실행 가능
- **PATH 기반 탐색**에서는 실패 가능

### 해결

두 층으로 보강함.

#### 1. `run_paperclipai_service.sh`에서 PATH 강제 설정

서비스 러너 시작 시 다음 환경을 명시적으로 export:

```bash
export PATH="/home/aa/.local/bin:/home/aa/.npm-global/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
export HOME="/home/aa"
```

이렇게 하면 PaperclipAI 서비스 프로세스와 그 하위 프로세스가 `~/.local/bin`을 항상 우선적으로 볼 수 있음.

#### 2. `/usr/local/bin/opencode` 심볼릭 링크 생성

설치 스크립트에서 다음 링크를 자동 생성하도록 수정:

```bash
sudo ln -sf /home/aa/.local/bin/opencode /usr/local/bin/opencode
```

이제 비대화형 PATH가 `~/.local/bin`을 누락하더라도 `/usr/local/bin/opencode`로 명령을 찾을 수 있음.

### 설치 스크립트 보강

`install_paperclipai_service.sh`에 다음 검증도 추가함:

```bash
if [[ ! -x /home/aa/.local/bin/opencode ]]; then
    echo "ERROR: /home/aa/.local/bin/opencode 실행 파일을 찾을 수 없습니다."
    echo "먼저 호스트 OpenCode CLI를 설치하세요."
    exit 1
fi
```

즉, 호스트 CLI가 아예 없는 상태에서는 조용히 실패하지 않고 설치 단계에서 즉시 경고함.

### 적용 방법

```bash
cd /home/aa/vllm
sudo bash ./install_paperclipai_service.sh
sudo systemctl daemon-reload
sudo systemctl restart paperclipai.service
```

### 검증 방법

#### 1. 전역 PATH에서 `opencode`가 보이는지 확인

```bash
which opencode
```

정상 기대값:

```bash
/usr/local/bin/opencode
```

#### 2. 버전 확인

```bash
opencode --version
```

정상 기대값:

- 버전 문자열 출력
- 더 이상 `command not found`가 나오지 않음

#### 3. 모델 조회 확인

```bash
opencode models
```

정상 기대값:

- 사용 가능한 모델 목록 출력

#### 4. 서비스 로그 확인

```bash
journalctl -u paperclipai.service -n 80 --no-pager
```

정상 기대값:

- 더 이상 `Command not found in PATH: "opencode"` 메시지가 없음

### 운영 메모

- `opencode_local` 어댑터는 Docker 컨테이너 내부 `opencode-web`와 별개로, **호스트의 `opencode` CLI** 가 필요함.
- 따라서 Docker가 떠 있어도 호스트 PATH에 `opencode`가 없으면 Paperclip agent 실행은 실패할 수 있음.
- 이후 유사 문제 발생 시 가장 먼저 아래를 확인:

```bash
which opencode
ls -l /home/aa/.local/bin/opencode /usr/local/bin/opencode
opencode --version
opencode models
```
