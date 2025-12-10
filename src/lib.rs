// ! bareun_rs::bareun
//! =====  
//! Provides
//!   1. a Korean Part-Of-Speech Tagger as bareun client
//!   2. Multiple custom dictionaries which is kept in the your bareun server.
//!   3. Korean spelling corrector
//!
//!
//! How to use the documentation
//! ----------------------------
//! Full documentation for bareun is available in
//! installable tarball or docker images.
//! - see `docs/intro.html` at installable tarball.
//! - or `http://localhost:5757/intro.html` after running docker.
//!
//! The docstring examples assume that `bareun_rs::bareun` has been imported as `brn`.
//!   >>> use bareun_rs::bareun as brn;
//!
//! Classes
//! -------
//! Tagger  
//!     the bareun POS tagger for Korean  
//!     `use bareun_rs::bareun::Tagger;`  
//! Tagged  
//!     Wrapper for tagged output  
//!     `use bareun_rs::bareun::Tagged;`  
//! Tokenizer  
//!     the bareun tokenizer for Korean  
//!     `use bareun_rs::bareun::Tokenizer;`  
//! Tokenized  
//!     Wrapper for tokenized output  
//!     `use bareun_rs::bareun::Tokenized;`  
//! Corrector  
//!     Korean spelling corrector  
//!     `use bareun_rs::bareun::Corrector;`  
//! CustomDict  
//!     Custom dictionary for Korean.  
//!     `use bareun_rs::bareun::CustomDict;`  
//!
//!
//! Get bareun
//! ----------------------------
//! - Use docker, <https://hub.docker.com/r/bareunai/bareun>
//! - Or visit <https://bareun.ai/>

mod constants;
mod corrector;
mod custom_dict;
mod custom_dict_client;
mod error;
mod lang_service_client;
mod revision_service_client;
mod tagger;
mod tokenizer;

pub use crate::corrector::*;
pub use crate::custom_dict::*;
pub use crate::custom_dict_client::*;
pub use crate::error::*;
pub use crate::lang_service_client::*;
pub use crate::revision_service_client::*;
pub use crate::tagger::*;
pub use crate::tokenizer::*;

pub mod bareun {
    tonic::include_proto!("bareun");
}
