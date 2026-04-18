# Day 7 실습 가이드: 스마트 포인터 - Box<T> (Recursive Types)

이 가이드는 힙(Heap) 메모리에 데이터를 할당하고 관리하는 가장 기본적인 스마트 포인터인 `Box<T>`를 배우고, 이를 통해 재귀적 구조(Recursive Types)를 어떻게 구현하는지 익히는 것을 목표로 합니다.

## 1. 목표
- `Box<T>`를 사용하여 데이터를 힙에 할당한다.
- 재귀적 타입(Recursive Type)의 정의와 필요성을 이해한다.
- `Box<T>`를 사용하여 간단한 트리(Tree) 또는 리스트(List) 구조를 구현한다.

## 2. 프로젝트 구조
```text
day7_box_recursive/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day7_box_recursive
cd day7_box_recursive
```

### 단계 2: 재귀적 트리 구조 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하세요. 트리 구조를 정의할 때 왜 `Box`가 필요한지 코드를 통해 확인합니다.

```rust
// 재귀적 구조체 정의
// Node는 자기 자신(Node)을 필드로 가질 수 있습니다.
// 하지만 Rust는 컴파일 타임에 구조체의 크기를 알아야 하므로, 
// 직접적인 재귀는 불가능합니다. 대신 크기가 고정된 '포인터'인 Box를 사용합니다.

#[derive(Debug)]
enum TreeNode {
    Node {
        value: i32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    },
    Empty,
}

impl TreeNode {
    // 새로운 노드 생성
    fn new(value: i32) -> Self {
        TreeNode::Node {
            value,
            left: None,
            right: None,
        }
    }

    // 왼쪽 자식 추가
    fn set_left(&mut self, left: TreeNode) {
        if let TreeNode::Node { left: ref mut l, .. } = self {
            *l = Some(Box::new(left));
        }
    }

    // 오른쪽 자식 추가
    fn set_right(&mut self, right: TreeNode) {
        if let TreeNode::Node { right: ref mut r, .. } = self {
            *r = Some(Box::new(right));
        }
    }
}

fn main() {
    println!("--- 재귀적 트리 구조 구현 ---");

    // 1. 루트 노드 생성
    let mut root = TreeNode::new(10);

    // 2. 자식 노드 생성 및 연결
    let mut left_child = TreeNode::new(5);
    left_child.set_left(TreeNode::new(2));
    left_child.set_right(TreeNode::new(7));

    let mut right_child = TreeNode::new(15);
    right_child.set_left(TreeNode::new(12));
    right_child.set_right(TreeNode::new(20));

    // 3. 루트에 자식 연결
    root.set_left(left_child);
    root.set_right(right_child);

    // 4. 트리 출력 (Debug 트레이트 활용)
    println!("{:#?}", root);
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. `enum TreeNode`에서 `Box<TreeNode>` 대신 그냥 `TreeNode`를 사용했을 때 어떤 에러가 발생하는지 주석을 풀고 확인해 보세요. (에러 메시지: "recursive type `TreeNode` has infinite size")
2. `Box`가 왜 '크기를 알 수 없는 타입'을 '고정된 크기의 포인터'로 바꾸어 주는 역할을 하는지 이해해 보세요.

## 5. 도전 과제 (Extra Credit)
- 트리의 모든 노드에 있는 값을 합산하는 `sum(&self) -> i32` 메서드를 구현해 보세요.
- 트리의 깊이(depth)를 계산하는 `depth(&self) -> usize` 메서드를 구현해 보세요.
- `Box` 대신 다른 포인터(예: `&`)를 사용했을 때의 차이점을 고민해 보세요.
