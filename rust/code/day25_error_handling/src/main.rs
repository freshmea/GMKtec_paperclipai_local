// --- 1. Library Layer (Simulated) ---
// In a real project, this would be in a separate crate.
// 라이브러리 레이어에서는 명확하고 구조화된 에러 타입을 정의하여 
// 사용자(애플리케이션 개발자)에게 어떤 에러가 발생했는지 정확히 알려줍니다.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("connection error: {0}")]
    ConnectionError(String),
    #[error("query error: {0}")]
    QueryError(String),
    #[error("user not found: {0}")]
    NotFound(u32),
}

pub fn fetch_user(id: u32) -> Result<String, DbError> {
    if id == 0 {
        return Err(DbError::ConnectionError("Database is down".to_string()));
    }
    if id > 100 {
        return Err(DbError::NotFound(id));
    }
    Ok(format!("User_{}", id))
}

// --- 2. Application Layer ---
// 애플리케이션 레이어에서는 구체적인 에러 타입보다는 
// 에러의 발생 맥락(Context)을 추가하여 에러를 전파하는 데 집중합니다.

use anyhow::{Context, Result};

pub fn run_service(user_id: u32) -> Result<String> {
    // `?` 연산자로 에러를 전파하면서, `.context()`를 사용하여 
    // "왜 이 에러가 발생했는지"에 대한 정보를 덧붙입니다.
    let user = fetch_user(user_id)
        .with_context(|| format!("Failed to retrieve user with ID: {}", user_id))?;
    
    Ok(user)
}

// --- 3. Main Execution ---

fn main() {
    println!("--- Error Handling Demo ---");

    // Case 1: Success
    println!("\n[Case 1: Success]");
    match run_service(42) {
        Ok(user) => println!("Success: {}", user),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    // Case 2: Connection Error (Root Cause)
    println!("\n[Case 2: Connection Error]");
    match run_service(0) {
        Ok(user) => println!("Success: {}", user),
        Err(e) => {
            // `{:?}`를 사용하여 출력하면 컨텍스트와 근본 원인(Root Cause)이 포함된 전체 에러 체인을 볼 수 있습니다.
            eprintln!("Detailed Error Chain:\n{:?}", e);
        }
    }

    // Case 3: Not Found Error
    println!("\n[Case 3: Not Found]");
    match run_service(999) {
        Ok(user) => println!("Success: {}", user),
        Err(e) => {
            eprintln!("Detailed Error Chain:\n{:?}", e);
        }
    }

    println!("\n--- Demo Finished ---");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_user_success() {
        assert_eq!(fetch_user(1).unwrap(), "User_1");
    }

    #[test]
    fn test_fetch_user_not_found() {
        let res = fetch_user(101);
        assert!(matches!(res, Err(DbError::NotFound(101))));
    }

    #[test]
    fn test_app_error_context() {
        let res = run_service(0);
        assert!(res.is_err());
        let err = res.unwrap_err();
        // 에러 메시지에 우리가 추가한 컨텍스트가 포함되어 있는지 확인합니다.
        assert!(format!("{}", err).contains("Failed to retrieve user with ID: 0"));
    }
}
