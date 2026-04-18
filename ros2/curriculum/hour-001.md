# [Hour 001] ROS2 환경 설정 및 워크스페이스 기초

## 🎯 학습 목표
- Ubuntu 26.04 환경에서 ROS2 Lyrical Luth 개발 환경의 구조를 이해합니다.
- `colcon` 빌드 도구를 사용하여 ROS2 워크스페이스를 생성하고 패키지를 빌드할 수 있습니다.
- ROS2 환경 변수(`setup.bash`)의 역할을 이해합니다.

## 📖 학습 내용
1. **ROS2 설치 확인 및 환경 변수 설정**
   - `source /opt/ros/rolling/setup.bash`를 통한 환경 로드
   - `printenv | grep ROS`를 통한 설치 확인
2. **Workspace 생성 (colcon workspace)**
   - `mkdir -p ~/ros2_ws/src`
   - `colcon build` 명령어를 통한 워크스페이스 빌드 과정
3. **패키지 생성 기초**
   - `ros2 pkg create --build-type ament_python my_package`
   - 패키지 구조(setup.py, package.xml) 이해

## 🔗 공식 문서 및 참고 자료
- [ROS 2 Installation Guide](https://docs.ros.org/en/rolling/Installation.html)
- [Building ROS 2 Packages](https://docs.ros.org/en/rolling/Tutorials/Beginner-Client-Libraries/Building-ROS2-Packages.html)

---
*본 문서는 교육 목적으로 작성되었습니다.*
