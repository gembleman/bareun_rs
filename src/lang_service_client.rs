use std::collections::HashMap;

use tonic::transport::{channel, Channel, Error};
use tonic::Status;

use crate::bareun::language_service_client::LanguageServiceClient;
use crate::bareun::{
    AnalyzeSyntaxListRequest, AnalyzeSyntaxListResponse, AnalyzeSyntaxRequest,
    AnalyzeSyntaxResponse, Document, EncodingType, TokenizeRequest, TokenizeResponse,
};
// use grpcio::{ChannelBuilder, Environment};
pub const MAX_MESSAGE_LENGTH: usize = 100 * 1024 * 1024;
pub struct BareunLanguageServiceClient {
    /**
    형태소 분석을 처리하는 클라이언트
    */
    client: LanguageServiceClient<Channel>,
    apikey: String,
    metadata: HashMap<String, String>,
}
impl BareunLanguageServiceClient {
    /**
    클라이언트 생성자
    Args:
        channel (grpcio::Channel): 원격 채널 정보
    */
    pub async fn new(apikey: &str, host: &str, port: i32) -> Self {
        // let env = Environment::new(1);
        // let channel = ChannelBuilder::new(env).connect(&format!("{}:{}", host, port));
        let endpoint = channel::Endpoint::new(format!("grpc://{}:{}", host, port)).unwrap();
        let channel = endpoint.connect().await.unwrap();
        let client = LanguageServiceClient::new(channel);
        println!(
            "Connected to server at {}:{}\napikey:{}",
            host, port, apikey
        );

        BareunLanguageServiceClient {
            client,
            apikey: apikey.to_string(),
            metadata: HashMap::new(),
        }
    }
    /**
    형태소 분석을 수행합니다.

    Args:
        content (str): 형태소 분석할 원문, 여러 문장일 경우에 개행문자로 줄바꿈을 하면 됩니다.
        domain (str, optional): 사용사 사전의 이름. 기본값은 "".
        auto_split (bool, optional): 문장 자동 분리 여부, 기본값은 사용하지 않음.
        auto_spacing (bool, optional): 띄어쓰기 보정 기능, 기본값은 사용하도록 함.
        auto_jointing (bool, optional): 붙여쓰기 보정 기능, 기본값은 사용하지 않음.

    Raises:
        e: Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        AnalyzeSyntaxResponse: 형태소 분석 결과
    */
    pub async fn analyze_syntax(
        &mut self,
        content: &str,
        domain: &str,
        auto_split: bool,
        auto_spacing: bool,
        auto_jointing: bool,
    ) -> Result<AnalyzeSyntaxResponse, Status> {
        let mut req = AnalyzeSyntaxRequest::default();
        let mut document = Document::default();
        document.content = content.to_string();
        document.language = "ko_KR".to_string();
        req.document = Some(document);
        req.encoding_type = EncodingType::Utf32.into();
        req.auto_split_sentence = auto_split;
        req.auto_spacing = auto_spacing;
        req.auto_jointing = auto_jointing;
        if !domain.is_empty() {
            req.custom_domain = domain.to_string();
        }

        let mut req = tonic::Request::new(req);
        req.metadata_mut()
            .insert("api-key", self.apikey.parse().unwrap());

        match self.client.analyze_syntax(req).await {
            Ok(response) => Ok(response.into_inner()),
            Err(error) => Err(error),
        }
    }
    /**
    형태소 분석을 수행하되, 입력된 문장 단위가 일치하도록 반환됩니다.
    문장 분할 기능을 사용하지 않습니다.

    Args:
        content (Vec<String>): 형태소 분석할 원문의 리스트
        domain (str, optional): 사용사 사전의 이름. 기본값은 "".
        auto_spacing (bool, optional): 띄어쓰기 보정 기능, 기본값은 사용하도록 함.
        auto_jointing (bool, optional): 붙여쓰기 보정 기능, 기본값은 사용하지 않음.

    Raises:
        e: Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        AnalyzeSyntaxListResponse: 형태소 분석 결과
    */
    pub async fn analyze_syntax_list(
        &mut self,
        content: &[String],
        domain: &str,
        auto_spacing: bool,
        auto_jointing: bool,
    ) -> Result<AnalyzeSyntaxListResponse, Status> {
        let mut req = AnalyzeSyntaxListRequest::default();
        req.sentences = content.to_vec().into();
        req.language = "ko_KR".to_string();
        req.encoding_type = EncodingType::Utf32.into();
        req.auto_spacing = auto_spacing;
        req.auto_jointing = auto_jointing;
        if !domain.is_empty() {
            req.custom_domain = domain.to_string();
        }
        match self
            .client
            .analyze_syntax_list(tonic::Request::new(req))
            .await
        {
            Ok(response) => Ok(response.into_inner()),
            Err(error) => Err(error),
        }
    }
    /**
    형태소 분석을 수행합니다.

    Args:
        content (str): 형태소 분석할 원문, 여러 문장일 경우에 개행문자로 줄바꿈을 하면 됩니다.
        auto_split (bool, optional): 문장 자동 분리 여부, 기본값은 사용하지 않음.

    Raises:
        e: Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        TokenizeResponse: 형태소 분석 결과
    */
    pub async fn tokenize(
        &mut self,
        content: &str,
        auto_split: bool,
    ) -> Result<TokenizeResponse, Status> {
        let mut req = TokenizeRequest::default();
        let mut document = Document::default();
        document.content = content.to_string();
        document.language = "ko_KR".to_string();
        req.document = Some(document);

        req.encoding_type = EncodingType::Utf32.into();
        req.auto_split_sentence = auto_split;
        match self.client.tokenize(tonic::Request::new(req)).await {
            Ok(response) => Ok(response.into_inner()),
            Err(error) => Err(error),
        }
    }
}
