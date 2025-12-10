#[cfg(test)]
mod tests {
    use bareun_rs::Tagger;

    #[tokio::test]
    async fn test_tagger_pos() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), vec![])
            .await
            .unwrap();
        let sample1 = "오늘은 정말 추운 날이네요.";
        let result = tagger.pos(sample1, true, false, false).await.unwrap();
        // flatten=true이므로 단일 벡터로 반환됨
        assert_eq!(result.len(), 1);
        let flat_result: Vec<&str> = result[0]
            .iter()
            .map(|s| s.as_str())
            .collect();

        // 탭으로 구분된 형태소\t품사 형식을 파싱
        let parsed: Vec<(&str, &str)> = flat_result
            .iter()
            .map(|s| {
                let parts: Vec<&str> = s.split('\t').collect();
                (parts[0], parts[1])
            })
            .collect();

        assert_eq!(
            parsed,
            vec![
                ("오늘", "NNG"),
                ("은", "JX"),
                ("정말", "MAG"),
                ("춥", "VA"),
                ("ㄴ", "ETM"),
                ("날", "NNG"),
                ("이", "VCP"),
                ("네", "EF"),
                ("요", "JX"),
                (".", "SF")
            ]
        );
    }

    #[tokio::test]
    async fn test_tagger_pos_join() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), vec![])
            .await
            .unwrap();
        let sample1 = "오늘은 정말 추운 날이네요.";
        let result = tagger.pos(sample1, true, true, false).await.unwrap();
        // flatten=true이므로 단일 벡터로 반환됨
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            vec![
                "오늘/NNG",
                "은/JX",
                "정말/MAG",
                "춥/VA",
                "ㄴ/ETM",
                "날/NNG",
                "이/VCP",
                "네/EF",
                "요/JX",
                "./SF"
            ]
        );
    }

    #[tokio::test]
    async fn test_tagger_pos_detail() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), vec![])
            .await
            .unwrap();
        let sample1 = "오늘은 정말 추운 날이네요.";
        let result = tagger.pos(sample1, true, false, true).await.unwrap();
        // flatten=true이므로 단일 벡터로 반환됨
        assert_eq!(result.len(), 1);
        let flat_result: Vec<&str> = result[0]
            .iter()
            .map(|s| s.as_str())
            .collect();

        // 탭으로 구분된 형태소\t품사\tOOV\t확률 형식을 파싱
        let temp2: Vec<(&str, &str, &str)> = flat_result
            .iter()
            .map(|s| {
                let parts: Vec<&str> = s.split('\t').collect();
                (parts[0], parts[1], parts[2])
            })
            .collect();

        assert_eq!(
            temp2,
            vec![
                ("오늘", "NNG", "IN_WORD_EMBEDDING"),
                ("은", "JX", "IN_WORD_EMBEDDING"),
                ("정말", "MAG", "IN_WORD_EMBEDDING"),
                ("춥", "VA", "IN_WORD_EMBEDDING"),
                ("ㄴ", "ETM", "IN_WORD_EMBEDDING"),
                ("날", "NNG", "IN_WORD_EMBEDDING"),
                ("이", "VCP", "IN_WORD_EMBEDDING"),
                ("네", "EF", "IN_WORD_EMBEDDING"),
                ("요", "JX", "IN_WORD_EMBEDDING"),
                (".", "SF", "IN_WORD_EMBEDDING")
            ]
        );
    }

    #[tokio::test]
    async fn test_tagger_morphs() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), vec![])
            .await
            .unwrap();
        let sample1 = "오늘은 정말 추운 날이네요.";
        let result = tagger.morphs(sample1).await.unwrap();
        assert_eq!(
            result,
            vec![
                "오늘", "은", "정말", "춥", "ㄴ", "날", "이", "네", "요", "."
            ]
        );
    }

    #[tokio::test]
    async fn test_tagger_nouns() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), vec![])
            .await
            .unwrap();
        let sample1 = "오늘은 정말 추운 날이네요.";
        let result = tagger.nouns(sample1).await.unwrap();
        assert_eq!(result, vec!["오늘", "날"]);
    }

    // #[tokio::test]
    // async fn test_tagger_tag_as_json_str() {
    //     let tagger = Tagger::new(
    //         "appppppiiii",
    //         "127.0.0.1",
    //         Some(5656),
    //         "",
    //     )
    //     .await;
    //     let sample1 = "오늘은 정말 추운 날이네요.";
    //     let j = tagger.tag(sample1, false, true, false).await.as_json();
    //     assert_eq!(j["sentences"].as_array().unwrap().len(), 1);
    //     assert_eq!(j["sentences"][0]["tokens"].as_array().unwrap().len(), 4);
    //     assert_eq!(
    //         j["sentences"][0]["tokens"][0]["morphemes"]
    //             .as_array()
    //             .unwrap()
    //             .len(),
    //         2
    //     );
    //     assert_eq!(
    //         j["sentences"][0]["tokens"][1]["morphemes"]
    //             .as_array()
    //             .unwrap()
    //             .len(),
    //         1
    //     );
    //     assert_eq!(
    //         j["sentences"][0]["tokens"][2]["morphemes"]
    //             .as_array()
    //             .unwrap()
    //             .len(),
    //         2
    //     );
    //     assert_eq!(
    //         j["sentences"][0]["tokens"][3]["morphemes"]
    //             .as_array()
    //             .unwrap()
    //             .len(),
    //         5
    //     );
    //     assert_eq!(
    //         j["sentences"][0]["tokens"][3]["morphemes"]
    //             .as_array()
    //             .unwrap()
    //             .len(),
    //         5
    //     );
    // }
    #[tokio::test]
    async fn test_tagger_tag_as_msg() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), vec![])
            .await
            .unwrap();
        let sample1 = "오늘은 정말 추운 날이네요.";
        let m = tagger
            .tag(sample1, false, true, false)
            .await
            .unwrap()
            .msg()
            .clone();
        assert_eq!(
            m.sentences[0].tokens[3].tagged,
            "날/NNG+이/VCP+네/EF+요/JX+./SF"
        );
    }

    #[tokio::test]
    async fn test_tagger_create_custom_dict() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), vec![])
            .await
            .unwrap();
        let cd = tagger.custom_dict("my");
        assert!(cd.domain == "my");
    }

    #[tokio::test]
    async fn test_tagger_update_custom_dict() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), vec![])
            .await
            .unwrap();
        let cd = tagger.custom_dict("my");
        cd.copy_np_set(
            vec![
                "유리왕".to_string(),
                "근초고왕".to_string(),
                "누루하치".to_string(),
                "베링거인겔하임".to_string(),
            ]
            .into_iter()
            .collect(),
        );
        cd.copy_cp_set(vec!["코로나19".to_string()].into_iter().collect());
        cd.copy_cp_caret_set(
            vec![
                "인공지능^데이터^학습".to_string(),
                "자연어^처리^엔진".to_string(),
            ]
            .into_iter()
            .collect(),
        );
        let result = cd.update().await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_tagger_get_custom_dict_np_set() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), vec![])
            .await
            .unwrap();
        let dic = tagger.custom_dict("my");
        dic.load().await.unwrap();
        println!("{:?}", dic.np_set);
        assert_eq!(dic.np_set.len(), 4);
        assert!(dic.np_set.contains("유리왕"));
        assert!(dic.np_set.contains("근초고왕"));
        assert!(dic.np_set.contains("누루하치"));
        assert!(dic.np_set.contains("베링거인겔하임"));
    }

    #[tokio::test]
    async fn test_tagger_get_custom_dict_cp_set() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), vec![])
            .await
            .unwrap();
        let dic = tagger.custom_dict("my");
        dic.load().await.unwrap();
        println!(
            "{:?}, {:?}, {:?}, {:?},",
            dic.cp_set, dic.cp_caret_set, dic.vv_set, dic.va_set
        );
        assert_eq!(dic.cp_set.len(), 1);
        assert!(dic.cp_set.contains("코로나19"));
    }

    #[tokio::test]
    async fn test_tagger_get_custom_dict_cp_caret_set() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), vec![])
            .await
            .unwrap();
        let dic = tagger.custom_dict("my");
        dic.load().await.unwrap();

        assert_eq!(dic.cp_caret_set.len(), 2);
        assert!(dic.cp_caret_set.contains("인공지능^데이터^학습"));
        assert!(dic.cp_caret_set.contains("자연어^처리^엔진"));
    }

    #[tokio::test]
    #[ignore] // 로컬 테스트 서버는 API 키 검증을 하지 않으므로 실제 서버 테스트 시에만 실행
    async fn test_exception_apikey() {
        // 잘못된 API 키로 연결 시도
        let mut tagger = Tagger::new(
            "invalid-api-key",
            "api.bareun.ai",
            Some(443),
            vec![],
        )
        .await
        .unwrap(); // 연결 자체는 성공할 수 있음

        let sample1 = "오늘은 정말 추운 날이네요.";
        // API 키가 잘못되면 실제 요청 시 에러 발생
        let result = tagger.pos(sample1, true, false, false).await;

        // 잘못된 API 키로 인한 에러 발생 확인
        assert!(result.is_err(), "Expected error for invalid API key during request");
    }

    #[tokio::test]
    async fn test_exception_host() {
        let tagger_result = Tagger::new(
            "appppppiiii",
            "127.0.0.1:5656",
            Some(5656),
            vec![],
        )
        .await;

        // 잘못된 호스트 형식으로 인한 연결 에러 확인
        assert!(tagger_result.is_err(), "Expected error for invalid host format");
    }
}
