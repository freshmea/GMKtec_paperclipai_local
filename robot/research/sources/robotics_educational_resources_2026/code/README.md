# 로보틱스 교육용 코드 예제

이 디렉터리는 로보틱스 교육 목적으로 선별된 예제 소스 코드를 담고 있습니다.

## 구성

### ROS 2 예제
- `ros2_examples/minimal_publisher.py`: 토픽에 "Hello World" 문자열을 발행하는 기본 ROS 2 Python 노드
- `ros2_examples/subscribers/minimal_subscriber.py`: 토픽을 구독하고 수신 메시지를 출력하는 기본 ROS 2 Python 노드

### OpenCV 예제
- `opencv_examples/basic_camera_grayscale.py`: 카메라 영상을 캡처해 OpenCV로 그레이스케일 화면을 표시하는 스크립트
- `opencv_examples/detection/face_detection.py`: Haar Cascade를 사용해 실시간 비디오 스트림에서 얼굴을 검출하는 스크립트

### Python 로보틱스 예제
- `python_robotics_examples/basic_kinematics.py`: 2자유도 평면 로봇 팔의 순기구학을 단순하게 구현한 예제

## 사용 방법
다음 라이브러리가 설치되어 있어야 합니다.
- `rclpy` (ROS 2용)
- `opencv-python` (OpenCV용)
- `numpy` (Python 로보틱스용)
