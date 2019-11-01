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
    fn char_at(&self, param: usize) -> String;
    fn code_point_at(&self, param: usize) -> Vec<u16>;
    fn first(&self, param: usize) -> String;
    fn foreign_key(&self) -> String;
    fn grapheme_at(&self, param: usize) -> String;
    fn last(&self, param: usize) -> String;
    fn prune(&self, param1: usize, param2: &str) -> String;
    fn slice(&self, param1: isize, param2: isize) -> String;
    fn substr(&self, param1: usize, param2: usize) -> String;
    fn substring(&self, param1: usize, param2: usize) -> String;
    fn truncate(&self, param1: usize, param2: &str) -> String;
    fn max_code_point(&self) -> String;
    fn min_code_point(&self) -> String;
    // count
    fn count(&self) -> usize;
    fn count_graphemes(&self) -> usize;
    fn count_substrings(&self, param1: &str) -> usize;
    fn count_words(&self, param1: &str) -> usize;
    // escape
    fn escape_html(&self) -> String;
    fn escape_regexp(&self) -> String;
    fn unescape_html(&self) -> String;
    // index
    fn index_all(&self, param1: &str, param2: usize) -> Vec<usize>;
    fn index_of(&self, param1: &str, param2: usize) -> i8;
    fn last_index_of(&self, param1: &str, param2: usize) -> i8;
    fn search(&self, param1: &str, param2: usize) -> i8;
    // manipulate
    fn expand_tabs(&self, param1: usize) -> String;
    fn expand_spaces(&self, param1: usize) -> String;
    fn insert(&self, param1: &str, param2: usize) -> String;
    fn latinise(&self) -> String;
    fn pad(&self, param1: usize, param2: &str) -> String;
    fn pad_left(&self, param1: usize, param2: &str) -> String;
    fn pad_right(&self, param1: usize, param2: &str) -> String;
    fn repeat(&self, param1: usize) -> String;
    fn replace(&self, param1: &str, param2: &str) -> String;
    fn replace_all(&self, param1: &str, param2: &str) -> String;
    fn reverse(&self) -> String;
    fn reverse_grapheme(&self) -> String;
    fn slugify(&self) -> String;
    fn splice(&self, param1: isize, param2: usize, param3: &str) -> String;
    fn trim_me(&self, param1: &str) -> String;
    fn trim_right_me(&self, param1: &str) -> String;
    fn trim_left_me(&self, param1: &str) -> String;
    fn zfill(&self, param1: usize) -> String;
    fn tr(&self, param1: &str, param2: &str) -> String;
    fn word_wrap(&self, param1: usize, param2: &str, param3: &str) -> String;
    // query
    fn is_foreign_key(&self) -> bool;
    fn ends_with(&self, param1: &str) -> bool;
    fn includes(&self, param1: &str, param2: usize) -> bool;
    fn is_alpha(&self) -> bool;
    fn is_alphadigit(&self) -> bool;
    fn is_blank(&self) -> bool;
    fn is_camel_case(&self) -> bool;
    fn is_capitalize(&self) -> bool;
    fn is_decapitalize(&self) -> bool;
    fn is_digit(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_lowercase(&self) -> bool;
    fn is_lower_first(&self) -> bool;
    fn is_kebab_case(&self) -> bool;
    // split

    // strip
}

