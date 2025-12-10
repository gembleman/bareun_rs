use std::collections::HashMap;

use crate::bareun::morpheme::{OutOfVocab, Tag};
use crate::bareun::{AnalyzeSyntaxResponse, Morpheme, Sentence};
use crate::custom_dict::CustomDict;
use crate::error::{BareunError, Result};
use crate::lang_service_client::BareunLanguageServiceClient;

pub struct Tagged {
    pub phrase: String,
    pub r: AnalyzeSyntaxResponse,
}

impl Tagged {
    pub fn new(phrase: String, res: AnalyzeSyntaxResponse) -> Self {
        Tagged { phrase, r: res }
    }

    pub fn phrase(&self) -> &str {
        &self.phrase
    }

    pub fn msg(&self) -> &AnalyzeSyntaxResponse {
        &self.r
    }

    pub fn sentences(&self) -> Vec<Sentence> {
        self.r.sentences.to_vec()
    }

    fn _pos(m: &Morpheme, join: bool, detail: bool) -> String {
        if join {
            if detail {
                let p = if m.probability > 0.0 {
                    format!(":{:5.3}", m.probability)
                } else {
                    String::new()
                };
                let oov = if m.out_of_vocab != OutOfVocab::InWordEmbedding as i32 {
                    format!("#{}", m.out_of_vocab().as_str_name())
                } else {
                    String::new()
                };
                format!(
                    "{}/{}{}{}",
                    m.text.as_ref().expect("text is empty").content,
                    m.tag().as_str_name(),
                    p,
                    oov
                )
            } else {
                format!(
                    "{}/{}",
                    m.text.as_ref().expect("text is empty").content,
                    m.tag().as_str_name()
                )
            }
        } else {
            if detail {
                format!(
                    "{}\t{}\t{}\t{}",
                    m.text.as_ref().expect("text is empty").content,
                    m.tag().as_str_name(),
                    m.out_of_vocab().as_str_name(),
                    m.probability
                )
            } else {
                format!(
                    "{}\t{}",
                    m.text.as_ref().expect("text is empty").content,
                    m.tag().as_str_name()
                )
            }
        }
    }

    pub fn pos(&self, flatten: bool, join: bool, detail: bool) -> Vec<Vec<String>> {
        if flatten {
            vec![
                self.r
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
                    .collect(),
            ]
        } else {
            self.r
                .sentences
                .iter()
                .map(|s| {
                    s.tokens
                        .iter()
                        .flat_map(|token| {
                            token
                                .morphemes
                                .iter()
                                .map(|m| Tagged::_pos(m, join, detail))
                        })
                        .collect()
                })
                .collect()
        }
    }

    /// 형태소를 추출합니다.
    ///
    /// # Returns
    ///
    /// 분석된 모든 형태소의 벡터
    pub fn morphs(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.morphemes)
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }

    /// 명사를 추출합니다.
    ///
    /// # Returns
    ///
    /// 분석된 모든 명사(고유명사, 일반명사, 대명사, 의존명사)의 벡터
    pub fn nouns(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.morphemes)
            .filter(|m| {
                m.tag == Tag::Nnp as i32
                    || m.tag == Tag::Nng as i32
                    || m.tag == Tag::Np as i32
                    || m.tag == Tag::Nnb as i32
            })
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }

    /// 동사를 추출합니다.
    ///
    /// # Returns
    ///
    /// 분석된 모든 동사의 벡터
    pub fn verbs(&self) -> Vec<String> {
        self.r
            .sentences
            .iter()
            .flat_map(|s| &s.tokens)
            .flat_map(|token| &token.morphemes)
            .filter(|m| m.tag == Tag::Vv as i32)
            .filter_map(|m| m.text.as_ref().map(|t| t.content.clone()))
            .collect()
    }

    /// 분석 결과를 JSON 문자열로 변환
    ///
    /// Returns:
    ///     JSON 문자열
    pub fn as_json_str(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.r)?)
    }

    /// 분석 결과를 JSON 형식으로 출력
    pub fn print_as_json(&self) -> Result<()> {
        println!("{}", self.as_json_str()?);
        Ok(())
    }
}

pub struct Tagger {
    client: BareunLanguageServiceClient,
    custom_dicts: Vec<String>,
    internal_custom_dicts: HashMap<String, CustomDict>,
    apikey: String,
    host: String,
    port: i32,
}

