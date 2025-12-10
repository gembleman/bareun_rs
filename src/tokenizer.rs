use std::collections::HashMap;

use crate::bareun::{Segment, SegmentSentence, SegmentToken, TextSpan, TokenizeResponse};
use crate::lang_service_client::{BareunLanguageServiceClient, MAX_MESSAGE_LENGTH};
use serde::{Deserialize, Serialize};
// use grpcio::{ChannelBuilder, Environment};
use serde_json::json;
use tonic::transport::Endpoint;
pub enum SegResult {
    Flat(Vec<String>),
    Nested(Vec<Vec<String>>),
}
pub struct Tokenized {
    /**
    Tokenized result.
    It has various output manipulations.
    */
    phrase: String,
    r: TokenizeResponse,
}
impl Tokenized {
    /**
    constructor, which is used internally.
    :param phrase: requested sentences.
    :param res:
    */
    pub fn new(phrase: String, res: TokenizeResponse) -> Self {
        Tokenized { phrase, r: res }
    }
    /**
    Protobuf message object containing all of NLP engine.
    */
    pub fn msg(&self) -> &TokenizeResponse {
        &self.r
    }
    /**
    :return: get sentences from tagged results.
    */
    pub fn sentences(&self) -> Vec<SegmentSentence> {
        self.r.sentences.to_vec()
    }
    // json 기능은 굳이 넣지 않음.
    // /**
    // convert the message to a json object.
    // :return: Json Obejct
    // */
    // pub fn as_json(&self) -> serde_json::Value {
    //     serde_json::to_value(&self.r).unwrap()
    // }
    // /**
    // a json string representing analyzed sentences.
    // :return: json string
    // */
    // pub fn as_json_str(&self) -> String {
    //     serde_json::to_string(&self.r).unwrap()
    // }
    // /**
    // print the analysis result
    // :return: None
    // */
    // pub fn print_as_json(&self) {
    //     println!(
    //         "{}",
    //         serde_json::to_string_pretty(&self.r).unwrap()
    //     );
    // }

    fn _segment(m: &Segment, join: bool, detail: bool) -> String {
        if join {
            if detail {
                format!("{}/{}", m.text.clone().unwrap().content, m.hint)
            } else {
                m.text.clone().unwrap().content.to_string()
            }
        } else {
            if detail {
                format!("{},{}", m.text.clone().unwrap().content, m.hint)
            } else {
                m.text.clone().unwrap().content.to_string()
            }
        }
    }
    /**
    분절의 결과를 튜플 형태로 반환한다.
    :param flatten : If False, returns original morphs.
    :param join    : If True, returns joined sets of morph and tag.
    :param detail  : if True, returns everything of morph result
    */
    pub fn seg(&self, flatten: bool, join: bool, detail: bool) -> SegResult {
        if flatten {
            SegResult::Flat(
                self.r
                    .sentences
                    .iter()
                    .flat_map(|s| {
                        s.tokens.iter().flat_map(|token| {
                            token
                                .segments
                                .iter()
                                .map(|m| Tokenized::_segment(m, join, detail))
                        })
                    })
                    .collect(),
            )
        } else {
            SegResult::Nested(
                self.r
                    .sentences
                    .iter()
                    .map(|s| {
                        s.tokens
                            .iter()
                            .map(|token| {
                                token
                                    .segments
                                    .iter()
                                    .map(|m| Tokenized::_segment(m, join, detail))
                                    .collect()
                            })
                            .collect()
                    })
                    .collect(),
            )
        }
    }
    /**문장의 모든 segment들을 반환한다. */
    pub fn segments(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.segments)
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**체언을 추출한다.*/
    pub fn nouns(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.segments)
            .filter(|m| m.hint == "N")
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**동사 또는 형용사, 즉, 용언을 추출한다.*/
    pub fn verbs(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.segments)
            .filter(|m| m.hint == "V")
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**용언을 추출한다.*/
    pub fn predicates(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.segments)
            .filter(|m| m.hint == "V")
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**체언을 추출한다.*/
    pub fn substantives(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.segments)
            .filter(|m| m.hint == "N")
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**기호를 추출한다.*/
    pub fn symbols(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.segments)
            .filter(|m| m.hint == "S")
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**부사를 추출한다.*/
    pub fn adverbs(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.segments)
            .filter(|m| m.hint == "A")
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**관형사를 추출한다.*/
    pub fn prenouns(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.segments)
            .filter(|m| m.hint == "M")
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**감탄사를 추출한다.*/
    pub fn postpositions(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.segments)
            .filter(|m| m.hint == "J")
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**감탄사를 추출한다.*/
    pub fn interjections(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.segments)
            .filter(|m| m.hint == "I")
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**어미를 반환한다.*/
    pub fn endings(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.segments)
            .filter(|m| m.hint == "E")
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
}
pub struct Tokenizer {
    /**Wrapper for bareun v1.7.x <https://github.com/bareun-nlp>_.
    'bareun' is a morphological analyzer developed by Baikal AI, Inc. and Korea Press Foundation.
     .. code-block:: rust
         :emphasize-lines: 1
         >>> use bareunpy::Tokenizer;
         >>> let tokenizer = Tokenizer::new("YOUR_API_KEY", "HOST", 5656);
         >>> let segments = tokenizer.segments("안녕하세요, 반가워요.");
         >>> println!("{:?}", segments);
         ["안녕", "하", "시", "어요", ",", "반갑", "어요", "."]
         >>> let nouns = tokenizer.nouns("나비 허리에 새파란 초생달이 시리다.");
         >>> println!("{:?}", nouns);
         ["나비", "허리", "초생달"]
         >>> let seg_result = tokenizer.seg("햇빛이 선명하게 나뭇잎을 핥고 있었다.", true, false, false);
         >>> println!("{}", serde_json::to_string_pretty(&seg_result).unwrap());
         [
           {
             "text": "햇빛",
             "tag": "NNG"
           },
           {
             "text": "이",
             "tag": "JKS"
           },
           {
             "text": "선명",
             "tag": "NNG"
           },
           {
             "text": "하",
             "tag": "XSA"
           },
           {
             "text": "게",
             "tag": "EC"
           },
           {
             "text": "나뭇잎",
             "tag": "NNG"
           },
           {
             "text": "을",
             "tag": "JKO"
           },
           {
             "text": "핥",
             "tag": "VV"
           },
           {
             "text": "고",
             "tag": "EC"
           },
           {
             "text": "있",
             "tag": "VX"
           },
           {
             "text": "었",
             "tag": "EP"
           },
           {
             "text": "다",
             "tag": "EF"
           },
           {
             "text": ".",
             "tag": "SF"
           }
         ]
    :param host         : str. host name for bareun server
    :param port         : int. port  for bareun server
    */
    client: BareunLanguageServiceClient,
}
impl Tokenizer {
    pub async fn new(apikey: &str, host: &str, port: Option<i32>) -> Self {
        let host = host.trim();
        let host = if host.is_empty() {
            "nlp.bareun.ai"
        } else {
            host
        };
        let port = if port.is_none() { 5656 } else { port.unwrap() };

        if apikey.is_empty() {
            panic!("a apikey must be provided!");
        }

        // let endpoint = format!("http://{}:{}", host, port);
        // let channel = Endpoint::from_shared(endpoint)
        //     .unwrap()
        //     // .max_send_message_size(MAX_MESSAGE_LENGTH)
        //     // .max_receive_message_size(MAX_MESSAGE_LENGTH)
        //     .connect()
        //     .await
        //     .unwrap();

        let client = BareunLanguageServiceClient::new(apikey, &host, port).await;

        Tokenizer { client }
    }

