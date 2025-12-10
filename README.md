# bareun_rs

[![Crates.io](https://img.shields.io/crates/v/bareun_rs)](https://crates.io/crates/bareun_rs)

**bareun_rs** is a Rust library for Bareun, a Korean morphological analyzer.

Bareun is a Korean NLP system that provides:
- Tokenizing
- POS (Part-of-Speech) tagging
- Korean spelling correction

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bareun_rs = "1.7"
tokio = { version = "1", features = ["full"] }
```

## Getting Started

### Prerequisites

You need a Bareun API key to use this library:
1. Visit [https://bareun.ai/](https://bareun.ai/)
2. Sign up and verify your email
3. Get your API key from "Login → My Info"

Or use Docker:
```shell
docker pull bareunai/bareun:latest
```

## Usage

### Tagger (POS Tagging)

```rust
use bareun_rs::{Tagger, Tagged};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize with API key
    let api_key = "koba-ABCDEFG-1234567-LMNOPQR-7654321"; // Replace with your API key

    // Use official hosted endpoint
    let mut tagger = Tagger::new(api_key, "api.bareun.ai", Some(443), vec![]).await?;

    // Or use localhost if you have your own server
    // let mut tagger = Tagger::new(api_key, "localhost", Some(5656), vec![]).await?;

    // Get morphemes
    let morphs = tagger.morphs("안녕하세요, 반가워요!").await?;
    println!("{:?}", morphs);
    // ["안녕", "하", "시", "어요", ",", "반갑", "어요", "!"]

    // Get nouns
    let nouns = tagger.nouns("나비 허리에 새파란 초생달이 시리다.").await?;
    println!("{:?}", nouns);
    // ["나비", "허리", "초생달"]

    // Get POS tags
    let pos_result = tagger.pos("햇빛이 선명하게 나뭇잎을 핥고 있었다.", true, false, false).await?;
    println!("{:?}", pos_result);

    Ok(())
}
```

### Tokenizer

```rust
use bareun_rs::Tokenizer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = "koba-ABCDEFG-1234567-LMNOPQR-7654321"; //sample :)
    let mut tokenizer = Tokenizer::new(api_key, "api.bareun.ai", Some(443)).await?;

    let segments = tokenizer.segments("안녕하세요, 반가워요!").await?;
    println!("{:?}", segments);

    let nouns = tokenizer.nouns("나비 허리에 새파란 초생달이 시리다.").await?;
    println!("{:?}", nouns);

    Ok(())
}
```

### Corrector (Spelling Correction)

```rust
use bareun_rs::Corrector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = "koba-ABCDEFG-1234567-LMNOPQR-7654321";

    // Corrector is only available via api.bareun.ai
    let mut corrector = Corrector::new(api_key, "api.bareun.ai", Some(443)).await?;

    let response = corrector
        .correct_error("영수 도 줄기가 얇어서 시들을 것 같은 꽃에물을 주었다.", &[], None)
        .await?;

    println!("Original: {}", response.origin);
    println!("Corrected: {}", response.revised);

    corrector.print_results(&response);

    Ok(())
}
```

### Custom Dictionaries

```rust
use bareun_rs::Tagger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = "koba-ABCDEFG-1234567-LMNOPQR-7654321";
    let mut tagger = Tagger::new(api_key, "api.bareun.ai", Some(443), vec![]).await?;

    // Get custom dictionary
    let cust_dic = tagger.custom_dict("my");
    cust_dic.copy_np_set(vec!["내고유명사".to_string(), "우리집고유명사".to_string()].into_iter().collect());
    cust_dic.copy_cp_set(vec!["코로나19".to_string()].into_iter().collect());
    cust_dic.copy_cp_caret_set(vec!["코로나^백신".to_string()].into_iter().collect());
    cust_dic.update();

    // Use the custom dictionary
    tagger.set_custom_dicts(vec!["my".to_string()]);
    let result = tagger.morphs("코로나19는 언제 끝날까요?").await?;
    println!("{:?}", result);

    Ok(())
}
```

## Links

- [Bareun AI](https://bareun.ai/)
- [Python Library](https://github.com/bareun-nlp/bareunpy)
