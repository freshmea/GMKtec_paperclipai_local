# Day 11 실습 가이드: 에러 전파 (Error Propagation)

이 가이드는 Rust에서 `?` 연산자를 사용하여 에러를 효율적으로 상위 함수로 전달하고, 중첩된 `match` 문 없이 깔끔하고 가독성 높은 에러 처리 파이프라인을 구축하는 방법을 배우는 것을 목표로 합니다.

## 1. 목표
- `?` 연산자의 작동 원리와 에러 전파 메커니즘을 이해한다.
- 여러 단계의 함수 호출에서 발생하는 에러를 단일 `Result` 타입으로 통합하여 관리한다.
- 에러 전파를 통해 코드의 복잡도를 줄이고 비즈니스 로직의 가독성을 높인다.

## 2. 프로젝트 구조
```text
day11_error_propagation/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day11_error_propagation
cd day11_error_propagation
```

### 단계 2: 에러 전파 실습 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, `?` 연산자가 어떻게 중첩된 에러 처리를 단순화하는지 관찰하세요.

```rust
// 에러 타입을 정의
#[derive(Debug)]
enum MyError {
    ReadError(String),
    ParseError(String),
    ValidationError(String),
}

// 에러를 전파하기 위해 Result 타입을 사용
type MyResult<T> = Result<T, MyError>;

// 1단계: 데이터 읽기 (파일 읽기 시뮬레이션)
fn read_data() -> MyResult<String> {
    let data = "10, 20, 30, invalid, 50"; // 중간에 잘못된 데이터 포함
    if data.is_empty() {
        return Err(MyError::ReadError("데이터가 비어있습니다.".to_string()));
    }
    Ok(data.to_string())
}

// 2단계: 데이터 파싱 (문자열 -> 숫자 리스트)
fn parse_data(data: String) -> MyResult<Vec<i32>> {
    let mut numbers = Vec::new();
    for part in data.split(',') {
        let trimmed = part.trim();
        let num = trimmed.parse::<i32>().map_err(|_| {
            MyError::ParseError(format!("숫자 파싱 실패: '{}'", trimmed))
        })?; // '?'를 사용하여 ParseError 발생 시 즉시 반환
        numbers.push(num);
    }
    Ok(numbers)
}

// 3단계: 데이터 검증 (양수인지 확인)
fn validate_data(numbers: Vec<i32>) -> MyResult<Vec<i32>> {
    for &num in &numbers {
        if num < 0 {
            return Err(MyError::ValidationError(format!("음수 발견: {}", num)));
        }
    }
    Ok(numbers)
}

// 4단계: 전체 파이프라인 통합
// '?' 연산자를 사용하여 각 단계의 에러를 상위로 전파
fn process_pipeline() -> MyResult<Vec<i32>> {
    let raw_data = read_data()?;          // ReadError 발생 시 즉시 반환
    let parsed_data = parse_data(raw_data)?; // ParseError 발생 시 즉시 반환
    let validated_data = validate_data(parsed_data)?; // ValidationError 발생 시 즉시 반환
    
    Ok(validated_data)
}

fn main() {
    println!("--- 에러 전파 파이프라인 테스트 ---");

    match process_pipeline() {
        Ok(data) => println!("성공적으로 처리된 데이터: {:?}", data),
        Err(e) => println!("파이프라인 중단됨! 에러 내용: {:?}", e),
    }

    println!("\n[정상 시나리오 테스트]");
    // 정상적인 데이터를 반환하는 함수를 직접 호출하여 성공 케이스 확인
    let success_result = (|| -> MyResult<Vec<i32>> {
        let data = Ok("1, 2, 3".to_string())?;
        let parsed = parse_data(data)?;
        Ok(parsed)
    })();

    match success_result {
        Ok(data) => println!("성공: {:?}", data),
        Err(e) => println!("실패: {:?}", e),
    }
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. **`?`의 마법:** `process_pipeline` 함수 내에서 `?`가 없을 때 `match`를 사용하여 어떻게 코드가 길어질지 상상해 보세요.
2. **에러 타입의 일관성:** `parse_data`에서 `map_err`를 사용하는 이유는 `parse()`가 반환하는 `ParseIntError`를 우리가 정의한 `MyError::ParseError`로 변환하여 `?` 연산자가 사용할 수 있게 만들기 위함입니다.
3. **즉시 반환:** `parse_data`에서 `"invalid"`를 만났을 때, 함수가 나머지 숫자들을 처리하지 않고 즉시 `Err`를 반환하는지 확인하세요.

## 5. 도전 과제 (Extra Credit)
- **에러 변환 확장:** `From` 트레이트를 구현하여 `map_err` 없이 `?`만으로도 `ParseIntError`가 `MyError`로 자동 변환되도록 만들어 보세요.
- **데이터 수정:** `parse_data`에서 숫자가 아닌 값이 들어왔을 때 에러를 내는 대신, 해당 값을 `0`으로 치환하여 계속 진행하도록 코드를 수정해 보세요.
- **더 복잡한 파이프라인:** `validate_data`에 "합계가 100을 넘으면 안 된다"는 규칙을 추가하여 새로운 에러 타입을 정의해 보세요.
