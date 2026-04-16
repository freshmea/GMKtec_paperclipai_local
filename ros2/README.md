# ROS2 Lyrical Luth 한글 교육 커리큘럼

이 문서는 ROS2 Lyrical Luth(차기 릴리스)를 기준으로 설계된 로보틱스 개발자를 위한 160시간(4주) 단계별 교육 과정 안내서입니다.

## 📚 교육 목표
* ROS2 Lyrical Luth의 차세대 아키텍처 이해
* Ubuntu 26.04 기반의 최신 개발 환경 구축 및 워크스페이스 관리 능력 습득
* Topic, Service, Action을 이용한 고성능 노드 간 통신 구현
* Launch 파일을 이용한 시스템 통합 및 강화된 TF2 좌표 변환 활용
* Rviz2 및 Nav2를 활용한 시각화, SLAM, 내비게이션 기술 습득

## 🗓️ 커리큘럼 구성 (총 160시간)

| 주차 | 주제 | 주요 내용 |
| :--- | :--- | :--- |
| **1주차** | **환경 설정 및 기초** | Ubuntu 26.04, Lyrical Luth 설치, Workspace 구성, 패키지 생성 및 빌드(colcon) |
| **2주차** | **통신 아키텍처** | Topic (Pub/Sub), Service (Client/Server), Action (Goal/Feedback/Result), QoS 최적화 |
| **3주차** | **시스템 통합** | Launch 파일 작성, TF2 좌표 변환 시스템, 컴포넌트 기반 설계 (class_loader) |
| **4주차** | **시각화 및 내비게이션** | Rviz2 활용, Lifecycle Node 기반 데이터 스트림 시각화, Nav2 및 SLAM Toolbox |

## 🛠️ 기술 스택
* **OS:** Ubuntu 26.04 (Resolute)
* **ROS2 Distro:** Lyrical Luth (Rolling Release)
* **Build Tool:** colcon
* **Language:** Python (3.12+), C++ (C++20)

## 🔗 공식 문서 및 참고 자료
* [ROS 2 Documentation (Rolling)](https://docs.ros.org/en/rolling/index.html)
* [ROS 2 Design Concepts](https://docs.ros.org/en/jazzy/Concepts/Intermediate/Design.html)

---
*본 교육 자료는 공식 ROS 2 문서를 기반으로 Lyrical Luth 환경에 맞춰 작성되었습니다.*
