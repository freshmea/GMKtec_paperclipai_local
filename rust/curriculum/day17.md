# Day 17: gRPC와 Tonic을 이용한 고성능 RPC 통신 (gRPC with Tonic)

## 1. 학습 목표 (Objectives)

본 세션의 목표는 현대적인 마이크로서비스 아키텍처에서 표준으로 자리 잡은 **gRPC**를 이해하고, Rust의 대표적인 gRPC 라이브러리인 **Tonic**을 사용하여 고성능 통신 시스템을 구축하는 방법을 배우는 것입니다.

* **gRPC 프로토콜 이해:** HTTP/2 기반의 프로토콜 특성과 Protocol Buffers(protobuf)의 역할 파악.
* **Protocol Buffers 설계:** `.proto` 파일을 사용하여 서비스 인터페이스와 메시지 구조 정의.
* **Tonic 프레임워크 활용:** 정의된 `.proto` 파일을 기반으로 Rust 서버와 클라이언트 코드 생성 및 구현.
* **Unary vs Streaming RPC:** 단일 요청/응답 모델과 스트리밍(Server-side, Client-side, Bidirectional) 모델의 차이점과 활용 사례 학습.
* **에러 핸들링:** gRPC 표준 상태 코드(Status Code)를 이용한 에러 전달 방식 익히기.

---

## 2. 세션 일정 (Session Breakdown)

| 시간 | 주제 | 내용 |
| :--- | :--- | :--- |
| 00:00 - 00:40 | **gRPC & Protobuf 기초** | HTTP/2, Protobuf 직렬화, IDL(Interface Definition Language) 개념 |
| 00:40 - 01:30 | **Tonic 환경 설정** | `prost`, `tonic-build`를 이용한 코드 생성 자동화 및 환경 구축 |
| 01:30 - 02:30 | **Unary RPC 구현** | 기본 요청/응답 서비스 구현 및 클라이언트 테스트 |
| 02:30 - 03:30 | **Streaming RPC 구현** | Server-to-Client, Client-to-Server 스트리밍 구현 |
| 03:30 - 05:00 | **Hands-on Lab** | 실시간 데이터 스트리밍 서비스(예: 센서 데이터 모니터링) 구축 |

---

## 3. 핵심 개념 (Key Concepts)

### 3.1 Protocol Buffers (protobuf)
데이터를 직렬화하는 효율적인 바이너리 포맷입니다. 언어 중립적이며, `.proto` 파일 하나로 다양한 언어의 코드를 생성할 수 있어 서비스 간 계약(Contract) 역할을 수행합니다.

### 3.2 Tonic 프레임워크
Tonic은 `hyper`, `tokio`, `prost`를 기반으로 구축된 gRPC 구현체입니다.
* **Code Generation:** `build.rs`를 통해 컴파일 시점에 `.proto` 파일을 읽어 Rust 트레이트와 구조체를 자동으로 생성합니다.
* **High Performance:** 비동기 런타임인 `tokio`를 활용하여 매우 높은 처리량을 제공합니다.

### 3.3 RPC 통신 모델
1.  **Unary:** 가장 일반적인 방식. 하나의 요청에 하나의 응답.
2.  **Server Streaming:** 클라이언트가 요청을 보내면, 서버가 여러 개의 응답을 연속적으로 보냄 (예: 실시간 뉴스 피드).
3.  **Client Streaming:** 클라이언트가 여러 개의 데이터를 보내면, 서버가 마지막에 하나의 응답을 보냄 (예: 대용량 파일 업로드).
4.  **Bidirectional Streaming:** 클라이언트와 서버가 서로 독립적으로 메시지를 주고받음 (예: 채팅 서비스).

---

## 4. 데모 포인트 (Demonstration Points)

1.  **Proto Definition:** `helloworld.proto` 파일을 작성하고 메시지 구조 정의하기.
2.  **Build Script:** `build.rs`를 사용하여 컴파일 시점에 Rust 코드가 생성되는 과정 보여주기.
3.  **Unary Service:** 간단한 `SayHello` 서비스 구현.
4.  **Streaming Demo:** 서버가 클라이언트에게 1초마다 숫자를 보내는 스트리밍 서비스 구현.

---

## 5. 실습 과제 (Lab/Assignment)

### 과제명: 실시간 센서 데이터 수집 서버 구현

**요구사항:**
1.  `SensorService`를 정의하는 `.proto` 파일을 작성합니다.
    * `SensorReading` 메시지: `sensor_id` (string), `value` (double), `timestamp` (int64).
    * `StreamSensorData` 메서드: 클라이언트가 `sensor_id`를 보내면, 서버가 해당 센서의 가상 데이터를 5초 동안 스트리밍합니다.
2.  **Tonic**을 사용하여 서버를 구현합니다.
    * 가상의 데이터를 생성하기 위해 `tokio::time::sleep`을 사용합니다.
3.  **Client**를 구현하여 서버에 연결하고 스트리밍되는 데이터를 출력합니다.

**검증 방법:**
* 클라이언트 실행 시 서버로부터 연속적인 데이터 패킷이 터미널에 출력되는지 확인합니다.
* 스트리밍이 정해진 횟수나 시간 후에 정상적으로 종료되는지 확인합니다.
