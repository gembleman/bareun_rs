use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint};
use tonic::{Request, Status};

use crate::bareun::language_service_client::LanguageServiceClient;
use crate::bareun::{
    AnalyzeSyntaxListRequest, AnalyzeSyntaxListResponse, AnalyzeSyntaxRequest,
    AnalyzeSyntaxResponse, Document, EncodingType, TokenizeRequest, TokenizeResponse,
};
use crate::constants::{CA_BUNDLE, MAX_MESSAGE_LENGTH};
use crate::error::{BareunError, Result};

fn resolve_port(host: &str, port: Option<u16>) -> u16 {
    if let Some(p) = port {
        p
    } else if host.to_lowercase().starts_with("api.bareun.ai") {
        443
    } else {
        5656
    }
}

pub struct BareunLanguageServiceClient {
    pub client: LanguageServiceClient<Channel>,
    pub apikey: String,
    pub host: String,
    pub port: u16,
}

impl BareunLanguageServiceClient {
    /// 클라이언트 생성자
    ///
    /// Args:
    ///     apikey: Bareun API 키
    ///     host: Bareun 서버 호스트 주소
    ///     port: Bareun 서버 포트 번호
    pub async fn new(apikey: &str, host: &str, port: Option<u16>) -> Result<Self> {
        let host = host.trim();
        let host = if host.is_empty() {
            "api.bareun.ai"
        } else {
            host
        };

        let port = resolve_port(host, port);
        let channel = Self::create_channel(host, port).await?;
        let client = LanguageServiceClient::new(channel)
            .max_decoding_message_size(MAX_MESSAGE_LENGTH)
            .max_encoding_message_size(MAX_MESSAGE_LENGTH);

        Ok(BareunLanguageServiceClient {
            client,
            apikey: apikey.to_string(),
            host: host.to_string(),
            port,
        })
    }

    async fn create_channel(host: &str, port: u16) -> Result<Channel> {
        let uri = if host.to_lowercase().starts_with("api.bareun.ai") {
            format!("https://{}:{}", host, port)
        } else {
            format!("http://{}:{}", host, port)
        };

        let endpoint = Endpoint::from_shared(uri)?;

        let channel = if host.to_lowercase().starts_with("api.bareun.ai") {
            let cert = Certificate::from_pem(CA_BUNDLE);
            let tls = ClientTlsConfig::new().ca_certificate(cert);
            endpoint.tls_config(tls)?.connect().await?
        } else {
            endpoint.connect().await?
        };

        Ok(channel)
    }

    fn handle_grpc_error(&self, e: Status) -> BareunError {
        let code = e.code();
        let details = e.message();
        let server_message = if details.is_empty() {
            "서버에서 추가 메시지를 제공하지 않았습니다.".to_string()
        } else {
            details.to_string()
        };

        match code {
            tonic::Code::PermissionDenied => BareunError::PermissionDenied {
                apikey: self.apikey.clone(),
                message: server_message,
            },
            tonic::Code::Unavailable => BareunError::ServerUnavailable {
                host: self.host.clone(),
                port: self.port,
                message: server_message,
            },
            tonic::Code::InvalidArgument => BareunError::InvalidArgument {
                message: server_message,
            },
            _ => BareunError::GrpcError(server_message),
        }
    }

    /// 형태소 분석을 수행합니다.
    ///
    /// Args:
    ///     content: 형태소 분석할 원문
    ///     custom_dicts: 사용할 커스텀 사전 이름 목록
    ///     auto_split: 문장 자동 분리 여부
    ///     auto_spacing: 띄어쓰기 보정 기능
    ///     auto_jointing: 붙여쓰기 보정 기능
    pub async fn analyze_syntax(
        &mut self,
        content: &str,
        custom_dicts: &[String],
        auto_split: bool,
        auto_spacing: bool,
        auto_jointing: bool,
    ) -> Result<AnalyzeSyntaxResponse> {
        let req = AnalyzeSyntaxRequest {
            document: Some(Document {
                content: content.to_string(),
                language: "ko_KR".to_string(),
            }),
            encoding_type: EncodingType::Utf32.into(),
            auto_split_sentence: auto_split,
            auto_spacing,
            auto_jointing,
            #[allow(deprecated)]
            custom_domain: String::new(), // deprecated field
            custom_dict_names: custom_dicts.to_vec(),
        };

        let mut request = Request::new(req);
        request
            .metadata_mut()
            .insert("api-key", self.apikey.parse().unwrap());

        match self.client.analyze_syntax(request).await {
            Ok(response) => Ok(response.into_inner()),
            Err(e) => Err(self.handle_grpc_error(e)),
        }
    }

    /// 형태소 분석을 수행하되, 입력된 문장 단위가 일치하도록 반환됩니다.
    pub async fn analyze_syntax_list(
        &mut self,
        content: &[String],
        custom_dicts: &[String],
        auto_spacing: bool,
        auto_jointing: bool,
    ) -> Result<AnalyzeSyntaxListResponse> {
        let req = AnalyzeSyntaxListRequest {
            sentences: content.to_vec(),
            language: "ko_KR".to_string(),
            encoding_type: EncodingType::Utf32.into(),
            auto_spacing,
            auto_jointing,
            #[allow(deprecated)]
            custom_domain: String::new(), // deprecated field
            custom_dict_names: custom_dicts.to_vec(),
        };

        let mut request = Request::new(req);
        request
            .metadata_mut()
            .insert("api-key", self.apikey.parse().unwrap());

        match self.client.analyze_syntax_list(request).await {
            Ok(response) => Ok(response.into_inner()),
            Err(e) => Err(self.handle_grpc_error(e)),
        }
    }

    /// 토크나이즈를 수행합니다.
    pub async fn tokenize(
        &mut self,
        content: &str,
        auto_split: bool,
    ) -> Result<TokenizeResponse> {
        let req = TokenizeRequest {
            document: Some(Document {
                content: content.to_string(),
                language: "ko_KR".to_string(),
            }),
            encoding_type: EncodingType::Utf32.into(),
            auto_split_sentence: auto_split,
            auto_spacing: false,
        };

        let mut request = Request::new(req);
        request
            .metadata_mut()
            .insert("api-key", self.apikey.parse().unwrap());

        match self.client.tokenize(request).await {
            Ok(response) => Ok(response.into_inner()),
            Err(e) => Err(self.handle_grpc_error(e)),
        }
    }
}
