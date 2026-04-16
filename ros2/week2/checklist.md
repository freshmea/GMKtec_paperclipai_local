# 2주차 체크리스트: 통신 아키텍처
 
## ✅ 완료 여부 확인
 
### 1. Topic (Publish/Subscribe)
- [ ] Publisher 노드가 메시지를 지속적으로 발행하는가?
- [ ] Subscriber 노드가 발행된 메시지를 정상적으로 수신하고 출력하는가?
- [ ] `ros2 topic echo` 명령어로 데이터 흐름을 확인할 수 있는가?
 
### 2. Service (Request/Response)
- [ ] Service Server가 요청을 기다리는 상태로 실행 중인가?
- [ ] Service Client가 요청을 보냈을 때 Server로부터 응답을 받는가?
- [ ] `ros2 service call` 명령어로 수동 테스트가 가능한가?
 
### 3. Action (Goal/Feedback/Result)
- [ ] Action Server가 Goal을 수락하는가?
- [ ] 실행 과정 중 Feedback 메시지가 클라이언트에 전달되는가?
- [ ] 작업 완료 후 최종 Result가 정상적으로 반환되는가?
 
### 4. 통신 모니터링 및 QoS
- [ ] `ros2 topic list`, `ros2 service list`, `ros2 action list` 명령어를 숙지하고 활용하였는가?
- [ ] Lyrical Luth의 새로운 QoS 최적화 옵션을 이해하고 적용하였는가?
 
## 💡 문제 해결 (Troubleshooting)
* **메시지 타입 불일치:** Publisher와 Subscriber의 메시지 타입이 동일한지 확인하세요.
* **네트워크/Domain ID:** 동일한 `ROS_DOMAIN_ID`를 사용하고 있는지 확인하세요.
* **QoS 불일치:** 데이터가 보이지만 수신되지 않는다면 QoS 설정(Reliability 등)이 일치하는지 확인하세요.
