use std::collections::{HashMap, HashSet};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint};
use tonic::{Request, Status};

use crate::bareun::{
    CheckConflictRequest, CheckConflictResponse, CustomDictionary, CustomDictionaryMeta, DictSet,
    DictType, GetCustomDictionaryRequest, RemoveCustomDictionariesRequest,
    UpdateCustomDictionaryRequest,
    custom_dictionary_service_client::CustomDictionaryServiceClient as TonicClient,
};
use crate::constants::{CA_BUNDLE, MAX_MESSAGE_LENGTH};
use crate::error::{BareunError, Result};

/// 주어진 파라미터를 사용하여 사용자 사전의 한 표현 형태인 DictSet protobuf 메시지를 만듭니다.
///
/// Args:
///     domain (str): 사용자 사전의 이름
///     name (str): 사용자 사전에 대한 설명
///     dict_set (set): 사용자 사전에 들어가야 할 단어들의 집합
pub fn build_dict_set(domain: &str, name: &str, dict_set: &HashSet<String>) -> DictSet {
    let mut ret = DictSet::default();
    ret.name = format!("{}-{}", domain, name);
    ret.r#type = DictType::WordList as i32;
    let mut items = HashMap::default();
    for v in dict_set {
        items.insert(v.clone(), 1);
    }
    ret.items = items;
    ret
}

fn is_tls_host(host: &str) -> bool {
    host.to_lowercase().starts_with("api.bareun.ai")
}

fn resolve_port(host: &str, port: i32) -> u16 {
    if port > 0 {
        port as u16
    } else if is_tls_host(host) {
        443
    } else {
        5656
    }
}

/// 커스텀 사전을 생성, 조회, 업데이트, 삭제하는 클라이언트
pub struct CustomDictionaryServiceClient {
    pub channel: Channel,
    pub apikey: String,
    pub host: String,
    pub port: u16,
}

impl CustomDictionaryServiceClient {
    /// 사용자 사전을 관리하는 클라이언트 객체 생성자
    ///
    /// Args:
    ///     apikey: Bareun API 키
    ///     host: Bareun 서버 호스트 주소
    ///     port: Bareun 서버 포트 번호. 0 이하면 호스트에 맞춰 자동 설정
    pub async fn new(apikey: &str, host: &str, port: i32) -> Result<Self> {
        let host_trim = host.trim();
        let host = if host_trim.is_empty() {
            "api.bareun.ai"
        } else {
            host_trim
        };
        let port = resolve_port(host, port);
        let channel = Self::create_channel(host, port).await?;

        Ok(Self {
            channel,
            apikey: apikey.to_string(),
            host: host.to_string(),
            port,
        })
    }

