# 2주차: 통신 아키텍처 (Topic, Service, Action)

## 🎯 학습 목표
* ROS2의 핵심 통신 메커니즘인 Topic, Service, Action의 차이점을 이해합니다.
* 각 통신 방식이 적합한 사용 사례(Use Case)를 구분할 수 있습니다.
* Lyrical Luth에서 강화된 QoS(Quality of Service) 설정의 최신 트렌드를 파악합니다.

## 📖 이론 (Theory)

### 1. ROS2 통신 방식 비교

| 방식 | 통신 모델 | 특징 | 적합한 사례 |
| :--- | :--- | :--- | :--- |
| **Topic** | Publish/Subscribe (비동기) | 연속적인 데이터 흐름, 일대다(1:N) 통신 가능 | 센서 데이터(LiDAR, Camera), 상태 정보 업데이트 |
| **Service** | Request/Response (동기/비동기) | 요청에 대한 응답 필요, 일대일(1:1) 통신 | 설정 변경, 계산 요청, 일회성 작업 실행 |
| **Action** | Goal/Feedback/Result (비동기) | 작업 진행 상황(Feedback)을 실시간으로 확인 가능 | 로봇 이동, 매니퓰레이터 동작 등 시간이 걸리는 작업 |

### 2. QoS (Quality of Service)
데이터 전송의 신뢰성, 지연 시간, 대역폭 등을 제어하기 위한 설정입니다. Lyrical Luth에서는 DDS 레이어의 최적화를 통해 더욱 세밀한 제어가 가능합니다.
* **Reliability:** 데이터가 반드시 전달되어야 하는지(Reliable) 여부
* **Durability:** 새로운 구독자가 나타났을 때 과거 데이터를 전달할지 여부
* **History:** 보관할 메시지의 개수 설정

## 🔗 공식 출처
* [ROS 2 Communication Middleware](https://docs.ros.org/en/jazzy/Concepts/Intermediate/About-DDS.html)
* [Topic, Service, and Action](https://docs.ros.org/en/jazzy/Tutorials/Beginner-Client-Libraries/Beginner-Client-Libraries-Using-A-Single-Node.html)
