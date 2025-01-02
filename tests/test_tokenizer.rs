#[cfg(test)]
mod tests {
    use bareun_rs::Tokenized;
    use bareun_rs::Tokenizer;

    const TEST_STR: &str = "오늘은 정말 추운 날이네요.";

    #[tokio::test]
    async fn test_tokenizer_seg_not_flatten() {
        let tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", Some(5757));
        assert_eq!(
            tokenizer.await.seg(TEST_STR, false),
            vec![
                vec!["오늘".to_string(), "은".to_string()],
                vec!["정말".to_string()],
                vec!["춥".to_string(), "ㄴ".to_string()],
                vec![
                    "날".to_string(),
                    "이".to_string(),
                    "네".to_string(),
                    "요".to_string(),
                    ".".to_string()
                ]
            ]
        );
    }

    #[test]
    fn test_tokenizer_seg_join() {
        let tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", 5757);
        assert_eq!(
            tokenizer.seg(TEST_STR, true, true),
            vec![
                "오늘/N".to_string(),
                "은/J".to_string(),
                "정말/A".to_string(),
                "춥/V".to_string(),
                "ㄴ/E".to_string(),
                "날/N".to_string(),
                "이/V".to_string(),
                "네/E".to_string(),
                "요/J".to_string(),
                "./S".to_string()
            ]
        );
    }

    #[test]
    fn test_tokenizer_seg_detail() {
        let tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", 5757);
        let temp = tokenizer.seg(TEST_STR, true);
        let temp2: Vec<(&str, &str)> = temp.iter().map(|t| (&t.0, &t.1)).collect();
        assert_eq!(
            temp2,
            vec![
                ("오늘", "N"),
                ("은", "J"),
                ("정말", "A"),
                ("춥", "V"),
                ("ㄴ", "E"),
                ("날", "N"),
                ("이", "V"),
                ("네", "E"),
                ("요", "J"),
                (".", "S")
            ]
        );
    }

    #[test]
    fn test_tokenizer_seg() {
        let tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", 5757);
        assert_eq!(
            tokenizer.seg(TEST_STR),
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
    }

    #[test]
    fn test_tokenizer_nouns() {
        let tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", 5757);
        assert_eq!(
            tokenizer.nouns(TEST_STR),
            vec!["오늘".to_string(), "날".to_string()]
        );
    }

    #[test]
    fn test_tokenized_nouns() {
        let tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", 5757);
        let tokenized = tokenizer.tokenize(TEST_STR);
        assert_eq!(
            tokenized.nouns(),
            vec!["오늘".to_string(), "날".to_string()]
        );
    }

    #[test]
    fn test_tokenized_verbs() {
        let tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", 5757);
        let tokenized = tokenizer.tokenize(TEST_STR);
        assert_eq!(
            tokenized.predicates(),
            vec!["춥".to_string(), "이".to_string()]
        );
    }

    #[test]
    fn test_tokenized_symbols() {
        let tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", 5757);
        let tokenized = tokenizer.tokenize(TEST_STR);
        assert_eq!(tokenized.symbols(), vec![".".to_string()]);
    }

    #[test]
    fn test_tokenized_adverbs() {
        let tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", 5757);
        let tokenized = tokenizer.tokenize(TEST_STR);
        assert_eq!(tokenized.adverbs(), vec!["정말".to_string()]);
    }

    #[test]
    fn test_tokenized_endings() {
        let tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", 5757);
        let tokenized = tokenizer.tokenize(TEST_STR);
        assert_eq!(
            tokenized.endings(),
            vec!["ㄴ".to_string(), "네".to_string()]
        );
    }

    #[test]
    fn test_tokenized_postpositions() {
        let tokenizer = Tokenizer::new("appppppiiii", "127.0.0.1", 5757);
        let tokenized = tokenizer.tokenize(TEST_STR);
        assert_eq!(
            tokenized.postpositions(),
            vec!["은".to_string(), "요".to_string()]
        );
    }
}
