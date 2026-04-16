# 3주차 실습: Launch 파일 및 TF2 좌표 변환
 
 ## 🎯 실습 목표
 * Python 기반 Launch 파일을 작성하여 여러 노드를 동시에 실행합니다.
 * Launch 파일을 통해 노드에 파라미터를 전달하는 방법을 익힙니다.
 * `static_transform_publisher`를 사용하여 정적 좌표 변환을 설정합니다.
 * TF2 브로드캐스터를 구현하여 동적 좌표 변환을 생성합니다.
 * Lyrical Luth의 컴포넌트 로딩 방식을 활용하여 노드를 실행해 봅니다.
 
 ## 🛠️ 실습 단계
 
 ### 1. Launch 파일 작성 실습
 
 #### 1.1 Python Launch 파일 작성
 1. `my_first_package` 내에 `launch/` 디렉토리를 생성합니다.
 2. `talker_listener_launch.py` 파일을 작성하여 `talker`와 `listener` 노드를 동시에 실행하도록 설정합니다.
 3. `setup.py`의 `data_files`에 launch 파일을 포함하도록 수정합니다.
 4. Launch 파일 내에서 `DeclareLaunchArgument`를 사용하여 노드에 파라미터를 전달하는 코드를 포함합니다.
 5. 빌드 후 실행합니다:
    ```bash
    ros2 launch my_first_package talker_listener_launch.py my_param:=value
    ```
 
 ### 2. 정적 좌표 변환 (Static Transform)
 터미널에서 명령어를 통해 간단한 정적 변환을 생성해 봅니다.
 ```bash
 # base_link에서 sensor_link까지 x축으로 0.5m 떨어진 정적 변환 생성
 ros2 run tf2_ros static_transform_publisher --x 0.5 --y 0 --z 0 --yaw 0 --pitch 0 --roll 0 --frame-id base_link --child-frame-id sensor_link
 ```
 
 ### 3. 동적 좌표 변환 (Dynamic Transform) 구현
 1. `tf2_broadcaster_node.py`를 작성합니다.
 2. 루프 내에서 `TransformBroadcaster`를 사용하여 `base_link`와 `odom` 사이의 관계를 지속적으로 발행합니다.
 3. `ros2 run tf2_ros tf2_monitor` 또는 `rviz2`를 통해 변환이 정상적으로 흐르는지 확인합니다.
 
 ### 4. 컴포넌트 기반 실행 (Lyrical Luth 특화)
 Lyrical Luth에서는 노드를 컴포넌트로 만들어 하나의 프로세스에서 실행하는 것이 효율적입니다.
 
 #### 4.1 C++ 컴포넌트 작성
 1. `my_cpp_package`를 `rclcpp_components`를 지원하도록 수정합니다.
 2. 노드 클래스를 `rclcpp::Node`를 상속받는 컴포넌트 형태로 작성합니다.
 3. `CMakeLists.txt`에서 `rclcpp_components_register_nodes`를 사용하여 컴포넌트를 등록합니다.
 
 #### 4.2 컴포넌트 로딩 및 실행
 1. `component_container`를 실행합니다.
 2. `ros2 component load` 명령어를 통해 작성한 컴포넌트를 동적으로 로드해 봅니다.
 3. 또는, Python Launch 파일을 사용하여 `ComposableNodeContainer`를 통해 여러 컴포넌트를 한 번에 실행하는 파일을 작성합니다.
 
 ## 🔍 확인 사항
 * `ros2 run tf2_tools view_frames` 명령어를 사용하여 생성된 TF 트리를 PDF로 확인해 보세요.
 * `rviz2`에서 `TF` 디스플레이를 추가하여 좌표축이 올바르게 표시되는지 확인하세요.
 * Launch 파일을 통해 전달한 파라미터가 노드에서 정상적으로 읽히는지 확인하세요.
