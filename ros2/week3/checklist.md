# 3주차 체크리스트: Launch 파일 및 TF2
 
## ✅ 완료 여부 확인
 
### 1. Launch 파일
- [ ] `launch/` 디렉토리가 패키지 내에 존재하는가?
- [ ] `setup.py`에 launch 파일 경로가 올바르게 등록되었는가?
- [ ] `ros2 launch` 명령어로 여러 노드가 한 번에 실행되는가?
- [ ] Launch 파일을 통해 노드에 파라미터를 전달할 수 있는가?
- [ ] (Lyrical Luth) 컴포넌트 기반 Launch 파일을 작성하고 실행해 보았는가?
 
### 2. TF2 (좌표 변환)
- [ ] `static_transform_publisher`를 통해 정적 변환이 생성되었는가?
- [ ] `tf2_ros`를 사용하여 동적 변환을 브로드캐스팅하는 노드를 작성했는가?
- [ ] `view_frames` 명령어를 통해 TF 트리가 정상적으로 생성되었음을 확인했는가?
- [ ] `rviz2`에서 좌표계(Frame) 간의 관계가 시각적으로 올바르게 나타나는가?
 
## 💡 문제 해결 (Troubleshooting)
* **Launch 파일 실행 실패:** `setup.py`의 `data_files` 설정을 확인하고 다시 `colcon build` 하세요.
* **TF 트리가 끊김:** 부모 프레임(Parent Frame)과 자식 프레임(Child Frame)의 이름이 일치하는지, 그리고 `base_link`가 트리의 중심에 잘 연결되어 있는지 확인하세요.
* **좌표축이 이상하게 보임:** Euler 각도(Roll, Pitch, Yaw) 또는 Quaternion 값이 올바르게 설정되었는지 확인하세요.
* **컴포넌트 로딩 실패:** `class_loader` 설정 및 패키지 의존성이 올바른지 확인하세요.
