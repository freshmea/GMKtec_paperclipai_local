# Day 10 실습 가이드: 트레이트 객체 (Trait Objects)

이 가이드는 Rust의 정적 다형성(Generics)과 동적 다형성(Trait Objects)의 차이를 이해하고, `dyn Trait`를 사용하여 런타임에 서로 다른 타입을 하나의 컬렉션으로 다루는 방법을 배우는 것을 목표로 합니다.

## 1. 목표
- `Box<dyn Trait>`를 사용하여 런타임 다형성을 구현한다.
- 정적 디스패치(Static Dispatch)와 동적 디스패치(Dynamic Dispatch)의 차이점을 이해한다.
- `Trait Object`를 사용할 때의 메모리 레이아웃(vtable) 개념을 파악한다.

## 2. 프로젝트 구조
```text
day10_trait_objects/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day10_trait_objects
cd day10_trait_objects
```

### 단계 2: 동적 다형성 실습 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 정적 디스패치와 동적 디스패치의 동작 차이를 관찰하세요.

```rust
// 1. 트레이트 정의
trait Animal {
    fn make_sound(&self);
    fn name(&self) -> &str;
}

// 2. 다양한 타입 구현
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
    fn name(&self) -> &str {
        &self.name
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
    fn name(&self) -> &str {
        &self.name
    }
}

// 3. 정적 디스패치 (Static Dispatch) - Generics 사용
// 컴파일 타임에 타입이 결정되어 코드가 인라인화될 수 있음 (성능상 이점)
fn dog_bark<T: Animal>(animal: &T) {
    print!("{} says: ", animal.name());
    animal.make_sound();
}

// 4. 동적 디스패치 (Dynamic Dispatch) - Trait Objects 사용
// 런타임에 vtable을 통해 메서드를 찾아 호출함 (유연성 상의 이점)
fn animal_sound_dyn(animal: &dyn Animal) {
    print!("{} says (dyn): ", animal.name());
    animal.make_sound();
}

fn main() {
    let dog = Dog { name: String::from("Buddy") };
    let cat = Cat { name: String::from("Whiskers") };

    println!("--- 정적 디스패치 (Generics) ---");
    dog_bark(&dog);
    // dog_bark(&cat); // 컴파일 에러가 나지 않음 (T는 Animal을 구현했으므로)
    // 하지만 컴파일러는 dog_bark<Dog>와 dog_bark<Cat> 두 버전을 각각 생성함 (Monomorphization)

    println!("\n--- 동적 디스패치 (Trait Objects) ---");
    animal_sound_dyn(&dog);
    animal_sound_dyn(&cat);

    println!("\n--- 컬렉션에 담기 (Heterogeneous Collection) ---");
    // 서로 다른 타입을 하나의 벡터에 담으려면 Box<dyn Animal>이 필요함
    let zoo: Vec<Box<dyn Animal>> = vec![
        Box::new(dog),
        Box::new(cat),
    ];

    for animal in zoo {
        print!("Zoo visitor sees: {} says: ", animal.name());
        animal.make_sound();
    }
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. **Heterogeneous Collection:** 왜 `Vec<Animal>`은 불가능하고 `Vec<Box<dyn Animal>>`은 가능한지 생각해보세요. (크기가 다른 타입들을 담기 위해 포인터가 필요함)
2. **Performance vs Flexibility:** `dog_bark<T: Animal>`과 `animal_sound_dyn(&dyn Animal)`의 차이를 컴파일러의 관점에서 고민해보세요.
3. **vtable:** `dyn Animal`이 호출될 때, 프로그램이 어떻게 `make_sound` 메서드의 실제 주소를 찾아내는지(vtable) 개념을 정리해보세요.

## 5. 도전 과제 (Extra Credit)
- `Animal` 트레이트에 `age(&self) -> u32` 메서드를 추가하고, 모든 동물이 나이를 가지도록 구현해 보세요.
- `Box<dyn Animal>` 대신 `&dyn Animal`을 사용하여 `zoo` 리스트를 순회하는 함수를 작성해 보세요.
- `dyn Animal`을 사용할 때 `Clone` 트레이트를 구현하는 것이 왜 어려운지 조사해 보세요. (힌트: `Clone`은 `Self`를 반환하는데, `dyn Trait`는 크기를 알 수 없음)
