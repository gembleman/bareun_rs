use std::collections::HashMap;

use crate::bareun::morpheme::{OutOfVocab, Tag};
use crate::bareun::{AnalyzeSyntaxResponse, Morpheme, Sentence};
use crate::custom_dict::CustomDict;
// use crate::custom_dict::CustomDict;
use crate::lang_service_client::{BareunLanguageServiceClient, MAX_MESSAGE_LENGTH};
// use grpcio::{ChannelBuilder, Environment};

// use serde_json::json;
use tonic::transport::Endpoint;
pub struct Tagged {
    /**
    Tagged result.
    It has various output manipulations.
    */
    phrase: String,
    r: AnalyzeSyntaxResponse,
}
impl Tagged {
    /**
    constructor, which is used internally.
    :param phrase: requested sentences.
    :param res:
    */
    pub fn new(phrase: String, res: AnalyzeSyntaxResponse) -> Self {
        Tagged { phrase, r: res }
    }
    /**
    Protobuf message object containing all of NLP engine.
    */
    pub fn msg(&self) -> &AnalyzeSyntaxResponse {
        &self.r
    }
    /**
    :return: get sentences from tagged results.
    */
    pub fn sentences(&self) -> Vec<Sentence> {
        self.r.sentences.to_vec()
    }
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
    //     println!("{}", serde_json::to_string_pretty(&self.r).unwrap());
    // }

    fn _pos(m: &Morpheme, join: bool, detail: bool) -> String {
        if join {
            if detail {
                let p = if m.probability > 0.0 {
                    format!(":{:5.3}", m.probability)
                } else {
                    String::new()
                };
                let oov = if m.out_of_vocab != OutOfVocab::OutOfVocab as i32 {
                    //OutOfVocab = 1
                    format!("#{}", m.out_of_vocab().as_str_name())
                } else {
                    String::new()
                };
                format!(
                    "{}/{}{}{}",
                    m.text.clone().expect("text is empty").content,
                    m.tag().as_str_name(),
                    p,
                    oov
                )
            } else {
                format!(
                    "{}/{}",
                    m.text.clone().expect("text is empty").content,
                    m.tag().as_str_name()
                )
            }
        } else {
            if detail {
                format!(
                    "{}\t{}\t{}\t{}",
                    m.text.clone().expect("text is empty").content,
                    m.tag().as_str_name(),
                    m.out_of_vocab().as_str_name(),
                    m.probability
                )
            } else {
                format!(
                    "{}\t{}",
                    m.text.clone().expect("text is empty").content,
                    m.tag().as_str_name()
                )
            }
        }
    }
    /**
    POS tagger to tuple.
    :param flatten : If False, returns original morphs.
    :param join    : If True, returns joined sets of morph and tag.
    :param detail  : if True, returns everything of morph result
    */
    pub fn pos(&self, flatten: bool, join: bool, detail: bool) -> Vec<Vec<String>> {
        if flatten {
            vec![self
                .r
                .sentences
                .iter()
                .flat_map(|s| {
                    s.tokens.iter().flat_map(|token| {
                        token
                            .morphemes
                            .iter()
                            .map(|m| Tagged::_pos(m, join, detail))
                    })
                })
                .collect()]
        } else {
            self.r
                .sentences
                .iter()
                .map(|s| {
                    s.tokens
                        .iter()
                        .map(|token| {
                            token
                                .morphemes
                                .iter()
                                .map(|m| Tagged::_pos(m, join, detail))
                                .collect()
                        })
                        .collect()
                })
                .collect()
        }
    }
    /**Parse phrase to morphemes. */
    pub fn morphs(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.morphemes)
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**Noun extractor.*/
    pub fn nouns(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.morphemes)
            .filter(|m| {
                m.tag == 25 //&Tag::NNP
                    || m.tag == 24//&Tag::NNG
                    || m.tag == 26//&Tag::NP
                    || m.tag == 23 //&Tag::NNB
            })
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
    /**Noun extractor.*/
    pub fn verbs(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| s.tokens.clone())
            .flat_map(|token| token.morphemes)
            .filter(|m| m.tag == 41) //Tag::Vv
            .map(|m| m.text.unwrap().content.to_string())
            .collect()
    }
}
/**Wrapper for bareun v1.7.x <https://github.com/bareun-nlp>_.
       'bareun' is a morphological analyzer developed by Baikal AI, Inc. and Korea Press Foundation.
        .. code-block:: rust
:emphasize-lines: 1
>>> use bareunpy::Tagger;
>>> let tagger = Tagger::new("YOUR_API_KEY", "HOST", 5656, "custom");
>>> let morphs = tagger.morphs("안녕하세요, 반가워요.");
>>> println!("{:?}", morphs);
["안녕", "하", "시", "어요", ",", "반갑", "어요", "."]
>>> let nouns = tagger.nouns("나비 허리에 새파란 초생달이 시리다.");
>>> println!("{:?}", nouns);
["나비", "허리", "초생달"]
>>> let pos_result = tagger.pos("햇빛이 선명하게 나뭇잎을 핥고 있었다.", true, false, false);
>>> println!("{}", serde_json::to_string_pretty(&pos_result).unwrap());
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
       :param domain       : custom domain name for analyzing request
       */
