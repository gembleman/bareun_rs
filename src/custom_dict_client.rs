use std::collections::{HashMap, HashSet};
use tonic::transport::Channel;

use crate::bareun::{
    CheckConflictRequest, CheckConflictResponse, CustomDictionary, CustomDictionaryMeta, DictSet,
    DictType, GetCustomDictionaryRequest, RemoveCustomDictionariesRequest,
    UpdateCustomDictionaryRequest,
    custom_dictionary_service_client::CustomDictionaryServiceClient as TonicClient,
};
use crate::constants::MAX_MESSAGE_LENGTH;
use crate::error::Result;

/**
주어진 파라미터를 사용하여 사용자 사전의 한 표현 형태인 DictSet protobuf 메시지를 만듭니다.
Args:
    domain (str): 사용자 사전의 이름
    name (str): 사용자 사전에 대한 설명
    dict_set (set): 사용자 사전에 들어가야 할 단어들의 잡합

Returns:
    DictSet: protobuf DictSet 메시지
*/
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

/**
커스텀 사전을 생성, 조회, 업데이트, 삭제하는 클라이언트
The custom dictionary client which can create, update, list, delete your own one.
*/
pub struct CustomDictionaryServiceClient {
    pub channel: Channel,
    pub apikey: String,
    pub metadata: (String, String),
    // stub: CustomDictionaryServiceStub,
}

impl CustomDictionaryServiceClient {
    /**사용자 사전을 관리하는 클라이언트 객체 생성자
    Args:
        remote (grpc.Channel): 미리 만들어 놓은 channel 객체
    */
    pub async fn new(apikey: &str, host: &str, port: i32) -> Result<Self> {
        let channel = Channel::from_shared(format!("http://{}:{}", host, port))
            .unwrap()
            .connect()
            .await?;
        Ok(Self {
            channel,
            apikey: apikey.to_string(),
            metadata: ("api_key".to_string(), apikey.to_string()),
        })
    }

    /**사전 목록을 가져옵니다.

    Raises:
        e: grpc::Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        Vec<CustomDictionaryMeta>: 사전에 대한 정보들을 목록을 배열합니다.
    */
    pub async fn get_list(&mut self) -> Result<Vec<CustomDictionaryMeta>> {
        let mut client = TonicClient::new(self.channel.clone())
            .max_decoding_message_size(MAX_MESSAGE_LENGTH)
            .max_encoding_message_size(MAX_MESSAGE_LENGTH);

        let req = tonic::Request::new(());

        let res = client.get_custom_dictionary_list(req).await?;
        Ok(res.into_inner().domain_dicts)
    }

    /**
    정의된 사용사 사전의 내용 전체를 가져온다.

    Args:
        domain (str): 사용자 사전이 이름

    Raises:
        e: grpc::Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        CustomDictionary: 사용자 사전 데이터 전체를 담고 있는 protobuf 메시지
    */
    pub async fn get(&mut self, domain: &str) -> Result<CustomDictionary> {
        let mut client = TonicClient::new(self.channel.clone())
            .max_decoding_message_size(MAX_MESSAGE_LENGTH)
            .max_encoding_message_size(MAX_MESSAGE_LENGTH);
        let mut req_msg = GetCustomDictionaryRequest::default();
        req_msg.domain_name = domain.to_string();

        let mut req = tonic::Request::new(req_msg);
        req.metadata_mut()
            .insert("api-key", self.apikey.parse().unwrap());

        let res = client.get_custom_dictionary(req).await?;
        Ok(res.into_inner().dict.unwrap())
    }