    pub async fn tokenize(&mut self, phrase: &str, auto_split: bool) -> Tokenized {
        if phrase.is_empty() {
            eprintln!("OOPS, no sentences.");
            Tokenized::new(String::default(), TokenizeResponse::default())
        } else {
            Tokenized::new(
                phrase.to_string(),
                self.client.tokenize(phrase, auto_split).await.unwrap(),
            )
        }
    }
    /**
    tag string array.
    :param phrase: array of string
    :return: Tagged result instance
    */
    pub async fn tokenize_list(&mut self, phrase: &[String]) -> Tokenized {
        if phrase.is_empty() {
            eprintln!("OOPS, no sentences.");
            Tokenized::new(String::default(), TokenizeResponse::default())
        } else {
            let p = phrase.join("\n");
            Tokenized::new(p.clone(), self.client.tokenize(&p, false).await.unwrap())
        }
    }
    /**
    분절 하기,
    :param phrase  : string to analyse
    :param flatten : If False, returns original morphs.
    :param join    : If True, returns joined sets of morph and tag.
    :param detail  : if True, returns every things of morph result
    */
    pub async fn seg(
        &mut self,
        phrase: &str,
        flatten: bool,
        join: bool,
        detail: bool,
    ) -> SegResult {
        self.tokenize(phrase, false)
            .await
            .seg(flatten, join, detail)
    }
    /**문장을 분절하여 어절 내부의 기본 단위로 만들어 낸다.*/
    pub async fn segments(&mut self, phrase: &str) -> Vec<String> {
        self.tokenize(phrase, false).await.segments()
    }
    /**문장을 분절하여 어절 내부의 기본 단위로 만들어 내고 체언을 뽑아낸다.*/
    pub async fn nouns(&mut self, phrase: &str) -> Vec<String> {
        self.tokenize(phrase, false).await.nouns()
    }
    /**문장을 분절하여 어절 내부의 기본 단위로 만들어 내고 용언을 뽑아낸다.*/
    pub async fn verbs(&mut self, phrase: &str) -> Vec<String> {
        self.tokenize(phrase, false).await.verbs()
    }
}