pub struct Tagger {
    client: BareunLanguageServiceClient,
    domain: String,
    custom_dicts: HashMap<String, CustomDict>,
}
impl Tagger {
    pub async fn new(apikey: &str, host: &str, port: Option<i32>, domain: &str) -> Self {
        let host = host.trim();
        let domain = domain.trim();
        let host = if host.is_empty() {
            "nlp.bareun.ai"
        } else {
            host
        };

        let port = if port.is_none() { 5656 } else { port.unwrap() };

        // let endpoint = format!("http://{}:{}", host, port);
        // let channel = Endpoint::from_shared(endpoint)
        //     .unwrap()
        //     // .max_send_message_size(MAX_MESSAGE_LENGTH)
        //     // .max_receive_message_size(MAX_MESSAGE_LENGTH)
        //     .connect()
        //     .await
        //     .unwrap();

        if apikey.is_empty() {
            panic!("a apikey must be provided!");
        }

        let client = BareunLanguageServiceClient::new(apikey, host, port).await;

        Tagger {
            client,
            domain: domain.to_string(),
            custom_dicts: std::collections::HashMap::default(),
        }
    }
    /**
    Set domain of custom dict.
    :param domain: domain name of custom dict
    */
    pub fn set_domain(&mut self, domain: &str) {
        self.domain = domain.to_string();
    }

    pub fn custom_dict(&mut self, domain: &str) -> &mut CustomDict {
        if domain.is_empty() {
            panic!("invalid domain name for custom dict");
        }

        self.custom_dicts
            .entry(domain.to_string())
            .or_insert_with(|| CustomDict::new(domain))
    }

    pub async fn tag(
        &mut self,
        phrase: &str,
        auto_split: bool,
        auto_spacing: bool,
        auto_jointing: bool,
    ) -> Tagged {
        if phrase.is_empty() {
            eprintln!("OOPS, no sentences.");
            Tagged::new("".to_string(), AnalyzeSyntaxResponse::default())
        } else {
            let res = self
                .client
                .analyze_syntax(
                    phrase,
                    &self.domain,
                    auto_split,
                    auto_spacing,
                    auto_jointing,
                )
                .await
                .unwrap();
            Tagged::new(phrase.to_string(), res)
        }
    }
    /**
    tag string array.
    :param phrase: array of string
    :param auto_split(bool, optional): Whether to automatically perform sentence split
    :param auto_spacing(bool, optional): Whether to automatically perform space insertion for typo correction
    :param auto_jointing(bool, optional): Whether to automatically perform word joining for typo correction
    :return: Tagged result instance
    */
    pub async fn tags(
        &mut self,
        phrase: &[String],
        auto_split: bool,
        auto_spacing: bool,
        auto_jointing: bool,
    ) -> Tagged {
        if phrase.is_empty() {
            eprintln!("OOPS, no sentences.");
            Tagged::new("".to_string(), AnalyzeSyntaxResponse::default())
        } else {
            let p = phrase.join("\n");
            let res = self
                .client
                .analyze_syntax(&p, &self.domain, auto_split, auto_spacing, auto_jointing)
                .await
                .unwrap();
            Tagged::new(p, res)
        }
    }
    /**
    the array is not being split and the input value is being returned as-is.
    :param phrase: array of string
    :param auto_spacing(bool, optional): Whether to automatically perform space insertion for typo correction
    :param auto_jointing(bool, optional): Whether to automatically perform word joining for typo correction
    :return: Tagged result instance
    */
    pub async fn taglist(
        &mut self,
        phrase: &[String],
        auto_spacing: bool,
        auto_jointing: bool,
    ) -> Tagged {
        if phrase.is_empty() {
            eprintln!("OOPS, no sentences.");
            Tagged::new("".to_string(), AnalyzeSyntaxResponse::default())
        } else {
            let res = self
                .client
                .analyze_syntax_list(phrase, &self.domain, auto_spacing, auto_jointing)
                .await
                .unwrap();

            Tagged::new(
                phrase.join("\n"),
                AnalyzeSyntaxResponse {
                    sentences: res.sentences.to_vec(),
                    language: res.language,
                },
            )
        }
    }
    /**
    POS tagger.
    :param phrase  : string to analyse
    :param flatten : If False, returns original morphs.
    :param join    : If True, returns joined sets of morph and tag.
    :param detail  : if True, returns every things of morph result
    */
    pub async fn pos(
        &mut self,
        phrase: &str,
        flatten: bool,
        join: bool,
        detail: bool,
    ) -> Vec<Vec<String>> {
        self.tag(phrase, false, true, false)
            .await
            .pos(flatten, join, detail)
    }
    /**Parse phrase to morphemes. */
    pub async fn morphs(&mut self, phrase: &str) -> Vec<String> {
        self.tag(phrase, false, true, false).await.morphs()
    }
    /**Noun extractor.*/
    pub async fn nouns(&mut self, phrase: &str) -> Vec<String> {
        self.tag(phrase, false, true, false).await.nouns()
    }
    /**Verbs extractor.*/
    pub async fn verbs(&mut self, phrase: &str) -> Vec<String> {
        self.tag(phrase, false, true, false).await.verbs()
    }
}
