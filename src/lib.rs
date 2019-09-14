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

#[allow(missing_docs)]
pub trait Voca {
    // case
    fn camel_case(&self) -> String;
    fn capitalize(&self, param: bool) -> String;
    fn decapitalize(&self, param: bool) -> String;
    fn kebab_case(&self) -> String;
    fn shouty_kebab_case(&self) -> String;
    fn lower_case(&self) -> String;
    fn pascal_case(&self) -> String;
    fn snake_case(&self) -> String;
    fn shouty_snake_case(&self) -> String;
    fn swap_case(&self) -> String;
    fn title_case(&self) -> String;
    fn train_case(&self) -> String;
    fn upper_case(&self) -> String;
    fn lower_first(&self) -> String;
    fn upper_first(&self) -> String;
    // chop

    // count

    // escape

    // index

    // manipulate

    // query
    //fn is_camel_case(&self) -> bool;

    // split

    // strip
}

macro_rules! implement_string_for {
    ( $trt:ident; $($typ:ident), *) => {
        $(
            impl $trt for $typ {
                fn camel_case(&self) -> String {
                    case::camel_case(&self)
                }
                fn capitalize(&self, param: bool) -> String {
                    case::capitalize(&self, param)
                }
                fn decapitalize(&self, param: bool) -> String {
                    case::decapitalize(&self, param)
                }
                fn kebab_case(&self) -> String {
                    case::kebab_case(&self)
                }
                fn shouty_kebab_case(&self) -> String {
                    case::shouty_kebab_case(&self)
                }
                fn lower_case(&self) -> String {
                    case::lower_case(&self)
                }
                fn pascal_case(&self) -> String {
                    case::pascal_case(&self)
                }
                fn snake_case(&self) -> String {
                    case::snake_case(&self)
                }
                fn shouty_snake_case(&self) -> String {
                    case::shouty_snake_case(&self)
                }
                fn swap_case(&self) -> String {
                    case::swap_case(&self)
                }
                fn title_case(&self) -> String {
                    case::title_case(&self)
                }
                fn train_case(&self) -> String {
                    case::train_case(&self)
                }
                fn upper_case(&self) -> String {
                    case::upper_case(&self)
                }
                fn lower_first(&self) -> String {
                    case::lower_first(&self)
                }
                fn upper_first(&self) -> String {
                    case::upper_first(&self)
                }
            }
        )*
    }
}

implement_string_for![
    Voca;
    String, str
];