macro_rules! implement_string_for {
    ( $trt:ident; $($typ:ident), *) => {
        $(
            impl $trt for $typ {
                // case
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
                // chop
                fn char_at(&self, param: usize) -> String {
                    chop::char_at(&self, param)
                }
                fn code_point_at(&self, param: usize) -> Vec<u16> {
                    chop::code_point_at(&self, param)
                }
                fn first(&self, param: usize) -> String {
                    chop::first(&self, param)
                }
                fn foreign_key(&self) -> String {
                    chop::foreign_key(&self)
                }
                fn grapheme_at(&self, param: usize) -> String {
                    chop::grapheme_at(&self, param)
                }
                fn last(&self, param: usize) -> String {
                    chop::last(&self, param)
                }
                fn prune(&self, param1: usize, param2: &str) -> String {
                    chop::prune(&self, param1, param2)
                }
                fn slice(&self, param1: isize, param2: isize) -> String {
                    chop::slice(&self, param1, param2)
                }
                fn substr(&self, param1: usize, param2: usize) -> String {
                    chop::substr(&self, param1, param2)
                }
                fn substring(&self, param1: usize, param2: usize) -> String {
                    chop::substring(&self, param1, param2)
                }
                fn truncate(&self, param1: usize, param2: &str) -> String {
                    chop::truncate(&self, param1, param2)
                }
                fn max_code_point(&self) -> String {
                    chop::max(&self)
                }
                fn min_code_point(&self) -> String {
                    chop::min(&self)
                }
                // count
                fn count(&self) -> usize {
                    count::count(&self)
                }
                fn count_graphemes(&self) -> usize {
                    count::count_graphemes(&self)
                }
                fn count_substrings(&self, param1: &str) -> usize {
                    count::count_substrings(&self, param1)
                }
                fn count_words(&self, param1: &str) -> usize {
                    count::count_words(&self, param1)
                }
                // escape
                fn escape_html(&self) -> String {
                    escape::escape_html(&self)
                }
                fn escape_regexp(&self) -> String {
                    escape::escape_regexp(&self)
                }
                fn unescape_html(&self) -> String {
                    escape::unescape_html(&self)
                }
                // index
                fn index_all(&self, param1: &str, param2: usize) -> Vec<usize> {
                    index::index_all(&self, param1, param2)
                }
                fn index_of(&self, param1: &str, param2: usize) -> i8 {
                    index::index_of(&self, param1, param2)
                }
                fn last_index_of(&self, param1: &str, param2: usize) -> i8 {
                    index::last_index_of(&self, param1, param2)
                }
                fn search(&self, param1: &str, param2: usize) -> i8 {
                    index::search(&self, param1, param2)
                }
                // manipulate
                fn expand_tabs(&self, param1: usize) -> String {
                    manipulate::expand_tabs(&self, param1)
                }
                fn expand_spaces(&self, param1: usize) -> String {
                    manipulate::expand_spaces(&self, param1)
                }
                fn insert(&self, param1: &str, param2: usize) -> String {
                    manipulate::insert(&self, param1, param2)
                }
                fn latinise(&self) -> String {
                    manipulate::latinise(&self)
                }
                fn pad(&self, param1: usize, param2: &str ) -> String {
                    manipulate::pad(&self, param1, param2)
                }
                fn pad_left(&self, param1: usize, param2: &str ) -> String {
                    manipulate::pad_left(&self, param1, param2)
                }
                fn pad_right(&self, param1: usize, param2: &str ) -> String {
                    manipulate::pad_right(&self, param1, param2)
                }
                fn repeat(&self, param1: usize) -> String {
                    manipulate::repeat(&self, param1)
                }
                fn replace(&self, param1: &str, param2: &str ) -> String {
                    manipulate::replace(&self, param1, param2)
                }
                fn replace_all(&self, param1: &str, param2: &str) -> String {
                    manipulate::replace_all(&self, param1, param2)
                }
                fn reverse(&self) -> String {
                    manipulate::reverse(&self)
                }
                fn reverse_grapheme(&self) -> String {
                    manipulate::reverse_grapheme(&self)
                }
                fn slugify(&self) -> String {
                    manipulate::slugify(&self)
                }
                fn splice(&self, param1: isize, param2: usize, param3: &str) -> String {
                    manipulate::splice(&self, param1, param2, param3)
                }
                fn trim_me(&self, param1: &str) -> String {
                    manipulate::trim(&self, param1)
                }
                fn trim_right_me(&self, param1: &str) -> String {
                    manipulate::trim_right(&self, param1)
                }
                fn trim_left_me(&self, param1: &str) -> String {
                    manipulate::trim_left(&self, param1)
                }
                fn zfill(&self, param1: usize) -> String {
                    manipulate::zfill(&self, param1)
                }
                fn tr(&self, param1: &str, param2: &str) -> String {
                    manipulate::tr(&self, param1, param2)
                }
                fn word_wrap(&self, param1: usize, param2: &str, param3: &str) -> String {
                    manipulate::word_wrap(&self, param1, param2, param3)
                }
                // query
                fn is_foreign_key(&self) -> bool {
                    query::is_foreign_key(&self)
                }
                fn ends_with(&self, param1: &str) -> bool {
                    query::ends_with(&self, param1)
                }
                fn includes(&self, param1: &str, param2: usize) -> bool {
                    query::includes(&self, param1, param2)
                }
                fn is_alpha(&self) -> bool {
                    query::is_alpha(&self)
                }
                fn is_alphadigit(&self) -> bool {
                    query::is_alphadigit(&self)
                }
                fn is_blank(&self) -> bool {
                    query::is_blank(&self)
                }
                fn is_camel_case(&self) -> bool {
                    query::is_camel_case(&self)
                }
                fn is_capitalize(&self) -> bool {
                    query::is_capitalize(&self)
                }
                fn is_decapitalize(&self) -> bool {
                    query::is_decapitalize(&self)
                }
                fn is_digit(&self) -> bool {
                    query::is_digit(&self)
                }
                fn is_empty(&self) -> bool {
                    query::is_empty(&self)
                }
                fn is_lowercase(&self) -> bool {
                    query::is_lowercase(&self)
                }
                fn is_lower_first(&self) -> bool {
                    query::is_lower_first(&self)
                }
                fn is_kebab_case(&self) -> bool {
                    query::is_kebab_case(&self)
                }
            }
        )*
    }
}

implement_string_for![
    Voca;
    String, str
];
