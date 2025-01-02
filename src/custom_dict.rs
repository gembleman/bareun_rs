use crate::bareun::DictSet;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/**
사용자 사전의 파일을 읽어들입니다.
Args:
    user_dict_path (str): 사용자 사전 파일 이름

Returns:
    HashSet<String>: 사용자 사전을 HashSet 형식으로 만들어서 돌려줍니다.
*/
pub fn read_dic_file(user_dict_path: &str) -> HashSet<String> {
    let mut dict_set = HashSet::new();
    let file = File::open(user_dict_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(w) = line {
            if !w.starts_with("#") {
                let w2 = w.trim();
                if !w2.is_empty() {
                    dict_set.insert(w2.to_string());
                }
            }
        }
    }
    dict_set
}
/**
DictSet을 사전으로 변환합니다.
Args:
    ds (&DictSet): DictSet 객체

Returns:
    HashSet<String>: 중복이 없는 사전 객체
*/
pub fn pb_map_to_set(ds: &DictSet) -> HashSet<String> {
    let mut ret = HashSet::new();
    for k in ds.items.keys() {
        ret.insert(k.clone());
    }
    ret
}
/**
    사용자 사전을 쉽게 사용하도록 해주는 래퍼(wrapper).
    'CustomDict' .
:ref:`optional-installations`.
 .. code-block:: rust
     :emphasize-lines: 1
     >>> use bareunpy::{Tagger, CustomDict};
     >>> use std::collections::HashSet;
     >>> let mut tagger = Tagger::new("YOUR_API_KEY", "HOST", 5656, "");
     >>> let mut cd = tagger.custom_dict("law");
     >>> // or
     >>> let mut cd = CustomDict::new("law");
     >>> cd.read_cp_set_from_file("my_np_set.txt");
     >>> let cp_set = vec!["새단어".to_string(), "코로나19".to_string(), "K방역".to_string()];
     >>> cd.copy_cp_set(cp_set.into_iter().collect());
     >>> cd.read_cp_caret_set_from_file("my_cp_caret.txt");
     >>> let vv_set = vec!["카톡하".to_string(), "신박하다".to_string()];
     >>> cd.copy_vv_set(vv_set.into_iter().collect());
     >>> let va_set = vec!["드라마틱하".to_string(), "판타스틱하".to_string()];
     >>> cd.copy_va_set(va_set.into_iter().collect());
     >>> cd.update();
     >>> // copy data from server
     >>> let mut cd2 = tagger.custom_dict("law");
     >>> let custom_dict = cd2.get();
     >>> // cd2.save(dir="my_dir");

    사용자 사전 래퍼(wrapper)의 생성자

    Args:
        domain (str): 사용자 사전의 이름, 반드시 지정되어야 합니다.
    Raises:
        ValueError: 사용자 사전의 이름이 없으면 에러를 발생시킵니다.
*/
pub struct CustomDict {
    pub domain: String,
    pub cp_set: HashSet<String>,
    pub np_set: HashSet<String>,
    pub cp_caret_set: HashSet<String>,
    pub vv_set: HashSet<String>,
    pub va_set: HashSet<String>,
}
impl CustomDict {
    pub fn new(domain: &str) -> Self {
        if domain.is_empty() {
            panic!("domain name must be specified.");
        }

        CustomDict {
            domain: domain.to_string(),
            cp_set: HashSet::new(),
            np_set: HashSet::new(),
            cp_caret_set: HashSet::new(),
            vv_set: HashSet::new(),
            va_set: HashSet::new(),
        }
    }
    /**
    고유명사 사전을 파일에서 읽어들입니다.

    이 파일은 한줄에 하나의 사전입니다. '#'로 시작하는 줄은 무시합니다.

    Args:
        fn (str): 고유명사 파일 이름
    */
    pub fn read_np_set_from_file(&mut self, user_dict_path: &str) {
        self.np_set = read_dic_file(user_dict_path);
    }
    /**
    복합명사 사전을 파일에서 읽어들입니다.

    이 파일은 한줄에 하나의 사전입니다. '#'로 시작하는 줄은 무시합니다.

    Args:
        fn (str): 복합명사 파일 이름
    */
    pub fn read_cp_set_from_file(&mut self, user_dict_path: &str) {
        self.cp_set = read_dic_file(user_dict_path);
    }
    /**
    복합명사 분리 사전을 파일에서 읽어들입니다.

    이 파일은 한줄에 하나의 사전입니다. '#'로 시작하는 줄은 무시합니다.

    Args:
        fn (str): 복합명사 분리 사전 파일 이름
    */
    pub fn read_cp_caret_set_from_file(&mut self, user_dict_path: &str) {
        self.cp_caret_set = read_dic_file(user_dict_path);
    }
    /**
    동사 사전을 파일에서 읽어들입니다.

    이 파일은 한줄에 하나의 사전입니다. '#'로 시작하는 줄은 무시합니다.

    Args:
        fn (str): 동사 사전 파일 이름
    */
    pub fn read_vv_set_from_file(&mut self, user_dict_path: &str) {
        self.vv_set = read_dic_file(user_dict_path);
    }
    /**
    형용사 사전을 파일에서 읽어들입니다.

    이 파일은 한줄에 하나의 사전입니다. '#'로 시작하는 줄은 무시합니다.

    Args:
        fn (str): 형용사 사전 파일 이름
    */
    pub fn read_va_set_from_file(&mut self, user_dict_path: &str) {
        self.va_set = read_dic_file(user_dict_path);
    }
    /**
    집합을 고유명사 사전으로 지정합니다.

    Args:
        dict_set (HashSet<String>): 고유명사 사전
    */
    pub fn copy_np_set(&mut self, dict_set: HashSet<String>) {
        self.np_set = dict_set;
    }
    /**
    집합을 복합명사 사전으로 지정합니다.

    Args:
        dict_set (HashSet<String>): 복합명사 사전
    */
    pub fn copy_cp_set(&mut self, dict_set: HashSet<String>) {
        self.cp_set = dict_set;
    }
    /**
    집합을 복합명사 분리 사전으로 지정합니다.

    Args:
        dict_set (HashSet<String>): 복합명사 분리 사전
    */
    pub fn copy_cp_caret_set(&mut self, dict_set: HashSet<String>) {
        self.cp_caret_set = dict_set;
    }
    /**
    집합을 동사 사전으로 지정합니다.

    Args:
        dict_set (HashSet<String>): 동사 사전
    */
    pub fn copy_vv_set(&mut self, dict_set: HashSet<String>) {
        self.vv_set = dict_set;
    }
    /**
    집합을 형용사 사전으로 지정합니다.

    Args:
        dict_set (HashSet<String>): 형용사 사전
    */
    pub fn copy_va_set(&mut self, dict_set: HashSet<String>) {
        self.va_set = dict_set;
    }
    /**
    복합명사 사전을 바이칼 NLP 서버에 갱신합니다.

    Raises:
        e: grpc::Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        bool: 갱신이 성공하면 참을 돌려줍니다.
    */
    pub fn update(&self) -> bool {
        // TODO: Implement update method
        unimplemented!()
    }
    /**
    사용자 사전의 내용을 가져옵니다.
    가져온 결과는 현재 설정된 사전의 내용을 반영하지 않습니다.

    Raises:
        e: grpc::Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        CustomDictionary: 사용자 사전 데이터 전체를 담고 있는 protobuf 메시지
    */
    pub fn get(&self) {
        // TODO: Implement get method
        unimplemented!()
    }
    /**
    서버에 저정되어 있는 사용자 사전을 모두 가져옵니다.
    */
    pub fn load(&mut self) {
        // TODO: Implement load method
        unimplemented!()
    }
    /**
    사용자 사전의 내용을 삭제합니다.

    Raises:
        e: grpc::Error, 원격 호출시 예외가 발생할 수 있습니다.

    Returns:
        Vec<String>: 삭제한 사용자 사전의 이름
    */
    pub fn clear(&mut self) -> Vec<String> {
        self.np_set.clear();
        self.cp_set.clear();
        self.cp_caret_set.clear();
        // TODO: Implement clear method with gRPC
        unimplemented!()
    }
}
