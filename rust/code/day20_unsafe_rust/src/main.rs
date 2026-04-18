use std::ptr;

/// `UnsafeStack`은 Raw Pointer를 사용하여 스택을 구현한 예제입니다.
/// 내부적으로는 `unsafe`를 사용하지만, 외부 인터페이스는 완전히 Safe 합니다.
pub struct UnsafeStack<T> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
}

impl<T> UnsafeStack<T> {
    /// 새로운 스택을 생성합니다.
    pub fn new(capacity: usize) -> Self {
        // 메모리를 할당하고 raw pointer를 얻습니다.
        let layout = std::alloc::Layout::array::<T>(capacity).expect("Failed to create layout");
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };

        if ptr.is_null() {
            panic!("Failed to allocate memory");
        }

        Self {
            ptr,
            capacity,
            len: 0,
        }
    }

    /// 스택에 요소를 추가합니다.
    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.len >= self.capacity {
            return Err("Stack overflow");
        }

        unsafe {
            // 포인터 연산을 통해 올바른 위치에 값을 씁니다.
            // ptr.add(index)는 index만큼 떨어진 주소를 반환합니다.
            ptr::write(self.ptr.add(self.len), value);
        }

        self.len += 1;
        Ok(())
    }

    /// 스택에서 요소를 꺼냅니다.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        unsafe {
            // ptr.add(index)를 사용하여 값을 읽어옵니다.
            // ptr::read는 값을 복사하여 가져오며, 메모리 소유권을 이전하는 것과 유사하게 동작합니다.
            Some(ptr::read(self.ptr.add(self.len)))
        }
    }

    /// 현재 스택의 길이를 반환합니다.
    pub fn len(&self) -> usize {
        self.len
    }

    /// 스택이 비어있는지 확인합니다.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// 메모리 누수를 방지하기 위해 Drop 트레이트를 구현해야 합니다.
impl<T> Drop for UnsafeStack<T> {
    fn drop(&mut self) {
        if self.len > 0 {
            unsafe {
                // 남아있는 요소들을 drop 시켜줍니다.
                for i in 0..self.len {
                    ptr::drop_in_place(self.ptr.add(i));
                }
            }
        }

        let layout = std::alloc::Layout::array::<T>(self.capacity).expect("Failed to create layout");
        unsafe {
            std::alloc::dealloc(self.ptr as *mut u8, layout);
        }
        println!("UnsafeStack: Memory deallocated");
    }
}

fn main() {
    println!("--- UnsafeStack Demo ---");

    let mut stack = UnsafeStack::new(5);

    println!("Pushing: 10, 20, 30");
    stack.push(10).unwrap();
    stack.push(20).unwrap();
    stack.push(30).unwrap();

    println!("Stack length: {}", stack.len());

    println!("Popping elements:");
    while let Some(val) = stack.pop() {
        println!("Popped: {}", val);
    }

    println!("Stack is empty: {}", stack.is_empty());
    println!("--- Demo Finished ---");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stack = UnsafeStack::new(3);
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_overflow() {
        let mut stack = UnsafeStack::new(1);
        stack.push(1).unwrap();
        let result = stack.push(2);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_pop() {
        let mut stack: UnsafeStack<i32> = UnsafeStack::new(5);
        assert_eq!(stack.pop(), None);
    }
}
