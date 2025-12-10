use crate::bareun::{
    CorrectErrorRequest, CorrectErrorResponse, Document, EncodingType, RevisionConfig,
};
use crate::error::Result;
use crate::revision_service_client::BareunRevisionServiceClient;

pub struct Corrector {
    pub client: BareunRevisionServiceClient,
}

impl Corrector {
    /// Corrector 초기화
    ///
    /// Args:
    ///     apikey: API 키
    ///     host: gRPC 서버 호스트 (기본값: api.bareun.ai)
    ///     port: gRPC 서버 포트 (기본값: 443)
    pub async fn new(apikey: &str, host: &str, port: Option<u16>) -> Result<Self> {
        let client = BareunRevisionServiceClient::new(apikey, host, port).await?;

        Ok(Corrector { client })
    }

    /// 맞춤법 교정 요청
    ///
    /// Args:
    ///     content: 교정을 요청할 문장
    ///     custom_dicts: 커스텀 도메인 정보
    ///     config: 요청 설정
    pub async fn correct_error(
        &mut self,
        content: &str,
        custom_dicts: &[String],
        config: Option<RevisionConfig>,
    ) -> Result<CorrectErrorResponse> {
        let mut request = CorrectErrorRequest {
            document: Some(Document {
                content: content.to_string(),
                language: "ko_KR".to_string(),
            }),
            encoding_type: EncodingType::Utf32.into(),
            #[allow(deprecated)]
            custom_domain: String::new(), // deprecated field
            custom_dict_names: custom_dicts.to_vec(),
            config: None,
        };

        if let Some(cfg) = config {
            request.config = Some(cfg);
        }

        self.client.correct_error(request).await
    }

    /// 교정 결과를 출력
    pub fn print_results(&self, res: &CorrectErrorResponse) {
        println!("원문: {}", res.origin);
        println!("교정: {}", res.revised);

        println!("\n=== 교정된 문장들 ===");
        for sent in &res.revised_sentences {
            println!(" 원문: {}", sent.origin);
            println!("교정문: {}", sent.revised);
        }

        for block in &res.revised_blocks {
            if let Some(origin) = &block.origin {
                println!(
                    "원문:{} offset:{}, length:{}",
                    origin.content, origin.begin_offset, origin.length
                );
            }
            println!("대표 교정: {}", block.revised);
            for rev in &block.revisions {
                let help_text = res
                    .helps
                    .get(&rev.help_id)
                    .map(|h| h.comment.as_str())
                    .unwrap_or("");
                println!(
                    " 교정: {}, 카테고리:{}, 도움말 {}",
                    rev.revised, rev.category, help_text
                );
            }
        }

        for cleanup in &res.whitespace_cleanup_ranges {
            println!(
                "공백제거: offset:{} length:{} position: {}",
                cleanup.offset, cleanup.length, cleanup.position
            );
        }
    }

    /// 교정 결과를 JSON 문자열로 변환
    ///
    /// Args:
    ///     response: 교정 결과
    ///
    /// Returns:
    ///     JSON 문자열
    pub fn as_json_str(&self, response: &CorrectErrorResponse) -> Result<String> {
        Ok(serde_json::to_string_pretty(response)?)
    }

    /// 교정 결과를 JSON 형식으로 출력
    ///
    /// Args:
    ///     response: 교정 결과
    pub fn print_as_json(&self, response: &CorrectErrorResponse) -> Result<()> {
        println!("{}", self.as_json_str(response)?);
        Ok(())
    }
}
