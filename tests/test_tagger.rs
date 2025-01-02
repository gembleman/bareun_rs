#[cfg(test)]
mod tests {
    use bareun_rs::Tagger;

    #[tokio::test]
    async fn test_tagger_pos() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), "").await;
        let sample1 = "오늘은 정말 추운 날이네요.";
        let result = tagger.pos(sample1, false, false, false).await;
        println!("{:?}", result);
        // assert_eq!(
        //     result,
        //     vec![
        //         ("오늘".to_string(), "NNG".to_string()),
        //         ("은".to_string(), "JX".to_string()),
        //         ("정말".to_string(), "MAG".to_string()),
        //         ("춥".to_string(), "VA".to_string()),
        //         ("ㄴ".to_string(), "ETM".to_string()),
        //         ("날".to_string(), "NNG".to_string()),
        //         ("이".to_string(), "VCP".to_string()),
        //         ("네".to_string(), "EF".to_string()),
        //         ("요".to_string(), "JX".to_string()),
        //         (".".to_string(), "SF".to_string())
        //     ]
        // );
    }

    #[tokio::test]
    async fn test_tagger_pos_join() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), "").await;
        let sample1 = "오늘은 정말 추운 날이네요.";
        let result = tagger.pos(sample1, false, true, false).await;
        println!("{:?}", result);
        // assert_eq!(
        //     result,
        //     vec![
        //         "오늘/NNG",
        //         "은/JX",
        //         "정말/MAG",
        //         "춥/VA",
        //         "ㄴ/ETM",
        //         "날/NNG",
        //         "이/VCP",
        //         "네/EF",
        //         "요/JX",
        //         "./SF"
        //     ]
        // );
    }

    #[tokio::test]
    async fn test_tagger_pos_detail() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), "").await;
        let sample1 = "오늘은 정말 추운 날이네요.";
        let result = tagger.pos(sample1, false, false, true).await;
        println!("{:?}", result);
        // let temp2: Vec<(String, String, String)> = result
        //     .into_iter()
        //     .map(|(t0, t1, t2, _)| (t0, t1, t2))
        //     .collect();
        // assert_eq!(
        //     temp2,
        //     vec![
        //         (
        //             "오늘".to_string(),
        //             "NNG".to_string(),
        //             "IN_WORD_EMBEDDING".to_string()
        //         ),
        //         (
        //             "은".to_string(),
        //             "JX".to_string(),
        //             "IN_WORD_EMBEDDING".to_string()
        //         ),
        //         (
        //             "정말".to_string(),
        //             "MAG".to_string(),
        //             "IN_WORD_EMBEDDING".to_string()
        //         ),
        //         (
        //             "춥".to_string(),
        //             "VA".to_string(),
        //             "IN_WORD_EMBEDDING".to_string()
        //         ),
        //         (
        //             "ㄴ".to_string(),
        //             "ETM".to_string(),
        //             "IN_WORD_EMBEDDING".to_string()
        //         ),
        //         (
        //             "날".to_string(),
        //             "NNG".to_string(),
        //             "IN_WORD_EMBEDDING".to_string()
        //         ),
        //         (
        //             "이".to_string(),
        //             "VCP".to_string(),
        //             "IN_WORD_EMBEDDING".to_string()
        //         ),
        //         (
        //             "네".to_string(),
        //             "EF".to_string(),
        //             "IN_WORD_EMBEDDING".to_string()
        //         ),
        //         (
        //             "요".to_string(),
        //             "JX".to_string(),
        //             "IN_WORD_EMBEDDING".to_string()
        //         ),
        //         (
        //             ".".to_string(),
        //             "SF".to_string(),
        //             "IN_WORD_EMBEDDING".to_string()
        //         )
        //     ]
        // );
    }

    #[tokio::test]
    async fn test_tagger_morphs() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), "").await;
        let sample1 = "오늘은 정말 추운 날이네요.";
        let result = tagger.morphs(sample1).await;
        assert_eq!(
            result,
            vec!["오늘", "은", "정말", "춥", "ㄴ", "날", "이", "네", "요", "."]
        );
    }

    #[tokio::test]
    async fn test_tagger_nouns() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), "").await;
        let sample1 = "오늘은 정말 추운 날이네요.";
        let result = tagger.nouns(sample1).await;
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
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), "").await;
        let sample1 = "오늘은 정말 추운 날이네요.";
        let m = tagger.tag(sample1, false, true, false).await.msg().clone();
        assert_eq!(
            m.sentences[0].tokens[3].tagged,
            "날/NNG+이/VCP+네/EF+요/JX+./SF"
        );
    }

    #[tokio::test]
    async fn test_tagger_create_custom_dict() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), "").await;
        let cd = tagger.custom_dict("my");
        assert!(cd.domain == "my");
    }

    #[tokio::test]
    async fn test_tagger_update_custom_dict() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), "").await;
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
        let result = cd.update();
        assert!(result);
    }

    #[tokio::test]
    async fn test_tagger_get_custom_dict_np_set() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), "").await;
        let dic = tagger.custom_dict("my");
        println!("{:?}", dic.np_set);
        assert_eq!(dic.np_set.len(), 4);
        assert!(dic.np_set.contains("유리왕"));
        assert!(dic.np_set.contains("근초고왕"));
        assert!(dic.np_set.contains("누루하치"));
        assert!(dic.np_set.contains("베링거인겔하임"));
    }

    #[tokio::test]
    async fn test_tagger_get_custom_dict_cp_set() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), "").await;
        let dic = tagger.custom_dict("my");
        println!(
            "{:?}, {:?}, {:?}, {:?},",
            dic.cp_set, dic.cp_caret_set, dic.vv_set, dic.va_set
        );
        assert_eq!(dic.cp_set.len(), 1);
        assert!(dic.cp_set.contains("코로나19"));
    }

    #[tokio::test]
    async fn test_tagger_get_custom_dict_cp_caret_set() {
        let mut tagger = Tagger::new("appppppiiii", "127.0.0.1", Some(5656), "").await;
        let dic = tagger.custom_dict("my");

        assert_eq!(dic.cp_caret_set.len(), 2);
        assert!(dic.cp_caret_set.contains("인공지능^데이터^학습"));
        assert!(dic.cp_caret_set.contains("자연어^처리^엔진"));
    }
}
