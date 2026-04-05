# OpenCode 사용 가이드

## 개요

[OpenCode](https://github.com/opencode-ai/opencode)는 AI 기반 코딩 어시스턴트로, 웹 UI를 통해 LLM과 대화하며 코드 작성, 파일 편집, 셸 명령 실행 등을 수행할 수 있습니다.

- **버전:** 1.3.13 (`smanx/opencode:latest` Docker 이미지)
- **웹 UI:** `http://localhost:3000`
- **LLM 백엔드:** 로컬 llama-server (`http://llm:8000/v1`, OpenAI 호환)
- **작업 디렉토리:** 컨테이너 내부 `/workspace` (호스트의 `./workspace` 마운트)

> **참고:** OpenCode 원본 프로젝트는 아카이브되었으며, 후속 프로젝트는 [Crush](https://github.com/charmbracelet/crush)입니다.
> Docker 이미지 `smanx/opencode`는 100K+ 다운로드로 안정적으로 유지되고 있습니다.

---

## 1. 서비스 시작/중지

### 시작

```bash
cd /home/aa/vllm

# 전체 스택 (LLM + OpenCode + PaperClip)
docker compose up -d

# OpenCode만 시작 (LLM이 healthy 상태여야 함)
docker compose up -d opencode
```

### 중지

```bash
docker compose stop opencode

# 또는 전체 스택 중지
docker compose down
```

### 상태 확인

```bash
docker compose ps
docker compose logs -f opencode
```

---

## 2. 웹 UI 접속

브라우저에서 열기:

```
http://localhost:3000
```

> **포트 변경:** `.env` 파일에서 `OPENCODE_PORT` 값을 수정하면 됩니다.

---

## 3. 기본 사용법

### 채팅 인터페이스

웹 UI 접속 후 하단 입력창에 메시지를 입력하면 LLM이 응답합니다.
OpenCode는 단순 대화뿐 아니라 **AI 에이전트**로서 직접 파일을 읽고, 수정하고, 명령을 실행할 수 있습니다.

### 요청 예시

| 요청 유형 | 예시 |
|-----------|------|
| 코드 작성 | "Python으로 REST API 서버를 만들어줘" |
| 파일 편집 | "main.py의 버그를 찾아서 고쳐줘" |
| 코드 분석 | "이 프로젝트의 구조를 설명해줘" |
| 셸 명령 | "현재 디렉토리의 파일 목록을 보여줘" |
| 디버깅 | "이 에러 메시지를 분석해줘" |

### 모델 선택

- `Ctrl+O` — 모델 선택 다이얼로그 열기
- 기본 설정에서는 로컬 LLM (`gemma-4-31b-it`)이 `local` 프로바이더로 자동 연결됩니다.

---

## 4. AI 도구 (Tools)

OpenCode의 AI는 다음 도구들을 사용하여 실제 작업을 수행합니다:

### 파일 및 코드 도구

| 도구 | 설명 |
|------|------|
| `glob` | 패턴으로 파일 검색 |
| `grep` | 파일 내용 검색 (텍스트/정규식) |
| `ls` | 디렉토리 내용 목록 |
| `view` | 파일 내용 보기 |
| `write` | 파일 작성 |
| `edit` | 파일 편집 |
| `patch` | 파일에 diff 패치 적용 |
| `diagnostics` | LSP 진단 정보 확인 |

### 기타 도구

| 도구 | 설명 |
|------|------|
| `bash` | 셸 명령 실행 |
| `fetch` | URL에서 데이터 가져오기 |
| `agent` | 하위 에이전트로 서브태스크 실행 |

> **권한 시스템:** AI가 파일 수정이나 명령 실행 시 사용자에게 허가를 요청합니다.
> `a`로 허용, `A`로 세션 전체 허용, `d`로 거부할 수 있습니다.

---

## 5. 키보드 단축키

### 전역

| 단축키 | 기능 |
|--------|------|
| `Ctrl+C` | 종료 |
| `Ctrl+?` | 도움말 토글 |
| `Ctrl+L` | 로그 보기 |
| `Ctrl+A` | 세션 전환 |
| `Ctrl+K` | 명령 다이얼로그 열기 |
| `Ctrl+O` | 모델 선택 |
| `Esc` | 현재 다이얼로그 닫기 |

### 채팅

| 단축키 | 기능 |
|--------|------|
| `Ctrl+N` | 새 세션 생성 |
| `Ctrl+X` | 현재 생성 취소 |
| `i` | 에디터로 포커스 이동 |

### 에디터

| 단축키 | 기능 |
|--------|------|
| `Ctrl+S` | 메시지 전송 |
| `Enter` | 메시지 전송 (에디터 비포커스 시) |
| `Ctrl+E` | 외부 에디터 열기 |
| `Esc` | 에디터 포커스 해제 |

---

## 6. 세션 관리

OpenCode는 대화를 **세션** 단위로 관리합니다.

- `Ctrl+A` — 세션 목록 열기/전환
- `Ctrl+N` — 새 세션 시작
- 세션 다이얼로그에서 `↑`/`↓` 또는 `k`/`j`로 탐색, `Enter`로 선택

### Auto Compact

컨텍스트 윈도우가 95%에 도달하면 자동으로 대화를 요약하여 새 세션을 생성합니다.
이를 통해 긴 대화에서도 컨텍스트가 유실되지 않습니다.

---

## 7. 커스텀 명령

### 명령 만들기

마크다운 파일을 명령 디렉토리에 생성하면 커스텀 명령이 됩니다:

- **사용자 명령:** `~/.config/opencode/commands/` → `user:명령이름`
- **프로젝트 명령:** `<프로젝트>/.opencode/commands/` → `project:명령이름`

예시 (`~/.config/opencode/commands/review.md`):

```markdown
이 프로젝트의 코드를 리뷰해줘. 보안 취약점, 성능 문제, 코드 스타일을 확인해줘.
```

### 명령 사용

1. `Ctrl+K`로 명령 다이얼로그 열기
2. `user:review` 선택
3. `Enter`로 실행

### 내장 명령

| 명령 | 설명 |
|------|------|
| Initialize Project | `OpenCode.md` 메모리 파일 생성/업데이트 |
| Compact Session | 현재 세션을 수동으로 요약 |

---

## 8. 로컬 LLM 연동 설정

현재 Docker Compose 스택에서 OpenCode는 자동으로 로컬 LLM 서버에 연결됩니다:

| 환경변수 | 값 | 설명 |
|----------|-----|------|
| `OPENAI_API_BASE` | `http://llm:8000/v1` | LLM API 엔드포인트 |
| `OPENAI_API_KEY` | `sk-no-key-required` | 로컬 서버이므로 더미 키 |
| `LOCAL_ENDPOINT` | `http://llm:8000/v1` | 셀프호스팅 모델 엔드포인트 |

현재 LLM 서버는 슬롯당 **128K 컨텍스트**, q8_0 KV 캐시, reasoning 활성화로 최적화되어 있습니다.
자세한 내용은 [llama-server 최적화 가이드](llama-server-tuning.md)를 참조하세요.

별도의 API 키 없이 로컬 `gemma-4-31b-it` 모델을 사용합니다.

### 외부 프로바이더 추가 (선택)

외부 AI 프로바이더를 사용하려면 해당 API 키를 환경변수로 설정합니다:

```bash
# docker-compose.yml의 opencode 서비스에 환경변수 추가
environment:
  - ANTHROPIC_API_KEY=sk-ant-...    # Claude
  - OPENAI_API_KEY=sk-...           # OpenAI
  - GEMINI_API_KEY=...              # Google Gemini
```

---

## 9. MCP 서버 연동

OpenCode는 [MCP (Model Context Protocol)](https://modelcontextprotocol.io)을 지원하여 외부 도구를 연동할 수 있습니다.

설정 파일 (`/root/.config/opencode/opencode.jsonc`) 예시:

```json
{
  "mcpServers": {
    "example": {
      "type": "stdio",
      "command": "path/to/mcp-server",
      "args": []
    },
    "web-api": {
      "type": "sse",
      "url": "https://example.com/mcp"
    }
  }
}
```

---

## 10. 데이터 영속화

Docker 컨테이너 재시작 시 데이터를 유지하려면 다음 경로를 볼륨으로 마운트합니다:

| 경로 | 내용 |
|------|------|
| `/root/.config/opencode` | 설정 파일 (`opencode.jsonc`) |
| `/root/.local/share/opencode` | DB, 인증, 로그, 스토리지 |
| `/workspace` | 작업 디렉토리 (현재 마운트됨) |

> **현재 구성:** `./workspace`만 마운트되어 있습니다.
> 설정과 세션 데이터를 영속화하려면 `docker-compose.yml`에 볼륨을 추가하세요.

---

## 11. 인증 설정 (선택)

웹 UI에 비밀번호를 설정하려면:

```yaml
# docker-compose.yml의 opencode 서비스에 추가
environment:
  - OPENCODE_SERVER_USERNAME=opencode
  - OPENCODE_SERVER_PASSWORD=비밀번호
```

> **주의:** 현재는 인증 없이 열려 있습니다 (`OPENCODE_SERVER_PASSWORD is not set`).
> 외부 네트워크에 노출되는 경우 반드시 인증을 설정하세요.

---

## 12. 문제 해결

### 웹 UI에 접속할 수 없음

```bash
# 1. 컨테이너 상태 확인
docker compose ps

# 2. 로그 확인
docker compose logs --tail 50 opencode

# 3. LLM 서버 상태 확인 (OpenCode는 LLM healthy 후 시작)
curl http://localhost:8000/health
```

### LLM 응답이 없음

```bash
# LLM 서버 직접 테스트
curl http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"gemma-4-31b-it","messages":[{"role":"user","content":"hello"}],"max_tokens":32}'
```

### 세션/설정 초기화

```bash
# OpenCode 컨테이너 재생성 (데이터 초기화)
docker compose rm -f opencode
docker compose up -d opencode
```
