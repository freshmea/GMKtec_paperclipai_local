# [Hour 002] ROS2 기초: Topic (Pub/Sub) 통신

## 🎯 학습 목표
- ROS2의 가장 기본적인 통신 방식인 Topic (Publisher/Subscriber)의 개념을 이해합니다.
- Python을 사용하여 간단한 메시지를 발행(Publish)하고 구독(Subscribe)하는 노드를 작성할 수 있습니다.
- `ros2 topic` CLI 도구를 사용하여 통신 상태를 확인하고 디버깅할 수 있습니다.

## 📖 학습 내용
1. **Topic 통신 모델 이해**
   - 비동기식 데이터 스트림 개념
   - 메시지 타입(Message Type)의 중요성
2. **Publisher 구현**
   - `rclpy.node.Node` 상속을 통한 노드 정의
   - `create_publisher`를 이용한 토픽 발행 설정
   - 타이머를 이용한 주기적 메시지 송신
3. **Subscriber 구현**
   - `create_subscription`을 이용한 토픽 수신 설정
   - 콜백 함수(Callback Function)를 통한 데이터 처리
4. **CLI를 이용한 검증**
   - `ros2 topic list`: 현재 활성화된 토픽 확인
   - `ros2 topic echo <topic_name>`: 실시간 메시지 데이터 확인
   - `ros2 topic info <topic_name>`: 토픽 상세 정보 확인

## 🔗 공식 문서 및 참고 자료
- [Writing a simple publisher and subscriber (Python)](https://docs.ros.org/en/rolling/Tutorials/Beginner-Client-Libraries/Writing-A-Simple-Py-Publisher-And-Subscriber.html)
- [ROS 2 Topic Concepts](https://docs.ros.org/en/jazzy/Concepts/Intermediate/Design.html#topics)

---
*본 문서는 교육 목적으로 작성되었습니다.*
