# 1주차 실습: 환경 설정 및 Workspace 구성

 ## 🎯 실습 목표
 * ROS2 Lyrical Luth 환경을 설치하고 기본 설정을 완료합니다.
 * `colcon`을 사용하여 ROS2 워크스페이스를 생성하고 첫 번째 패키지(Python & C++)를 빌드합니다.
 * `rosdep`을 사용하여 패키지 의존성을 관리하는 방법을 익힙니다.

 ## 🛠️ 실습 단계

 ### 1. ROS2 Lyrical Luth 설치
 먼저 공식 가이드에 따라 Ubuntu 26.04에 ROS2 Lyrical Luth를 설치합니다.
 (설치가 완료되었다고 가정합니다.)

 ### 2. 워크스페이스 생성 및 구성
 터미널을 열고 다음 명령어를 순서대로 입력하세요.

 ```bash
 # 워크스페이스 디렉토리 생성
 mkdir -p ~/ros2_ws/src
 cd ~/ros2_ws/

 # 워크스페이스 빌드
 colcon build
 ```

 ### 2.5. 의존성 설치 (rosdep)
 패키지를 생성하기 전에 필요한 의존성들이 모두 설치되어 있는지 확인해야 합니다.

 ```bash
 # 의존성 자동 설치
 sudo apt update
 rosdep update
 rosdep install -i --from-path src --rosdistro lyrical -y
 ```

 ### 3. 환경 설정 (Sourcing)
 빌드 후에는 반드시 ROS2 환경을 로드해야 합니다.

 ```bash
 source /opt/ros/lyrical/setup.bash
 source install/setup.bash
 ```

 ### 4. 패키지 생성 실습

 #### 4.1 Python 패키지 생성 (ament_python)
 ```bash
 cd ~/ros2_ws/src
 ros2 pkg create --build-type ament_python my_py_package --dependencies rclpy
 ```

 #### 4.2 C++ 패키지 생성 (ament_cmake)
 Lyrical Luth의 최신 기능을 활용하기 위해 C++20 기반 패키지도 생성해 봅니다.
 ```bash
 ros2 pkg create --build-type ament_cmake my_cpp_package --dependencies rclcpp
 ```

 ### 5. 패키지 빌드 및 확인

 ```bash
 cd ~/ros2_ws/
 # 전체 빌드 또는 특정 패키지만 선택 빌드
 colcon build --packages-select my_py_package my_cpp_package
 source install/setup.bash
 ```

 ## ⚠️ 주의 사항
 * `source` 명령어를 실행하지 않으면 ROS2 명령어를 인식하지 못할 수 있습니다.
 * 새로운 터미널을 열 때마다 `source /opt/ros/lyrical/setup.bash`를 실행해야 합니다.
 * `rosdep` 실행 시 에러가 발생한다면, 네트워크 연결 상태나 `rosdep update` 여부를 확인하세요.