impl Tagger {
    pub async fn new(
        apikey: &str,
        host: &str,
        port: Option<u16>,
        custom_dicts: Vec<String>,
    ) -> Result<Self> {
        if apikey.is_empty() {
            return Err(BareunError::MissingApiKey);
        }

        let client = BareunLanguageServiceClient::new(apikey, host, port).await?;

        // Store connection info for CustomDict creation
        let host_str = if host.trim().is_empty() {
            "api.bareun.ai"
        } else {
            host.trim()
        };
        let port_i32 = port.unwrap_or(if host_str == "api.bareun.ai" {
            443
        } else {
            5656
        }) as i32;

        Ok(Tagger {
            client,
            custom_dicts,
            internal_custom_dicts: HashMap::new(),
            apikey: apikey.to_string(),
            host: host_str.to_string(),
            port: port_i32,
        })
    }

    pub fn set_custom_dicts(&mut self, custom_dicts: Vec<String>) {
        self.custom_dicts = custom_dicts;
    }

    pub fn custom_dict(&mut self, name: &str) -> &mut CustomDict {
        if name.is_empty() {
            panic!("invalid name for custom dict");
        }

        let apikey = self.apikey.clone();
        let host = self.host.clone();
        let port = self.port;

        self.internal_custom_dicts
            .entry(name.to_string())
            .or_insert_with(|| {
                let mut cd = CustomDict::new(name);
                cd.set_connection(&apikey, &host, port);
                cd
            })
    }

    pub async fn tag(
        &mut self,
        phrase: &str,
        auto_split: bool,
        auto_spacing: bool,
        auto_jointing: bool,
    ) -> Result<Tagged> {
        if phrase.is_empty() {
            eprintln!("OOPS, no sentences.");
            return Ok(Tagged::new(
                "".to_string(),
                AnalyzeSyntaxResponse::default(),
            ));
        }

        let res = self
            .client
            .analyze_syntax(
                phrase,
                &self.custom_dicts,
                auto_split,
                auto_spacing,
                auto_jointing,
            )
            .await?;

        Ok(Tagged::new(phrase.to_string(), res))
    }

    pub async fn tags(
        &mut self,
        phrase: &[String],
        auto_split: bool,
        auto_spacing: bool,
        auto_jointing: bool,
    ) -> Result<Tagged> {
        if phrase.is_empty() {
            eprintln!("OOPS, no sentences.");
            return Ok(Tagged::new(
                "".to_string(),
                AnalyzeSyntaxResponse::default(),
            ));
        }

        let p = phrase.join("\n");
        let res = self
            .client
            .analyze_syntax(
                &p,
                &self.custom_dicts,
                auto_split,
                auto_spacing,
                auto_jointing,
            )
            .await?;

        Ok(Tagged::new(p, res))
    }

    pub async fn taglist(
        &mut self,
        phrase: &[String],
        auto_spacing: bool,
        auto_jointing: bool,
    ) -> Result<Tagged> {
        if phrase.is_empty() {
            eprintln!("OOPS, no sentences.");
            return Ok(Tagged::new(
                "".to_string(),
                AnalyzeSyntaxResponse::default(),
            ));
        }

        let res = self
            .client
            .analyze_syntax_list(phrase, &self.custom_dicts, auto_spacing, auto_jointing)
            .await?;

        Ok(Tagged::new(
            phrase.join("\n"),
            AnalyzeSyntaxResponse {
                sentences: res.sentences.to_vec(),
                language: res.language,
                tokens_count: res.tokens_count,
            },
        ))
    }

    pub async fn pos(
        &mut self,
        phrase: &str,
        flatten: bool,
        join: bool,
        detail: bool,
    ) -> Result<Vec<Vec<String>>> {
        Ok(self
            .tag(phrase, false, true, false)
            .await?
            .pos(flatten, join, detail))
    }

    /// 문장을 분석하여 형태소를 추출합니다.
    ///
    /// # Arguments
    ///
    /// * `phrase` - 분석할 문장
    ///
    /// # Returns
    ///
    /// 분석된 모든 형태소의 벡터
    pub async fn morphs(&mut self, phrase: &str) -> Result<Vec<String>> {
        Ok(self.tag(phrase, false, true, false).await?.morphs())
    }

    /// 문장을 분석하여 명사를 추출합니다.
    ///
    /// # Arguments
    ///
    /// * `phrase` - 분석할 문장
    ///
    /// # Returns
    ///
    /// 분석된 모든 명사의 벡터
    pub async fn nouns(&mut self, phrase: &str) -> Result<Vec<String>> {
        Ok(self.tag(phrase, false, true, false).await?.nouns())
    }

    /// 문장을 분석하여 동사를 추출합니다.
    ///
    /// # Arguments
    ///
    /// * `phrase` - 분석할 문장
    ///
    /// # Returns
    ///
    /// 분석된 모든 동사의 벡터
    pub async fn verbs(&mut self, phrase: &str) -> Result<Vec<String>> {
        Ok(self.tag(phrase, false, true, false).await?.verbs())
    }
}
