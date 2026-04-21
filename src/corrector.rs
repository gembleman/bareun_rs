use crate::bareun::stream_correct_error_response::Res as StreamRes;
use crate::bareun::{
    CancelledRevision, CorrectErrorRequest, CorrectErrorResponse, Document, EncodingType,
    PostRevision, ProgressRevision, RevisionConfig, StreamCorrectErrorRequest,
    StreamCorrectErrorResponse, StreamFirstCorrectError,
};
use crate::error::Result;
use crate::revision_service_client::BareunRevisionServiceClient;
use tonic::Streaming;

/// RevisionConfig 편의 빌더
///
/// proto에서 추가된 `disable_confusion`, `disable_typo_correction`을 포함한
/// 모든 옵션을 체이닝 형태로 구성한다.
#[derive(Default, Clone)]
pub struct RevisionConfigBuilder {
    config: RevisionConfig,
}

impl RevisionConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn disable_split_sentence(mut self, v: bool) -> Self {
        self.config.disable_split_sentence = v;
        self
    }

    pub fn disable_caret_spacing(mut self, v: bool) -> Self {
        self.config.disable_caret_spacing = v;
        self
    }

    pub fn disable_vx_spacing(mut self, v: bool) -> Self {
        self.config.disable_vx_spacing = v;
        self
    }

    pub fn treat_as_title(mut self, v: bool) -> Self {
        self.config.treat_as_title = v;
        self
    }

    pub fn enable_limited_punctuation(mut self, v: bool) -> Self {
        self.config.enable_limited_punctuation = v;
        self
    }

    /// 혼동모델 비활성화 (true로 설정시 혼동모델 호출 안함)
    pub fn disable_confusion(mut self, v: bool) -> Self {
        self.config.disable_confusion = v;
        self
    }

    pub fn enable_cleanup_whitespace(mut self, v: bool) -> Self {
        self.config.enable_cleanup_whitespace = v;
        self
    }

    /// 오타 교정 비활성화 (true로 설정시 오탈자 교정 안함)
    pub fn disable_typo_correction(mut self, v: bool) -> Self {
        self.config.disable_typo_correction = v;
        self
    }

    pub fn enable_sentence_check(mut self, v: bool) -> Self {
        self.config.enable_sentence_check = v;
        self
    }

    pub fn build(self) -> RevisionConfig {
        self.config
    }
}

/// 스트림 응답을 더 다루기 쉽게 래핑한 enum
pub enum StreamRevisionEvent {
    First(StreamFirstCorrectError),
    Cancelled(CancelledRevision),
    Post(PostRevision),
    Progress(ProgressRevision),
}

impl StreamRevisionEvent {
    /// gRPC 메시지(oneof res)에서 이벤트로 변환. 비어 있으면 None
    pub fn from_message(msg: StreamCorrectErrorResponse) -> Option<Self> {
        msg.res.map(|r| match r {
            StreamRes::First(v) => StreamRevisionEvent::First(v),
            StreamRes::Cancelled(v) => StreamRevisionEvent::Cancelled(v),
            StreamRes::Post(v) => StreamRevisionEvent::Post(v),
            StreamRes::Progress(v) => StreamRevisionEvent::Progress(v),
        })
    }
}

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

    /// `RevisionConfigBuilder`로 바로 교정을 요청하는 편의 메서드
    pub async fn correct_error_with(
        &mut self,
        content: &str,
        custom_dicts: &[String],
        builder: RevisionConfigBuilder,
    ) -> Result<CorrectErrorResponse> {
        self.correct_error(content, custom_dicts, Some(builder.build()))
            .await
    }

    /// `RevisionConfigBuilder`로 바로 스트리밍 교정을 요청하는 편의 메서드
    pub async fn stream_correct_error_builder(
        &mut self,
        content: &str,
        custom_dicts: &[String],
        builder: RevisionConfigBuilder,
        req_id: i64,
    ) -> Result<Streaming<StreamCorrectErrorResponse>> {
        self.stream_correct_error(content, custom_dicts, Some(builder.build()), req_id)
            .await
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
        #[allow(deprecated)]
        let request = CorrectErrorRequest {
            document: Some(Document {
                content: content.to_string(),
                language: "ko_KR".to_string(),
            }),
            encoding_type: EncodingType::Utf32.into(),
            custom_domain: String::new(), // deprecated field
            custom_dict_names: custom_dicts.to_vec(),
            config,
        };

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
            if let Some(tc) = block.thinking_count {
                println!(" 생각 중인 교정 수: {}", tc);
            }
            for rev in &block.revisions {
                let help_text = res
                    .helps
                    .get(&rev.help_id)
                    .map(|h| h.comment.as_str())
                    .unwrap_or("");
                let thinking = rev
                    .thinking_id
                    .map(|id| format!(", thinking_id:{}", id))
                    .unwrap_or_default();
                println!(
                    " 교정: {}, 카테고리:{}{}, 도움말 {}",
                    rev.revised, rev.category, thinking, help_text
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

    /// 스트리밍 맞춤법 교정 요청
    ///
    /// 첫 번째 응답(StreamFirstCorrectError)에 이어 서버가 후처리한
    /// thinking revision(Cancelled/Post/Progress)이 차례로 전달된다.
    ///
    /// Args:
    ///     content: 교정을 요청할 문장
    ///     custom_dicts: 커스텀 사전 이름들
    ///     config: 요청 설정
    ///     req_id: 요청 ID (0이면 서버가 생성)
    pub async fn stream_correct_error(
        &mut self,
        content: &str,
        custom_dicts: &[String],
        config: Option<RevisionConfig>,
        req_id: i64,
    ) -> Result<Streaming<StreamCorrectErrorResponse>> {
        #[allow(deprecated)]
        let request = StreamCorrectErrorRequest {
            document: Some(Document {
                content: content.to_string(),
                language: "ko_KR".to_string(),
            }),
            encoding_type: EncodingType::Utf32.into(),
            custom_domain: String::new(),
            custom_dict_names: custom_dicts.to_vec(),
            config,
            req_id,
        };

        self.client.stream_correct_error(request).await
    }

    /// 스트리밍 응답을 순차적으로 수신하며 `StreamRevisionEvent`로 변환해 처리한다.
    ///
    /// 호출자는 각 이벤트(First/Cancelled/Post/Progress)마다 동작을 정의할 수 있다.
    ///
    /// Args:
    ///     content: 교정을 요청할 문장
    ///     custom_dicts: 커스텀 사전 이름들
    ///     config: 요청 설정
    ///     req_id: 요청 ID (0이면 서버가 생성)
    ///     on_event: 이벤트 핸들러. `false`를 반환하면 루프를 조기 종료한다.
    pub async fn stream_correct_error_with<F>(
        &mut self,
        content: &str,
        custom_dicts: &[String],
        config: Option<RevisionConfig>,
        req_id: i64,
        mut on_event: F,
    ) -> Result<()>
    where
        F: FnMut(StreamRevisionEvent) -> bool,
    {
        let mut stream = self
            .stream_correct_error(content, custom_dicts, config, req_id)
            .await?;

        while let Some(msg) = stream.message().await? {
            if let Some(event) = StreamRevisionEvent::from_message(msg) {
                if !on_event(event) {
                    break;
                }
            }
        }

        Ok(())
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
