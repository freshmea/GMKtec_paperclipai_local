# [Hour 005] ROS2 기초: Launch 파일을 이용한 시스템 통합

## 🎯 학습 목표
- ROS2의 시스템 통합 도구인 Launch 파일의 개념과 필요성을 이해합니다.
- Python 기반의 Launch 파일을 작성하여 여러 노드를 동시에 실행하고 파라미터를 설정할 수 있습니다.
- Launch 파일을 통해 복잡한 노드 실행 환경을 자동화하고 관리하는 방법을 습득합니다.

## 📖 학습 내용
1. **Launch 파일의 필요성**
   - 여러 노드를 순차적/동시 실행할 때의 번거로움 해결
   - 노드별 파라미터, 리매핑(Remapping), 네임스페이스(Namespace) 설정의 중앙화
2. **Python 기반 Launch 파일 작성**
   - `launch` 라이브러리 사용법
   - `Node` 액션을 이용한 노드 정의 및 실행
   - `DeclareLaunchArgument`를 이용한 실행 시 인자 전달
3. **실습: Publisher와 Subscriber 동시 실행**
   - 작성한 `minimal_publisher.py`와 `minimal_subscriber.py`를 하나의 Launch 파일로 묶기
   - `LaunchDescription`을 활용한 실행 구성
4. **CLI를 이용한 실행**
   - `ros2 launch <package_name> <launch_file_name>` 명령어 사용법

## 🔗 공식 문서 및 참고 자료
- [Writing launch files (Python)](https://docs.ros.org/en/rolling/Tutorials/Beginner-Tools/Launch/Launch-Configure.html)
- [ROS 2 Launch Concepts](https://docs.ros.org/en/jazzy/Concepts/Intermediate/Design.html#launch-files)

---
*본 문서는 교육 목적으로 작성되었습니다.*
