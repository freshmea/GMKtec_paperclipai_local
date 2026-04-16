# 4주차: Rviz2 및 Nav2/SLAM 개요

## 🎯 학습 목표
* ROS2의 강력한 시각화 도구인 Rviz2의 사용법을 익힙니다.
* SLAM(Simultaneous Localization and Mapping)의 개념과 ROS2 SLAM Toolbox의 역할을 이해합니다.
* Nav2(Navigation 2) 스택의 구성 요소와 자율 주행의 흐름을 파악합니다.
* Lyrical Luth에서 강화된 시각화 및 로그 데이터 처리 방식을 이해합니다.

## 📖 이론 (Theory)

### 1. Rviz2 (ROS Visualization)
Rviz2는 로봇의 상태를 시각적으로 확인하기 위한 도구입니다.
* **Display Types:** LaserScan, PointCloud2, TF, RobotModel, Map, Odometry 등을 시각화할 수 있습니다.
* **Fixed Frame:** 시각화의 기준이 되는 좌표계를 설정하는 것이 매우 중요합니다 (예: `map` 또는 `odom`).
* **Lyrical Luth Feature:** 새로운 `image_transport` 및 `point_cloud_transport`의 Lifecycle 지원을 통해 데이터 스트림의 안정적인 시각화가 가능합니다.

### 2. SLAM (Simultaneous Localization and Mapping)
로봇이 미지의 환경에서 자신의 위치를 파악(Localization)함과 동시에 지도를 작성(Mapping)하는 기술입니다.
* **SLAM Toolbox:** ROS2에서 가장 널리 사용되는 SLAM 패키지 중 하나로, 2D LiDAR 데이터를 기반으로 지도를 생성합니다.
* **Process:** LiDAR 데이터 수신 $\rightarrow$ 특징점 추출 $\rightarrow$ 위치 추정 $\rightarrow$ 지도 업데이트.

### 3. Nav2 (Navigation 2)
생성된 지도를 바탕으로 로봇이 목표 지점까지 장애물을 피하며 이동하게 하는 스택입니다.
* **Planner:** 출발지에서 목적지까지의 경로(Global Path)를 계획합니다.
* **Controller:** 생성된 경로를 따라가기 위해 속도 명령(cmd_vel)을 생성합니다 (Local Planner).
* **Costmap:** 장애물 정보를 바탕으로 주행 가능 영역을 계산합니다.

## 🔗 공식 출처
* [Rviz2 Documentation](https://docs.ros.org/en/jazzy/Tutorials/Beginner-Client-Libraries/Beginner-Client-Libraries-Using-A-Single-Node.html)
* [Nav2 Documentation](https://navigation.ros.org/)
* [SLAM Toolbox GitHub](https://github.com/SteveMacenski/slam_toolbox)
