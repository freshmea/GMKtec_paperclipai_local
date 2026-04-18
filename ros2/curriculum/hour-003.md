# [Hour 003] ROS2 기초: Service (Client/Server) 통신

## 🎯 학습 목표
- ROS2의 요청-응답(Request-Response) 모델인 Service 통신의 개념을 이해합니다.
- Python을 사용하여 요청을 보내는 Client 노드와 요청을 처리하는 Server 노드를 작성할 수 있습니다.
- Topic(비동기 스트림)과 Service(동기적 요청-응답)의 차이점을 이해합니다.

## 📖 학습 내용
1. **Service 통신 모델 이해**
   - Client와 Server 간의 1:1 요청-응답 메커니즘
   - 동기(Synchronous) vs 비동기(Asynchronous) 서비스 호출
2. **Service Server 구현**
   - `create_service`를 이용한 서비스 서버 생성
   - 서비스 콜백(Service Callback) 함수를 통한 요청 데이터 처리 및 응답 반환
3. **Service Client 구현**
   - `create_client`를 이용한 클라이언트 생성
   - 서비스 서버가 활성화될 때까지 대기하는 과정 (`wait_for_service`)
   - `call_async`를 이용한 비동기 서비스 요청 및 결과 처리
4. **CLI를 이용한 검증**
   - `ros2 service list`: 현재 활성화된 서비스 목록 확인
   - `ros2 service type <service_name>`: 서비스 타입 확인
   - `ros2 service call <service_name> <service_type> <arguments>`: 명령행에서 서비스 호출 테스트

## 🔗 공식 문서 및 참고 자료
- [Writing a simple service and client (Python)](https://docs.ros.org/en/rolling/Tutorials/Beginner-Client-Libraries/Writing-A-Simple-Service-And-Client-Python.html)
- [ROS 2 Service Concepts](https://docs.ros.org/en/jazzy/Concepts/Intermediate/Design.html#services)

---
*본 문서는 교육 목적으로 작성되었습니다.*
