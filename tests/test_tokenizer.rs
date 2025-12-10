#[cfg(test)]
mod tests {
    use bareun_rs::{SegResult, Tokenizer};

    const TEST_STR: &str = "오늘은 정말 추운 날이네요.";

    #[tokio::test]
    async fn test_tokenizer_seg_not_flatten() {
        let mut tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5656))
            .await
            .unwrap();
        let result = tokenizer.seg(TEST_STR, false, false, false).await.unwrap();
        if let bareun_rs::SegResult::Nested(nested) = result {
            // 실제 서버 응답 구조 확인
            println!("Nested result: {:?}", nested);

            // 예상 결과
            let expected = vec![vec![
                "오늘은".to_string(),
                "정말".to_string(),
                "춥ㄴ".to_string(),
                "날이네요.".to_string(),
            ]];

            assert_eq!(nested, expected, "Nested segmentation mismatch");
        } else {
            panic!("Expected Nested result");
        }
    }

    #[tokio::test]
    async fn test_tokenizer_seg_join() {
        let mut tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5656))
            .await
            .unwrap();
        let result = tokenizer.seg(TEST_STR, true, true, false).await.unwrap();
        if let SegResult::Flat(flat) = result {
            println!("Flat result with join: {:?}", flat);

            // 예상 결과: join=true이면 "형태소/힌트" 형식으로 결합됨
            let expected = vec![
                "오늘/N".to_string(),
                "은/J".to_string(),
                "정말/A".to_string(),
                "춥/V".to_string(),
                "ㄴ/E".to_string(),
                "날/N".to_string(),
                "이/V".to_string(),
                "네/E".to_string(),
                "요/J".to_string(),
                "./S".to_string(),
            ];

            assert_eq!(flat, expected, "Joined segmentation mismatch");
        } else {
            panic!("Expected Flat result");
        }
    }

    #[tokio::test]
    async fn test_tokenizer_seg_detail() {
        let mut tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5656))
            .await
            .unwrap();
        let result = tokenizer.seg(TEST_STR, true, false, true).await.unwrap();
        if let SegResult::Flat(flat) = result {
            assert_eq!(
                flat,
                vec![
                    "오늘,N".to_string(),
                    "은,J".to_string(),
                    "정말,A".to_string(),
                    "춥,V".to_string(),
                    "ㄴ,E".to_string(),
                    "날,N".to_string(),
                    "이,V".to_string(),
                    "네,E".to_string(),
                    "요,J".to_string(),
                    ".,S".to_string()
                ]
            );
        } else {
            panic!("Expected Flat result");
        }
    }

    #[tokio::test]
    async fn test_tokenizer_seg() {
        let mut tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5656))
            .await
            .unwrap();
        let result = tokenizer.seg(TEST_STR, true, false, false).await.unwrap();
        if let SegResult::Flat(flat) = result {
            assert_eq!(
                flat,
                vec![
                    "오늘".to_string(),
                    "은".to_string(),
                    "정말".to_string(),
                    "춥".to_string(),
                    "ㄴ".to_string(),
                    "날".to_string(),
                    "이".to_string(),
                    "네".to_string(),
                    "요".to_string(),
                    ".".to_string()
                ]
            );
        } else {
            panic!("Expected Flat result");
        }
    }

    #[tokio::test]
    async fn test_tokenizer_nouns() {
        let mut tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5656))
            .await
            .unwrap();
        let result = tokenizer.nouns(TEST_STR).await.unwrap();
        assert_eq!(result, vec!["오늘".to_string(), "날".to_string()]);
    }

    #[tokio::test]
    async fn test_tokenized_nouns() {
        let mut tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5656))
            .await
            .unwrap();
        let tokenized = tokenizer.tokenize(TEST_STR, false).await.unwrap();
        assert_eq!(
            tokenized.nouns(),
            vec!["오늘".to_string(), "날".to_string()]
        );
    }

    #[tokio::test]
    async fn test_tokenized_verbs() {
        let mut tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5656))
            .await
            .unwrap();
        let tokenized = tokenizer.tokenize(TEST_STR, false).await.unwrap();
        assert_eq!(
            tokenized.predicates(),
            vec!["춥".to_string(), "이".to_string()]
        );
    }

    #[tokio::test]
    async fn test_tokenized_symbols() {
        let mut tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5656))
            .await
            .unwrap();
        let tokenized = tokenizer.tokenize(TEST_STR, false).await.unwrap();
        assert_eq!(tokenized.symbols(), vec![".".to_string()]);
    }

    #[tokio::test]
    async fn test_tokenized_adverbs() {
        let mut tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5656))
            .await
            .unwrap();
        let tokenized = tokenizer.tokenize(TEST_STR, false).await.unwrap();
        assert_eq!(tokenized.adverbs(), vec!["정말".to_string()]);
    }

    #[tokio::test]
    async fn test_tokenized_endings() {
        let mut tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5656))
            .await
            .unwrap();
        let tokenized = tokenizer.tokenize(TEST_STR, false).await.unwrap();
        assert_eq!(
            tokenized.endings(),
            vec!["ㄴ".to_string(), "네".to_string()]
        );
    }

    #[tokio::test]
    async fn test_tokenized_postpositions() {
        let mut tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5656))
            .await
            .unwrap();
        let tokenized = tokenizer.tokenize(TEST_STR, false).await.unwrap();
        assert_eq!(
            tokenized.postpositions(),
            vec!["은".to_string(), "요".to_string()]
        );
    }

    #[tokio::test]
    #[ignore] // 로컬 테스트 서버는 API 키 검증을 하지 않으므로 실제 서버 테스트 시에만 실행
    async fn test_exception_apikey_tokenizer() {
        // 잘못된 API 키로 연결 시도
        let mut tokenizer = Tokenizer::new(
            "invalid-api-key",
            "10.3.8.44",
            Some(5757),
        )
        .await
        .unwrap(); // 연결 자체는 성공할 수 있음

        // API 키가 잘못되면 실제 요청 시 에러 발생
        let result = tokenizer.seg(TEST_STR, true, false, false).await;

        // 잘못된 API 키로 인한 에러 발생 확인
        assert!(result.is_err(), "Expected error for invalid API key during request");
    }

    #[tokio::test]
    async fn test_exception_host_tokenizer() {
        let tokenizer_result = Tokenizer::new(
            "appppppiiii",
            "127.0.0.1:5656",
            Some(5656),
        )
        .await;

        // 잘못된 호스트 형식으로 인한 연결 에러 확인
        assert!(tokenizer_result.is_err(), "Expected error for invalid host format");
    }
}