    async fn create_channel(host: &str, port: u16) -> Result<Channel> {
        let uri = if is_tls_host(host) {
            format!("https://{}:{}", host, port)
        } else {
            format!("http://{}:{}", host, port)
        };

        let endpoint = Endpoint::from_shared(uri)?;

        let channel = if is_tls_host(host) {
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

    fn tonic_client(&self) -> TonicClient<Channel> {
        TonicClient::new(self.channel.clone())
            .max_decoding_message_size(MAX_MESSAGE_LENGTH)
            .max_encoding_message_size(MAX_MESSAGE_LENGTH)
    }

    fn build_request<T>(&self, msg: T) -> Result<Request<T>> {
        let mut req = Request::new(msg);
        let api_key = self
            .apikey
            .parse()
            .map_err(BareunError::InvalidMetadataValue)?;
        req.metadata_mut().insert("api-key", api_key);
        Ok(req)
    }

    /// 사전 목록을 가져옵니다.
    pub async fn get_list(&mut self) -> Result<Vec<CustomDictionaryMeta>> {
        let mut client = self.tonic_client();
        let req = self.build_request(())?;

        match client.get_custom_dictionary_list(req).await {
            Ok(res) => Ok(res.into_inner().domain_dicts),
            Err(e) => Err(self.handle_grpc_error(e)),
        }
    }

    /// 정의된 사용자 사전의 내용 전체를 가져온다.
    pub async fn get(&mut self, domain: &str) -> Result<CustomDictionary> {
        let mut client = self.tonic_client();
        let mut req_msg = GetCustomDictionaryRequest::default();
        req_msg.domain_name = domain.to_string();
        let req = self.build_request(req_msg)?;

        match client.get_custom_dictionary(req).await {
            Ok(res) => res.into_inner().dict.ok_or_else(|| BareunError::InvalidArgument {
                message: format!("server returned empty dict for domain '{}'", domain),
            }),
            Err(e) => Err(self.handle_grpc_error(e)),
        }
    }

    /// 사용자 사전을 갱신합니다.
    ///
    /// Args:
    ///     domain: 사용자 사전의 이름
    ///     np: 고유명사 단어 집합
    ///     cp: 복합명사 단어 집합
    ///     cp_caret: 복합명사 분리 단어 집합
    ///     vv: 동사 단어 집합
    ///     va: 형용사 단어 집합
    ///     mm: 관형사 단어 집합
    ///     mag: 부사 단어 집합
    ///     ic: 감탄사 단어 집합
    pub async fn update(
        &mut self,
        domain: &str,
        np: &HashSet<String>,
        cp: &HashSet<String>,
        cp_caret: &HashSet<String>,
        vv: &HashSet<String>,
        va: &HashSet<String>,
        mm: &HashSet<String>,
        mag: &HashSet<String>,
        ic: &HashSet<String>,
    ) -> Result<bool> {
        let mut client = self.tonic_client();
        let mut req_msg = UpdateCustomDictionaryRequest::default();
        req_msg.domain_name = domain.to_string();

        let mut dict = CustomDictionary::default();
        dict.domain_name = domain.to_string();
        dict.np_set = Some(build_dict_set(domain, "np-set", np));
        dict.cp_set = Some(build_dict_set(domain, "cp-set", cp));
        dict.cp_caret_set = Some(build_dict_set(domain, "cp-caret-set", cp_caret));
        dict.vv_set = Some(build_dict_set(domain, "vv-set", vv));
        dict.va_set = Some(build_dict_set(domain, "va-set", va));
        dict.mm_set = Some(build_dict_set(domain, "mm-set", mm));
        dict.mag_set = Some(build_dict_set(domain, "mag-set", mag));
        dict.ic_set = Some(build_dict_set(domain, "ic-set", ic));

        req_msg.dict = Some(dict);
        let req = self.build_request(req_msg)?;

        match client.update_custom_dictionary(req).await {
            Ok(res) => Ok(res.into_inner().updated_domain_name == domain),
            Err(e) => Err(self.handle_grpc_error(e)),
        }
    }

    /// 모든 커스텀 사전을 삭제한 다음 삭제한 사전의 이름을 돌려줍니다.
    pub async fn remove_all(&mut self) -> Result<Vec<String>> {
        let mut client = self.tonic_client();
        let mut msg = RemoveCustomDictionariesRequest::default();
        msg.all = true;
        let req = self.build_request(msg)?;

        match client.remove_custom_dictionaries(req).await {
            Ok(res) => Ok(res
                .into_inner()
                .deleted_domain_names
                .keys()
                .cloned()
                .collect()),
            Err(e) => Err(self.handle_grpc_error(e)),
        }
    }

    /// 지정한 도메인의 사용자 사전을 삭제한 다음 삭제한 사전의 목록을 반환합니다.
    pub async fn remove(&mut self, domains: &[String]) -> Result<Vec<String>> {
        let mut client = self.tonic_client();
        let mut msg = RemoveCustomDictionariesRequest::default();
        msg.domain_names = domains.to_vec();
        msg.all = false;
        let req = self.build_request(msg)?;

        match client.remove_custom_dictionaries(req).await {
            Ok(res) => Ok(res
                .into_inner()
                .deleted_domain_names
                .keys()
                .cloned()
                .collect()),
            Err(e) => Err(self.handle_grpc_error(e)),
        }
    }

    /// 사용자 사전들 사이의 충돌을 점검합니다.
    pub async fn check_conflict(
        &mut self,
        domain_names: &[String],
    ) -> Result<CheckConflictResponse> {
        let mut client = self.tonic_client();
        let mut msg = CheckConflictRequest::default();
        msg.domain_names = domain_names.to_vec();
        let req = self.build_request(msg)?;

        match client.check_conflict(req).await {
            Ok(res) => Ok(res.into_inner()),
            Err(e) => Err(self.handle_grpc_error(e)),
        }
    }
}
