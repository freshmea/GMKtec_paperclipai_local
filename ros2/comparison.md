# ROS2 버전별 주요 변화점 비교

본 섹션에서는 ROS2 Jazzy Jalisco를 기준으로 차기 릴리스인 Lyrical Luth(가칭)와의 주요 기술적 차이점을 비교합니다.

## 🚀 버전별 비교 요약

| 구분 | Lyrical Luth (차기 릴리스) | Jazzy Jalisco (현재 LTS) |
| :--- | :--- | :--- |
| **기본 OS** | **Ubuntu 26.04 (Resolute)** | Ubuntu 24.04 (Noble) |
| **C++ 표준** | **C++20 지원 강화** | C++17/20 지원 |
| **Python 버전** | **Python 3.12 - 3.14** | Python 3.12 |
| **주요 특징** | Lifecycle node 지원 확대, 새로운 logging 구현 | 보안 및 성능 고도화, LTS 안정성 |

## 🔍 주요 기술적 변화점

### 1. OS 및 환경 지원
* **Lyrical Luth:** Ubuntu 26.04 (Resolute)를 공식 지원하며, 최신 커널 및 라이브러리와의 호환성이 극대화되었습니다.
* **Jazzy:** Ubuntu 24.04 Noble Numbat을 공식 지원합니다.

### 2. 개발 도구 및 라이브러리 (Modernization)
* **C++20:** `rclcpp`에서 C++20 표준을 본격적으로 활용하여 더 안전하고 성능이 뛰어난 코드를 작성할 수 있습니다.
* **Python 3.12+:** 최신 Python 런타임 지원을 통해 스크립팅 및 AI 통합 성능이 개선되었습니다.

### 3. 핵심 기능 업데이트
* **Lifecycle Nodes:** `image_transport` 및 `point_cloud_transport`에서 Lifecycle node 지원이 확장되어 시스템 안정성이 향상되었습니다.
* **Class Loader:** `class_loader`의 인자 지원이 강화되어 컴포넌트 로딩 방식이 더욱 유연해졌습니다.
* **Logging:** 새로운 `rcl_logging_implementation`을 통해 더욱 세밀하고 효율적인 로깅이 가능합니다.

---
*참고: 상세한 기술 변경 사항은 공식 ROS 2 Release Notes를 참조하십시오.*
