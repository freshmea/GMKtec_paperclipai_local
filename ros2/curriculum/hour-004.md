# [Hour 004] ROS2 기초: Action 통신

## 🎯 학습 목표
- ROS2의 가장 복잡하고 강력한 통신 방식인 Action의 개념을 이해합니다.
- Goal, Feedback, Result로 이어지는 Action의 생명주기를 이해합니다.
- Python을 사용하여 Action Server와 Action Client를 작성할 수 있습니다.
- Topic(단방향 스트림) 및 Service(단일 요청-응답)와 Action의 차이점을 명확히 구분합니다.

## 📖 학습 내용
1. **Action 통신 모델 이해**
   - 장기 실행 작업(Long-running tasks)에 최적화된 모델
   - 비동기적 작업 수행과 중간 진행 상태(Feedback) 보고의 중요성
2. **Action Server 구현**
   - `create_action_server`를 이용한 서버 생성
   - `execute_callback`을 통한 실제 작업 수행 로직 구현
   - 작업 중간 단계에서 `feedback`을 발행하는 방법
   - 작업 완료 시 `result`를 반환하는 과정
3. **Action Client 구현**
   - `create_action_client`를 이용한 클라이언트 생성
   - Goal 요청 및 서버 응답 대기
   - `feedback_callback`을 통한 중간 진행 상태 모니터링
   - 최종 결과(Result) 수신 및 처리
4. **CLI를 이용한 검증**
   - `ros2 action list`: 현재 활성화된 액션 목록 확인
   - `ros2 action send_goal <action_name> <action_type> <arguments>`: 명령행에서 액션 호출 테스트

## 🔗 공식 문서 및 참고 자료
- [Writing a simple action server and client (Python)](https://docs.ros.org/en/rolling/Tutorials/Beginner-Client-Libraries/Writing-A-Simple-Py-Action-Server-And-Client.html)
- [ROS 2 Action Concepts](https://docs.ros.org/en/jazzy/Concepts/Intermediate/Design.html#actions)

---
*본 문서는 교육 목적으로 작성되었습니다.*
