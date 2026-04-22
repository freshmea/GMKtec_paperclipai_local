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
