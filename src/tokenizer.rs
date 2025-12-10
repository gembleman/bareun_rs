use crate::bareun::{Segment, SegmentSentence, TokenizeResponse};
use crate::error::Result;
use crate::lang_service_client::BareunLanguageServiceClient;
pub enum SegResult {
    Flat(Vec<String>),
    Nested(Vec<Vec<String>>),
}
pub struct Tokenized {
    /**
    Tokenized result.
    It has various output manipulations.
    */
    pub phrase: String,
    pub r: TokenizeResponse,
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
    Get the original phrase that was tokenized.
    */
    pub fn phrase(&self) -> &str {
        &self.phrase
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

    fn _segment(m: &Segment, join: bool, detail: bool) -> String {
        let content = m.text.clone().unwrap().content;
        if join {
            format!("{}/{}", content, m.hint)
        } else {
            if detail {
                format!("{},{}", content, m.hint)
            } else {
                content
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
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.segments)
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }
    /**체언을 추출한다.*/
    pub fn nouns(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.segments)
            .filter(|m| m.hint == "N")
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }
    /**동사 또는 형용사, 즉, 용언을 추출한다.*/
    pub fn verbs(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.segments)
            .filter(|m| m.hint == "V")
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }
    /**용언을 추출한다.*/
    pub fn predicates(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.segments)
            .filter(|m| m.hint == "V")
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }
    /**체언을 추출한다.*/
    pub fn substantives(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.segments)
            .filter(|m| m.hint == "N")
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }
    /**기호를 추출한다.*/
    pub fn symbols(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.segments)
            .filter(|m| m.hint == "S")
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }
    /**부사를 추출한다.*/
    pub fn adverbs(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.segments)
            .filter(|m| m.hint == "A")
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }
    /**관형사를 추출한다.*/
    pub fn prenouns(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.segments)
            .filter(|m| m.hint == "M")
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }
    /**감탄사를 추출한다.*/
    pub fn postpositions(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.segments)
            .filter(|m| m.hint == "J")
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }
    /**감탄사를 추출한다.*/
    pub fn interjections(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.segments)
            .filter(|m| m.hint == "I")
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }
    /**어미를 반환한다.*/
    pub fn endings(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.segments)
            .filter(|m| m.hint == "E")
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }

    /// 토큰화 결과를 JSON 문자열로 변환
    ///
    /// Returns:
    ///     JSON 문자열
    pub fn as_json_str(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.r)?)
    }

    /// 토큰화 결과를 JSON 형식으로 출력
    pub fn print_as_json(&self) -> Result<()> {
        println!("{}", self.as_json_str()?);
        Ok(())
    }
}
/// Wrapper for bareun v1.7.x <https://github.com/bareun-nlp>
///
/// 'bareun' is a morphological analyzer developed by Baikal AI, Inc. and Korea Press Foundation.
///
/// # Examples
///
/// ```rust
/// use bareun_rs::Tokenizer;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let tokenizer = Tokenizer::new("YOUR_API_KEY", "api.bareun.ai", Some(443)).await?;
///     let segments = tokenizer.segments("안녕하세요, 반가워요.").await?;
///     println!("{:?}", segments);
///     // ["안녕", "하", "시", "어요", ",", "반갑", "어요", "."]
///
///     let nouns = tokenizer.nouns("나비 허리에 새파란 초생달이 시리다.").await?;
///     println!("{:?}", nouns);
///     // ["나비", "허리", "초생달"]
///
///     Ok(())
/// }
/// ```
pub struct Tokenizer {
    pub client: BareunLanguageServiceClient,
}
impl Tokenizer {
    pub async fn new(apikey: &str, host: &str, port: Option<u16>) -> Result<Self> {
        if apikey.is_empty() {
            return Err(crate::error::BareunError::MissingApiKey);
        }

        let client = BareunLanguageServiceClient::new(apikey, host, port).await?;

        Ok(Tokenizer { client })
    }

    pub async fn tokenize(&mut self, phrase: &str, auto_split: bool) -> Result<Tokenized> {
        if phrase.is_empty() {
            eprintln!("OOPS, no sentences.");
            return Ok(Tokenized::new(
                String::default(),
                TokenizeResponse::default(),
            ));
        }

        let res = self.client.tokenize(phrase, auto_split).await?;
        Ok(Tokenized::new(phrase.to_string(), res))
    }
    /**
    tag string array.
    :param phrase: array of string
    :return: Tagged result instance
    */
    pub async fn tokenize_list(&mut self, phrase: &[String]) -> Result<Tokenized> {
        if phrase.is_empty() {
            eprintln!("OOPS, no sentences.");
            return Ok(Tokenized::new(
                String::default(),
                TokenizeResponse::default(),
            ));
        }

        let p = phrase.join("\n");
        let res = self.client.tokenize(&p, false).await?;
        Ok(Tokenized::new(p, res))
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
    ) -> Result<SegResult> {
        Ok(self
            .tokenize(phrase, false)
            .await?
            .seg(flatten, join, detail))
    }

    /**문장을 분절하여 어절 내부의 기본 단위로 만들어 낸다.*/
    pub async fn segments(&mut self, phrase: &str) -> Result<Vec<String>> {
        Ok(self.tokenize(phrase, false).await?.segments())
    }

    /**문장을 분절하여 어절 내부의 기본 단위로 만들어 내고 체언을 뽑아낸다.*/
    pub async fn nouns(&mut self, phrase: &str) -> Result<Vec<String>> {
        Ok(self.tokenize(phrase, false).await?.nouns())
    }

    /**문장을 분절하여 어절 내부의 기본 단위로 만들어 내고 용언을 뽑아낸다.*/
    pub async fn verbs(&mut self, phrase: &str) -> Result<Vec<String>> {
        Ok(self.tokenize(phrase, false).await?.verbs())
    }
}
