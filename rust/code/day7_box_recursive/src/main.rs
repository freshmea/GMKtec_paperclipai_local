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

    // [Extra Credit] 트리의 모든 노드에 있는 값을 합산
    fn sum(&self) -> i32 {
        match self {
            TreeNode::Node { value, left, right } => {
                let left_sum = left.as_ref().map_or(0, |node| node.sum());
                let right_sum = right.as_ref().map_or(0, |node| node.sum());
                value + left_sum + right_sum
            }
            TreeNode::Empty => 0,
        }
    }

    // [Extra Credit] 트리의 깊이(depth)를 계산
    fn depth(&self) -> usize {
        match self {
            TreeNode::Node { left, right, .. } => {
                let left_depth = left.as_ref().map_or(0, |node| node.depth());
                let right_depth = right.as_ref().map_or(0, |node| node.depth());
                1 + std::cmp::max(left_depth, right_depth)
            }
            TreeNode::Empty => 0,
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

    // 5. Extra Credit 결과 출력
    println!("\n--- Extra Credit ---");
    println!("Tree Sum: {}", root.sum());
    println!("Tree Depth: {}", root.depth());
}
