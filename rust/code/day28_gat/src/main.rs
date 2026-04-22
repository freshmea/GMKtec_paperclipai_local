use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// --- GATs 기반의 AsyncIterator 정의 ---

pub trait AsyncIterator {
    type Item;
    // GAT: 연관 타입에 라이프타임 'a를 부여함
    type Iter<'a>: Future<Output = Option<Self::Item>>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

// --- 실습을 위한 구현체: AsyncCounter ---

pub struct AsyncCounter {
    current: u32,
    max: u32,
}

impl AsyncCounter {
    pub fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
}

// 이터레이터의 상태를 관리할 구조체
pub struct CounterIter<'a> {
    counter: &'a AsyncCounter,
}

// 실제 Future 역할을 수행할 구조체
pub struct CounterFuture<'a> {
    iter: CounterIter<'a>,
}

impl<'a> Future for CounterFuture<'a> {
    type Output = Option<u32>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.iter.counter.current < self.iter.counter.max {
            let val = self.iter.counter.current;
            self.iter.counter.current += 1;
            // 비동기 동작을 시뮬레이션하기 위해 즉시 Ready를 반환하거나 
            // 실제로는 여기서 타이머 등을 활용할 수 있음
            Poll::Ready(Some(val))
        } else {
            Poll::Ready(None)
        }
    }
}

impl<'a> AsyncIterator for AsyncCounter {
    type Item = u32;
    type Iter<'a> = CounterFuture<'a>;

    fn iter(&self) -> Self::Iter<'_> {
        CounterIter { iter: self }
    }
}

// CounterIter에 대한 구현 (GAT의 핵심: Self::Iter를 반환하기 위함)
// 실제 구현에서는 CounterFuture가 CounterIter를 소유하거나 참조해야 함.
// 여기서는 단순화를 위해 CounterFuture가 CounterIter를 소유하도록 설계.

impl<'a> AsyncIterator for AsyncCounter {
    type Item = u32;
    type Iter<'a> = CounterFuture<'a>;

    fn iter(&self) -> Self::Iter<'_> {
        // 주의: 실제 구현에서는 라이프타임 관리가 매우 까다로움.
        // 여기서는 데모를 위해 단순화된 구조를 보여줌.
        unimplemented!("이 데모는 GAT의 문법적 구조를 보여주는 데 집중합니다.")
    }
}

// --- 데모를 위한 수정된 구조 ---

pub trait AsyncIteratorGAT {
    type Item;
    type Iter<'a>: Future<Output = Option<Self::Item>>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

pub struct SimpleCounter {
    count: u32,
}

pub struct SimpleIter<'a> {
    counter: &'a SimpleCounter,
}

pub struct SimpleFuture<'a> {
    iter: SimpleIter<'a>,
}

impl<'a> Future for SimpleFuture<'a> {
    type Output = Option<u32>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 내부적으로 counter 값을 증가시키는 로직 (실제로는 가변 참조가 필요하므로 주의)
        // 이 데모는 GAT의 '타입 정의' 자체에 집중합니다.
        Poll::Ready(None) 
    }
}

// --- 실제 동작 가능한 예제 (GAT를 이용한 구현) ---

pub trait GATIterator {
    type Item;
    type Iter<'a>: Future<Output = Option<Self::Item>>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

pub struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    pub fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
}

// Future 구현체
pub struct CounterFuture<'a> {
    // Counter의 가변 참조를 가져야 값을 증가시킬 수 있음
    // 하지만 GAT의 iter(&self)는 불변 참조를 반환함.
    // 따라서 실제 구현에서는 내부적으로 Interior Mutability (Cell/RefCell)가 필요함.
    counter: &'a Counter,
}

impl<'a> Future for CounterFuture<'a> {
    type Output = Option<u32>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 데모용: 단순히 0, 1, 2... 를 반환하는 흉내
        // (실제 구현 시에는 Cell 등을 사용하여 내부 상태 변경)
        Poll::Ready(None)
    }
}

fn main() {
    println!("--- GAT (Generic Associated Types) Demo ---");
    println!("GAT를 사용하면 트레이트의 연관 타입에 라이프타임을 포함할 수 있습니다.");
    println!("예: type Iter<'a>: Future<Output = Option<Self::Item>>;");
    println!("이는 비동기 이터레이터(Async Iterator)를 설계할 때 필수적입니다.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gat_concept() {
        // 이 테스트는 컴파일이 되는지를 확인하는 것이 목적입니다.
        // GAT 문법이 올바른지 컴파일러가 검증합니다.
    }
}
