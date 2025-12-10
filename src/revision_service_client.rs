use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint};
use tonic::{Request, Status};

use crate::bareun::revision_service_client::RevisionServiceClient;
use crate::bareun::{CorrectErrorRequest, CorrectErrorResponse};
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

pub struct BareunRevisionServiceClient {
    pub client: RevisionServiceClient<Channel>,
    pub apikey: String,
    pub host: String,
    pub port: u16,
}

impl BareunRevisionServiceClient {
    /// RevisionServiceClient 초기화
    ///
    /// Args:
    ///     apikey: API 키
    ///     host: gRPC 서버 주소
    ///     port: gRPC 서버 포트
    pub async fn new(apikey: &str, host: &str, port: Option<u16>) -> Result<Self> {
        let host = host.trim();
        let host = if host.is_empty() {
            "api.bareun.ai"
        } else {
            host
        };

        let port = resolve_port(host, port);
        let channel = Self::create_channel(host, port).await?;
        let client = RevisionServiceClient::new(channel)
            .max_decoding_message_size(MAX_MESSAGE_LENGTH)
            .max_encoding_message_size(MAX_MESSAGE_LENGTH);

        Ok(BareunRevisionServiceClient {
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

    /// 맞춤법 교정을 위한 gRPC 호출
    pub async fn correct_error(
        &mut self,
        request: CorrectErrorRequest,
    ) -> Result<CorrectErrorResponse> {
        let mut req = Request::new(request);
        req.metadata_mut()
            .insert("api-key", self.apikey.parse().unwrap());

        match self.client.correct_error(req).await {
            Ok(response) => Ok(response.into_inner()),
            Err(e) => Err(self.handle_grpc_error(e)),
        }
    }
}