    /** 사용자 사전을 갱신합니다.

    Args:
        domain (str): 사용자 사전의 이름
        np (`HashSet<String>`): 고유명사 단어 집합
        cp (`HashSet<String>`): 복합명사 단어 집합
        cp_caret (`HashSet<String>`): 복합명사 분리 단어 집합
        vv (`HashSet<String>`): 동사 단어 집합
        va (`HashSet<String>`): 형용사 단어 집합
    Raises:
        e: grpc::Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        bool: 정상적으로 갱신되면 참을 돌려줍니다.
    */
    pub async fn update(
        &mut self,
        domain: &str,
        np: &HashSet<String>,
        cp: &HashSet<String>,
        cp_caret: &HashSet<String>,
        vv: &HashSet<String>,
        va: &HashSet<String>,
    ) -> Result<bool> {
        let mut client = TonicClient::new(self.channel.clone())
            .max_decoding_message_size(MAX_MESSAGE_LENGTH)
            .max_encoding_message_size(MAX_MESSAGE_LENGTH);
        let mut req_msg = UpdateCustomDictionaryRequest::default();
        req_msg.domain_name = domain.to_string();

        let mut dict = CustomDictionary::default();
        dict.domain_name = domain.to_string();
        dict.np_set = Some(build_dict_set(domain, "np-set", np));
        dict.cp_set = Some(build_dict_set(domain, "cp-set", cp));
        dict.vv_set = Some(build_dict_set(domain, "vv-set", vv));
        dict.va_set = Some(build_dict_set(domain, "va-set", va));
        dict.cp_caret_set = Some(build_dict_set(domain, "cp-caret-set", cp_caret));

        req_msg.dict = Some(dict);

        let mut req = tonic::Request::new(req_msg);
        req.metadata_mut()
            .insert("api-key", self.apikey.parse().unwrap());

        let res = client.update_custom_dictionary(req).await?;
        Ok(res.into_inner().updated_domain_name == domain)
    }

    /**
    모든 커스텀 사전을 삭제한 다음 삭제한 사전의 이름을 돌려줍니다.

    Raises:
        e: grpc::Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        `Vec<String>`: 삭제한 사전의 이름
    */
    pub async fn remove_all(&mut self) -> Result<Vec<String>> {
        let mut client = TonicClient::new(self.channel.clone())
            .max_decoding_message_size(MAX_MESSAGE_LENGTH)
            .max_encoding_message_size(MAX_MESSAGE_LENGTH);
        let mut req = RemoveCustomDictionariesRequest::default();
        req.all = true;

        let res = client.remove_custom_dictionaries(req).await?;
        Ok(res
            .into_inner()
            .deleted_domain_names
            .keys()
            .cloned()
            .collect())
    }

    /** 지정한 도메인의 사용지 사전을 삭제한 다음 삭제한 사전의 목록을 반환합니다.

    Args:
        domains (`Vec<String>`): 삭제할 커스텀 사전의 이름들

    Raises:
        e: grpc::Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        `Vec<String>`: 정상 삭제된 도메인의 이름 목록을 돌려줍니다.
    */
    pub async fn remove(&mut self, domains: &[String]) -> Result<Vec<String>> {
        let mut client = TonicClient::new(self.channel.clone())
            .max_decoding_message_size(MAX_MESSAGE_LENGTH)
            .max_encoding_message_size(MAX_MESSAGE_LENGTH);
        let mut req = RemoveCustomDictionariesRequest::default();
        req.domain_names = domains.to_vec();
        req.all = false;

        let res = client.remove_custom_dictionaries(req).await?;
        Ok(res
            .into_inner()
            .deleted_domain_names
            .keys()
            .cloned()
            .collect())
    }

    /** 사용자 사전들 사이의 충돌을 점검합니다.

    Args:
        domain_names (`Vec<String>`): 점검할 사용자 사전들의 이름

    Raises:
        e: grpc::Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        CheckConflictResponse: 충돌 점검 결과
    */
    pub async fn check_conflict(
        &mut self,
        domain_names: &[String],
    ) -> Result<CheckConflictResponse> {
        let mut client = TonicClient::new(self.channel.clone())
            .max_decoding_message_size(MAX_MESSAGE_LENGTH)
            .max_encoding_message_size(MAX_MESSAGE_LENGTH);
        let mut req = CheckConflictRequest::default();
        req.domain_names = domain_names.to_vec();

        let res = client.check_conflict(req).await?;
        Ok(res.into_inner())
    }
}
