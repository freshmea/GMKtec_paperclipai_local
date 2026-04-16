# 4주차 실습: Rviz2 및 Nav2/SLAM 개요
 
 ## 🎯 실습 목표
 * Rviz2를 실행하고 센서 데이터(LaserScan) 및 TF를 시각화합니다.
 * 시뮬레이션 환경(예: TurtleBot3 Gazebo)을 구축하고 구동합니다.
 * SLAM Toolbox를 사용하여 가상 환경에서 지도를 생성하는 과정을 이해합니다.
 * Nav2 스택의 동작 흐름을 시뮬레이션 환경을 통해 관찰합니다.
 * Lyrical Luth의 새로운 이미지/포인트클라우드 전송 기능을 활용한 시각화 환경을 확인합니다.
 
 ## 🛠️ 실습 단계
 
 ### 1. 시뮬레이션 환경 준비 (Gazebo)
 실습을 위해 가상 로봇 환경을 먼저 실행해야 합니다.
 1. TurtleBot3 시뮬레이션 패키지를 설치합니다.
 2. Gazebo 시뮬레이터를 실행합니다:
    ```bash
    export TURTLEBOT3_MODEL=burger
    ros2 launch turtlebot3_gazebo turtlebot3_world.launch.py
    ```
 
 ### 2. Rviz2 시각화 실습
 1. 새로운 터미널에서 `rviz2`를 실행합니다.
 2. 좌측 하단의 **Add** 버튼을 눌러 다음 항목을 추가합니다:
    * `RobotModel`: 로봇의 외형 확인
    * `TF`: 좌표계 간의 관계 확인
    * `LaserScan`: LiDAR 데이터 확인 (Topic: `/scan`)
    * `Map`: 생성된 지도 확인 (Topic: `/map`)
 3. **Fixed Frame**을 `map` 또는 `odom`으로 설정하여 데이터가 올바르게 보이는지 확인합니다.
 4. (Lyrical Luth 특화) `image_transport`를 사용하여 카메라 스트림이 안정적으로 시각화되는지 확인합니다.
 
 ### 3. SLAM Toolbox를 이용한 지도 작성 (개요)
 1. SLAM Toolbox 노드를 실행합니다.
 2. 로봇을 조이스틱이나 키보드로 천천히 조종하여 환경을 탐색합니다.
 3. Rviz2에서 지도가 점진적으로 채워지는 것을 관찰합니다.
 4. 지도가 완성되면 `map_saver`를 사용하여 지도를 저장합니다.
 
 ### 4. Nav2 자율 주행 흐름 관찰
 1. Nav2 스택을 실행합니다.
 2. Rviz2 상단의 **"2D Pose Estimate"**를 사용하여 로봇의 현재 위치를 지정합니다.
 3. **"Nav2 Goal"** 버튼을 눌러 목적지를 클릭합니다.
 4. 로봇이 경로(Global Path)를 생성하고, 장애물을 회피하며(Local Planner) 이동하는지 확인합니다.
 
 ## 🔍 확인 사항
 * **Costmap 확인:** 장애물 주변에 생성된 Inflation Layer(팽창 영역)를 확인하세요.
 * **Path 확인:** 로봇이 계획한 경로가 장애물을 통과하지 않는지 확인하세요.
 * **Localization 확인:** 로봇이 이동할 때 `map` 좌표계 상에서 위치가 안정적으로 유지되는지 확인하세요.
 * **Transport 확인:** Lyrical Luth의 새로운 전송 레이어를 통해 고대역폭 데이터(Image, PointCloud)가 끊김 없이 전달되는지 확인하세요.
 * **[Pro Tip] Lifecycle Node:** Nav2의 각 노드(Planner, Controller 등)가 `unconfigured` -> `inactive` -> `active` 상태로 전환되는 과정을 확인하여 Lyrical Luth의 Lifecycle 지원을 이해해 보세요.
