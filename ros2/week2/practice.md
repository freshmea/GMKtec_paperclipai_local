# 2주차 실습: Topic, Service, Action 통신 구현
 
 ## 🎯 실습 목표
 * Python(`rclpy`) 및 C++(`rclcpp`)를 사용하여 Topic, Service, Action 노드를 작성하고 통신을 확인합니다.
 * Publisher와 Subscriber를 구현하여 데이터를 주고받습니다.
 * Service Client와 Server를 구현하여 요청과 응답을 처리합니다.
 * Action Client와 Server를 구현하여 목표 전달 및 피드백을 확인합니다.
 * 커스텀 인터페이스(msg/srv/action) 생성 방법을 익힙니다.
 
 ## 🛠️ 실습 단계
 
 ### 1. Topic 실습: Publisher & Subscriber
 
 #### 1.1 Python 구현
 1. `my_first_package` 내에 `talker.py`와 `listener.py`를 작성합니다.
 2. `setup.py`에 entry points를 등록합니다.
 3. `colcon build` 후 두 노드를 각각 실행합니다.
    ```bash
    ros2 run my_first_package talker
    ros2 run my_first_package listener
    ```
 
 #### 1.2 C++ 구현 (rclcpp)
 1. `my_cpp_package` 내에 `cpp_talker.cpp`와 `cpp_listener.cpp`를 작성합니다.
 2. `CMakeLists.txt`에 실행 파일 및 의존성을 등록합니다.
 3. `colcon build` 후 실행합니다.
 
 ### 2. 커스텀 인터페이스 생성 (Custom Interfaces)
 통신을 위해 사용자 정의 메시지 타입을 만들어 봅니다.
 
 ```bash
 # 인터페이스 전용 패키지 생성
 ros2 pkg create --build-type ament_cmake my_robot_interfaces --dependencies rosidl_default_generators
 ```
 * `msg/`, `srv/`, `action/` 디렉토리를 생성하고 `.msg`, `.srv`, `.action` 파일을 작성한 뒤 `CMakeLists.txt`와 `package.xml`을 설정합니다.
 
 ### 3. Service 실습: Request & Response
 1. `my_robot_interfaces`를 사용하여 커스텀 서비스 타입을 정의합니다.
 2. `service_server.py`와 `service_client.py`를 작성합니다.
 3. 서버를 실행한 상태에서 클라이언트를 통해 요청을 보냅니다.
 
 ### 4. Action 실습: Goal, Feedback, Result
 1. `action_server.py`와 `action_client.py`를 작성합니다.
 2. 서버는 목표를 받으면 일정 시간 동안 피드백을 전송하고 결과를 반환하도록 구현합니다.
 3. 클라이언트는 피드백을 출력하며 최종 결과를 확인합니다.
 
 ## 🔍 확인 사항
 * `ros2 topic list`, `ros2 service list`, `ros2 action list` 명령어를 통해 생성된 통신 인터페이스를 확인하세요.
 * `ros2 topic echo <topic_name>`으로 실제 데이터가 흐르는지 확인하세요.
 * **QoS 설정 확인:** Lyrical Luth 환경에서 `Reliability` 설정을 `Best Effort`로 변경했을 때 통신이 어떻게 달라지는지 확인해 보세요.
 
 ## 💡 문제 해결 (Troubleshooting)
 * **메시지 타입 불일치:** Publisher와 Subscriber의 메시지 타입이 동일한지 확인하세요.
 * **네트워크/Domain ID:** 동일한 `ROS_DOMAIN_ID`를 사용하고 있는지 확인하세요.
 * **QoS 불일치:** 데이터가 보이지만 수신되지 않는다면 QoS 설정(Reliability 등)이 일치하는지 확인하세요.
