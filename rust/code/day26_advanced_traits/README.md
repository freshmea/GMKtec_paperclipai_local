# Day 26 실습 가이드: Advanced Traits (Bounds, Associated Types, Default Impls)

이 가이드는 Rust의 핵심 추상화 도구인 Trait을 깊이 있게 다룹니다. 단순히 동작을 정의하는 것을 넘어, 정적/동적 디스패치의 차이, 연관 타입(Associated Types), 그리고 기본 구현(Default Implementations)을 통해 어떻게 유연하고 강력한 인터페이스를 설계할 수 있는지 배웁니다.

## 1. 목표
- **Trait Bounds**를 이해하고, 정적 디스패치(Static Dispatch)와 동적 디스패치(Dynamic Dispatch)의 차이점을 파악한다.
- **Associated Types**를 사용하여 Trait의 유연성을 높이는 방법을 익힌다.
- **Default Implementations**를 통해 코드 재사용성을 높이는 방법을 배운다.

## 2. 프로젝트 구조
```text
day26_advanced_traits/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 의존성 확인
`Cargo.toml`에 별도의 외부 라이브러리가 필요하지 않음을 확인합니다.

### 단계 2: 실습 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 각 섹션의 주석을 따라가며 실험해 보세요.

```rust
// --- 1. Trait Bounds (Static vs Dynamic Dispatch) ---
// Trait은 공통된 동작을 정의합니다. 
// 이를 사용하는 방식에 따라 정적 디스패치(Static Dispatch)와 동적 디스패치(Dynamic Dispatch)로 나뉩니다.

trait Speaker {
    fn speak(&self);
}

struct Dog;
impl Speaker for Dog {
    fn speak(&self) { println!("Woof!"); }
}

struct Cat;
impl Speaker for Cat {
    fn speak(&self) { println!("Meow!"); }
}

// 정적 디스패치 (Static Dispatch): 제네릭을 사용하여 컴파일 타임에 구체적인 타입이 결정됩니다. (Monomorphization)
// 성능상 이점이 있지만, 코드 크기가 커질 수 있습니다.
fn make_dog_speak<T: Speaker>(animal: T) {
    animal.speak();
}

// 동적 디스패치 (Dynamic Dispatch): trait object (&dyn Speaker)를 사용하여 런타임에 결정됩니다.
// 유연하지만 vtable을 거쳐야 하므로 약간의 오버헤드가 있습니다.
fn make_animal_speak(animal: &dyn Speaker) {
    animal.speak();
}

// --- 2. Associated Types ---
// Trait 내부에 연관 타입을 정의하여, 구현체마다 특정 타입을 지정할 수 있게 합니다.
// 제네릭보다 코드가 깔끔해지고, 타입 간의 관계를 명확히 할 수 있습니다.

trait Container {
    type Item; // Associated Type
    fn get(&self) -> Self::Item;
    fn add(&mut self, item: Self::Item);
}

struct IntContainer(Vec<i32>);
impl Container for IntContainer {
    type Item = i32;
    fn get(&self) -> i32 { self.0[0] }
    fn add(&mut self, item: i32) { self.0.push(item); }
}

// --- 3. Default Trait Implementations ---
// Trait 정의 시 기본 동작을 제공할 수 있습니다.

trait Summary {
    fn summarize(&self) -> String {
        String::from("(No summary available)")
    }
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Title: {}", self.title)
    }
}

fn main() {
    println!("--- 1. Trait Bounds ---");
    let dog = Dog;
    let cat = Cat;

    make_dog_speak(dog); // Static dispatch
    make_animal_speak(&cat); // Dynamic dispatch

    println!("\n--- 2. Associated Types ---");
    let mut container = IntContainer(vec![10, 20]);
    println!("Container item: {}", container.get());
    container.add(30);
    println!("After add: {:?}", container.get());

    println!("\n--- 3. Default Implementations ---");
    let article = Article {
        title: String::from("Rust Mastery"),
        content: String::from("Learning traits..."),
    };
    println!("{}", article.summarize());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_dispatch() {
        make_dog_speak(Dog);
    }

    #[test]
    fn test_associated_types() {
        let mut container = IntContainer(vec![1]);
        container.add(2);
        assert_eq!(container.get(), 1);
    }
}
```

## 4. 실행 및 테스트
```bash
cargo run
cargo test
```

**실습 포인트:**
1. **Static vs Dynamic:** `make_dog_speak<T: Speaker>(animal: T)`는 컴파일 타임에 타입이 결정되지만, `make_animal_speak(animal: &dyn Speaker)`는 런타임에 결정됩니다. 각각의 장단점(성능 vs 유연성)을 정리해 보세요.
2. **Associated Types:** `Container` Trait에서 `type Item;`이 왜 제네릭(`Container<T>`)보다 유리할 수 있는지(예: 하나의 구현체가 하나의 타입만 가져야 하는 경우) 고민해 보세요.
3. **Default Impls:** Trait에 기본 구현을 제공했을 때, 이를 오버라이딩(Override)하는 방식과 그렇지 않은 방식의 차이를 확인해 보세요.

## 5. 도전 과제 (Extra Credit)
- **Supertraits:** 특정 Trait을 구현하려면 반드시 다른 Trait도 구현해야 하는 'Supertrait' 개념을 찾아보고 코드로 구현해 보세요. (예: `trait Animal: Display {}`)
- **Blanket Implementation:** 특정 Trait을 구현한 모든 타입에 대해 자동으로 다른 Trait을 구현해 주는 'Blanket Implementation'을 실습해 보세요.
- **Trait Objects with Lifetimes:** `&dyn Speaker<'a>`와 같이 Trait Object에 Lifetime이 필요한 경우를 조사해 보세요.
