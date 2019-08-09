#![crate_name = "voca_rs"]
#![deny(
    warnings,
    unused_variables,
    missing_docs,
    unsafe_code,
    unused_extern_crates
)]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]

//! Voca_rs is the ultimate Rust string library inspired by Voca.js and string.py
//! ```rust
//! use voca_rs::*;
//! let input_string = "LazyLoad with XMLHttpRequest and snake_case";
//! let string_in_words = split::words(&input_string);
//! // => ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
//! let words_in_string = &string_in_words.join(" ");
//! // => "Lazy Load with XML Http Request and snake case"
//! let snake_string = case::snake_case(&chop::slice(&words_in_string, 13, 28));
//! // => "xml_http_request"
//! let truncated_string = chop::prune(&words_in_string, 15, "");
//! // => "Lazy Load..."
//! ```

extern crate dissolve;
extern crate regex;
extern crate stfu8;
extern crate unicode_segmentation;
extern crate unidecode;

// #[macro_use]
// extern crate lazy_static;

pub mod case;
pub mod chop;
pub mod count;
pub mod escape;
pub mod index;
pub mod manipulate;
pub mod query;
pub mod split;
pub mod strip;
pub mod utils;
