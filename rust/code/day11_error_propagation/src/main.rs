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
