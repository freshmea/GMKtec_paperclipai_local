# 1주차: ROS2 환경 설정 및 Workspace 구성

## 🎯 학습 목표
* ROS2 Lyrical Luth 환경을 구축하고 개발 준비를 마칩니다.
* `colcon` 빌드 도구를 사용하여 ROS2 워크스페이스를 생성하고 관리합니다.
* 기본적인 ROS2 패키지 구조를 이해하고 빌드 프로세스를 익힙니다.

## 📖 이론 (Theory)

### 1. ROS2 Lyrical Luth 소개
ROS2 Lyrical Luth는 Ubuntu 26.04 (Resolute)를 기본 OS로 지원하는 차기 버전입니다. C++20 및 최신 Python 런타임(3.12+) 지원을 통해 더욱 현대적인 로보틱스 소프트웨어 개발 환경을 제공합니다.

### 2. 워크스페이스(Workspace)란?
ROS2에서 워크스페이스는 소스 코드를 모아두고 빌드하여 실행 가능한 바이너리를 만드는 작업 공간입니다. 일반적으로 `dev_ws` 또는 `ros2_ws`라는 이름을 사용합니다.

### 3. 주요 도구
* **colcon:** ROS2 패키지를 빌드하기 위한 표준 빌드 도구입니다.
* **rosdep:** 패키지 의존성을 설치하고 관리하는 도구입니다.

## 🔗 공식 출처
* [ROS 2 Installation Guide](https://docs.ros.org/en/jazzy/Installation.html)
* [Building ROS 2 Workspace](https://docs.ros.org/en/jazzy/Tutorials/Beginner-Client-Libraries/Beginner-Client-Libraries-Using-A-Single-Node.html)
