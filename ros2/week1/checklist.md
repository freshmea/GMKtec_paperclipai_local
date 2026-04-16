# 1주차 체크리스트: 환경 설정 및 Workspace 구성

## ✅ 완료 여부 확인

### 1. ROS2 설치 및 환경 확인
- [ ] `ros2 --version` 명령어가 정상적으로 작동하는가?
- [ ] `/opt/ros/lyrical/setup.bash` 파일이 존재하는가?

### 2. 워크스페이스 및 빌드
- [ ] `~/ros2_ws/src` 디렉토리가 생성되었는가?
- [ ] `colcon build` 명령어가 오류 없이 완료되었는가?
- [ ] 빌드 후 `build/`, `install/`, `log/` 디렉토리가 생성되었는가?

### 3. 환경 설정 (Sourcing)
- [ ] `source install/setup.bash`를 실행한 후 `ros2 pkg list` 명령어가 작동하는가?
- [ ] 새로운 터미널에서 ROS2 명령어를 사용할 수 있도록 설정하였는가?

### 4. 패키지 생성 및 빌드
- [ ] `my_first_package`가 `src` 디렉토리 안에 생성되었는가?
- [ ] `colcon build --packages-select my_first_package` 명령어가 성공하였는가?

## 💡 문제 해결 (Troubleshooting)
* **명령어를 찾을 수 없음 (command not found):** `source /opt/ros/lyrical/setup.bash`를 실행했는지 확인하세요.
* **빌드 오류:** `rosdep install`을 통해 의존성이 모두 설치되었는지 확인하세요.
