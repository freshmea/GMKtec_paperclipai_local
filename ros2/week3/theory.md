# 3주차: 시스템 통합 (Launch 파일 및 TF2)

## 🎯 학습 목표
* 여러 개의 ROS2 노드를 한 번에 실행하기 위한 Launch 파일의 개념과 작성법을 익힙니다.
* TF2(Transform Library)를 사용하여 좌표 변환 시스템의 원리를 이해합니다.
* 정적(Static) 및 동적(Dynamic) 좌표 변환의 차이를 파악합니다.
* Lyrical Luth의 강화된 `class_loader`를 활용한 컴포넌트 로딩 방식을 이해합니다.

## 📖 이론 (Theory)

### 1. ROS2 Launch 파일
복잡한 로봇 시스템은 수많은 노드로 구성됩니다. Launch 파일을 사용하면:
* 여러 노드를 한 번의 명령으로 실행할 수 있습니다.
* 노드별 파라미터(Parameter), 리매핑(Remapping), 네임스페이스(Namespace)를 설정할 수 있습니다.
* Python, XML, YAML 형식을 지원하며, 특히 Python Launch는 강력한 프로그래밍 기능을 제공합니다.

### 2. TF2 (Transform Library)
로봇의 각 부품(바퀴, 센서, 팔 등)은 서로 다른 좌표계를 가집니다. TF2는 이 좌표계들 사이의 관계를 관리합니다.
* **Tree 구조:** 모든 좌표 변환은 `map` -> `odom` -> `base_link` -> `sensor_link`와 같은 계층적 트리 구조를 가집니다.
* **Static Transform:** 변하지 않는 관계 (예: 로봇 본체에 고정된 LiDAR).
* **Dynamic Transform:** 움직이는 관계 (예: 로봇이 이동함에 따라 변하는 `odom`과 `base_link` 사이의 관계).

### 3. Component Loading (Lyrical Luth 특화)
Lyrical Luth에서는 컴포넌트 기반 설계가 더욱 권장됩니다. `class_loader`를 사용하여 실행 중에 노드를 동적으로 로드하거나, 여러 노드를 하나의 프로세스 내에서 실행하여 통신 오버헤드를 최소화할 수 있습니다.

## 🔗 공식 출처
* [Writing Launch Files](https://docs.ros.org/en/jazzy/Tutorials/Beginner-Client-Libraries/Writing-Your-First-Launch-File.html)
* [TF2 Tutorial](https://docs.ros.org/en/jazzy/Tutorials/Robotters/tf2/IntroductionToTF2.html)
