# Day 11 실습 가이드: 사용자 정의 에러 (Custom Errors)

이 가이드는 프로젝트의 도메인 요구사항에 맞는 구체적인 에러 타입을 정의하고, 이를 통해 시스템의 오류 상황을 명확하고 체계적으로 관리하는 방법을 배우는 것을 목표로 합니다.

## 1. 목표
- `enum`을 사용하여 다양한 에러 상황을 하나의 타입으로 그룹화한다.
- `std::error::Error` 트레이트를 구현하여 표준 에러 시스템과 호환시킨다.
- `From` 트레이트를 사용하여 하위 레벨의 에러(예: `io::Error`)를 상위 레벨의 커스텀 에러로 변환(Wrapping)한다.

## 2. 프로젝트 구조
```text
day11_custom_error/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day11_custom_error
cd day11_custom_error
```

### 단계 2: 커스텀 에러 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하세요. 파일 읽기 실패와 데이터 파싱 실패를 하나의 `AppError`로 통합하는 과정을 살펴봅니다.

```rust
use std::fmt;
use std::io;

// 1. 커스텀 에러 타입 정의
#[derive(Debug)]
enum AppError {
    Io(io::Error),           // IO 에러 래핑
    Parse(String),           // 파싱 에러
    InvalidValue(i32),       // 값 검증 에러
}

// 2. Display 트레이트 구현 (사용자에게 보여줄 에러 메시지)
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO 오류 발생: {}", err),
            AppError::Parse(msg) => write!(f, "데이터 파싱 오류: {}", msg),
            AppError::InvalidValue(val) => write!(f, "유효하지 않은 값: {}", val),
        }
    }
}

// 3. Error 트레이트 구현
impl std::error::Error for AppError {}

// 4. From 트레이트 구현 (에러 자동 변환을 위한 핵심!)
// io::Error가 발생했을 때 자동으로 AppError::Io로 변환해줌
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

// 5. 비즈니스 로직 함수들
fn read_config_value() -> Result<String, AppError> {
    // std::fs::read_to_string은 io::Result를 반환함
    // '?'를 사용하면 From 구현 덕분에 AppError로 자동 변환됨
    let content = std::fs::read_to_string("config.txt")?; 
    Ok(content)
}

fn parse_config(content: String) -> Result<i32, AppError> {
    content.trim().parse::<i32>().map_err(|_| {
        AppError::Parse("숫자 형식이 아닙니다.".to_string())
    })
}

fn validate_config(val: i32) -> Result<i32, AppError> {
    if val < 0 {
        return Err(AppError::InvalidValue(val));
    }
    Ok(val)
}

fn run_app() -> Result<i32, AppError> {
    let content = read_config_value()?;
    let val = parse_config(content)?;
    let valid_val = validate_config(val)?;
    Ok(valid_val)
}

fn main() {
    println!("--- 커스텀 에러 처리 테스트 ---");

    // 테스트를 위해 임시 파일 생성
    let _ = std::fs::write("config.txt", "42");

    match run_app() {
        Ok(val) => println!("설정 로드 성공: {}", val),
        Err(e) => println!("애플리케이션 에러: {}", e),
    }

    // 테스트 2: 파일이 없는 경우
    let _ = std::fs::remove_file("config.txt");
    println!("\n[파일 없는 경우 테스트]");
    if let Err(e) = run_app() {
        println!("예상된 에러: {}", e);
    }

    // 테스트 3: 파싱 에러
    let _ = std::fs::write("config.txt", "not_a_number");
    println!("\n[잘못된 형식 테스트]");
    if let Err(e) = run_app() {
        println!("예상된 에러: {}", e);
    }
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. **`From` 트레이트의 마법:** `read_config_value` 함수 내에서 `std::fs::read_to_string(...)?`를 사용할 때, 어떻게 `io::Error`가 `AppError::Io`로 변환되는지 `From` 구현 부분을 집중해서 보세요.
2. **`map_err` 활용:** `parse_config`에서 `Result`의 에러 타입을 `AppError::Parse`로 바꾸기 위해 `map_err`를 사용하는 패턴을 익히세요.
3. **에러의 계층화:** 하위 레벨의 에러를 상위 레벨의 에러로 래핑함으로써, 호출자에게 더 풍부한 문맥(Context)을 제공하는 방법을 이해합니다.

## 5. 도전 과제 (Extra Credit)
- **에러 컨텍스트 추가:** `AppError`에 어떤 파일에서 에러가 났는지 정보를 담을 수 있는 필드를 추가해 보세요.
- **`thiserror` 라이브러리 조사:** 실무에서 가장 많이 쓰이는 `thiserror` 크레이트가 위에서 구현한 코드를 어떻게 간결하게 만들어 주는지 확인해 보세요.
- **다양한 에러 변환:** `parse_config`에서 `map_err` 대신 `From` 트레이트를 사용하여 구현할 수 있는 방법이 있을지 고민해 보세요.
