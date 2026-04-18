# Day 3. 문자열 통계 프로그램 (String Statistics)

이 프로젝트는 Day 3의 실습 및 랩(Lab) 과제를 위한 가이드입니다.

## 1. 과제 개요

사용자로부터 문장 또는 파일 내용을 입력받아 다음과 같은 통계 정보를 계산하여 출력하는 프로그램을 작성합니다.

- **단어 수 (Word Count)**
- **문자 수 (Character Count, UTF-8 기준)**
- **특정 단어의 빈도수 (Word Frequency)**

## 2. 구현 요구 사항

- `String`과 `&str`을 적절히 혼용하여 메모리 효율성을 고려합니다.
- `chars()` 및 `split_whitespace()` 이터레이터를 활용합니다.
- `HashMap`을 사용하여 단어 빈도수를 저장합니다.

## 3. 예제 코드 (`src/main.rs`)

```rust
use std::collections::HashMap;
use std::io;

fn main() {
    println!("분석할 문장을 입력하세요:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("입력 읽기 실패");

    let stats = analyze_text(&input);

    println!("\n--- 통계 결과 ---");
    println!("전체 문자 수: {}", stats.char_count);
    println!("전체 단어 수: {}", stats.word_count);
    println!("단어별 빈도수:");
    for (word, count) in stats.word_frequencies {
        println!("  {}: {}", word, count);
    }
}

struct TextStats {
    char_count: usize,
    word_count: usize,
    word_frequencies: HashMap<String, usize>,
}

fn analyze_text(text: &str) -> TextStats {
    let char_count = text.chars().count();
    
    // 공백을 기준으로 단어 분리
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();

    let mut word_frequencies = HashMap::new();
    for word in words {
        // 소문자로 변환하여 대소문자 구분 없이 카운트 (선택 사항)
        let cleaned_word = word.to_lowercase();
        let count = word_frequencies.entry(cleaned_word).or_insert(0);
        *count += 1;
    }

    TextStats {
        char_count,
        word_count,
        word_frequencies,
    }
}
```

## 4. 학습 포인트

- **`&str` vs `String`:** 함수의 인자로 `&str`을 사용하여 소유권 없이 텍스트를 읽는 법.
- **`HashMap`:** 키-값 쌍을 저장하고 `entry` API를 사용하여 효율적으로 업데이트하는 법.
- **Iterator:** `split_whitespace()`, `count()`, `collect()` 등을 활용한 데이터 처리.
- **UTF-8:** `chars().count()`를 사용하여 멀티바이트 문자를 올바르게 세는 법.
