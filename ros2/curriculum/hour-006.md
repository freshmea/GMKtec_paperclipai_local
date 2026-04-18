# [Hour 006] ROS2 기초: TF2 좌표 변환 시스템

## 🎯 학습 목표
- ROS2의 좌표 변환 시스템인 TF2의 핵심 개념을 이해합니다.
- 정적(Static) 및 동적(Dynamic) 좌표 변환(Transform)의 차이를 학습합니다.
- `tf2_ros` 라이브러리를 사용하여 좌표 변환을 발행(Broadcast)하고 조회(Listen)할 수 있습니다.
- TF 트리(TF Tree)의 구조와 좌표계 간의 관계를 파악합니다.

## 📖 학습 내용
1. **TF2 개념 이해**
   - 좌표계(Coordinate Frames)와 변환(Transform)의 정의
   - TF 트리(TF Tree) 구조: 부모-자식 관계를 통한 계층적 좌표계 관리
   - 정적 변환(Static Transform) vs 동적 변환(Dynamic Transform)
2. **TF2 라이브러리 활용**
   - `tf2_ros` 패키지 사용법
   - `TransformBroadcaster`: 좌표 변환 정보 발행
   - `TransformListener`: 좌표 변환 정보 수신 및 계산
3. **실습: 간단한 좌표계 구성**
   - `base_link` $\rightarrow$ `sensor_link` 형태의 정적 변환 발행
   - 이동하는 좌표계(예: 로봇 본체)와 고정 좌표계(예: 세계 좌표계) 간의 동적 변환 시뮬레이션
4. **도구 활용**
   - `view_frames`: 현재 TF 트리의 구조를 PDF로 시각화
   - `tf2_echo`: 특정 두 좌표계 사이의 변환 값 확인

## 🔗 공식 문서 및 참고 자료
- [TF2 Tutorial](https://docs.ros.org/en/rolling/Tutorials/Beginner-Client-Libraries/TF2/Introducing-TF2.html)
- [ROS 2 TF2 Concepts](https://docs.ros.org/en/jazzy/Concepts/Intermediate/Design.html#tf2-transforms)

---
*본 문서는 교육 목적으로 작성되었습니다.*
