use std::fmt;
use std::io;

// 1. 커스텀 에러 타입 정의
#[derive(Debug)]
enum AppError {
    Io { source: io::Error, context: String }, // 에러 컨텍스트 추가 (Extra Credit)
    Parse(String),           // 파싱 에러
    InvalidValue(i32),       // 값 검증 에러
}

// 2. Display 트레이트 구현 (사용자에게 보여줄 에러 메시지)
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io { source, context } => write!(f, "IO 오류 발생 [{}]: {}", context, source),
            AppError::Parse(msg) => write!(f, "데이터 파싱 오류: {}", msg),
            AppError::InvalidValue(val) => write!(f, "유효하지 않은 값: {}", val),
        }
    }
}

// 3. Error 트레이트 구현
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

// 4. From 트레이트 구현
// Note: Since we added context to AppError::Io, we can't use a simple From<io::Error> 
// if we want to provide specific context at each call site. 
// Instead, we'll use map_err or a custom helper for context-aware wrapping.
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io { 
            source: err, 
            context: "Unknown context".to_string() 
        }
    }
}

// 5. 비즈니스 로직 함수들
fn read_config_value(filename: &str) -> Result<String, AppError> {
    // Context-aware wrapping using map_err
    let content = std::fs::read_to_string(filename).map_err(|e| AppError::Io {
        source: e,
        context: format!("파일 '{}' 읽기 실패", filename),
    })?; 
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
    let filename = "config.txt";
    let content = read_config_value(filename)?;
    let val = parse_config(content)?;
    let valid_val = validate_config(val)?;
    Ok(valid_val)
}

fn main() {
    println!("--- 커스텀 에러 처리 테스트 (Extra Credit 적용) ---");

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
