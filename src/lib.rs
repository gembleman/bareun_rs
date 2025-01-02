//! bareun_rs::bareun
//! =====
//! Provides
//!   1. a Korean Part-Of-Speech Tagger as bareun client
//!   2. Multiple custom dictionaries which is kept in the your bareun server.
//!
//!
//! How to use the documentation
//! ----------------------------
//! Full documentation for bareun is available in
//! installable tarball or docker images.
//! - see `docs/intro.html` at installable tarball.
//! - or `http://localhost:5757/intro.html` after running docker.
//!
//! The docstring examples assume that `bareun_rs::bareun` has been imported as `brn`::
//!   >>> use bareun_rs::bareun as brn;
//!
//! Use the built-in ``help`` function to view a class's docstring::
//!   >>> help(brn::Tagger)
//!   ...
//!
//! Classes
//! -------
//! Tagger
//!     the bareun POS tagger for Korean
//!     `use bareun_rs::bareun::Tagger;`
//! Tagged
//!     Wrapper for tagged output
//!     `use bareun_rs::bareun::Tagged;`
//! CustomDict
//!     Custom dictionary for Korean.
//!     `use bareun_rs::bareun::CustomDict;`
//!
//! Version
//! -------
//! ```
//! use bareun_rs as brn;
//! println!("{}", brn::VERSION);
//! println!("{}", brn::BAREUN_VERSION);
//! ```
//!
//! Get bareun
//! ----------------------------
//! - Use docker, https://hub.docker.com/r/bareunai/bareun
//! - Or visit https://bareun.ai/

pub use crate::custom_dict::CustomDict;
// pub use crate::custom_dict_client::CustomDictionaryServiceClient; // not yet
pub use crate::lang_service_client::BareunLanguageServiceClient;
pub use crate::tagger::{Tagged, Tagger};
pub use crate::tokenizer::{Tokenized, Tokenizer};

mod custom_dict;
// mod custom_dict_client; // not yet
mod lang_service_client;
mod tagger;
mod tokenizer;

pub const VERSION: &str = "1.6.3";
pub const BAREUN_VERSION: &str = "1.8.0";

pub mod bareun {
    tonic::include_proto!("bareun");
}
