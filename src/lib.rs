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
//!
//! Using functions:
//! ```rust
//! use voca_rs::*;
//! let input_string = "LazyLoad with XMLHttpRequest and snake_case";
//! let string_in_words = split::words(&input_string);
//! // => ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
//! let words_in_string = &string_in_words.join(" ");
//! // => "Lazy Load with XML Http Request and snake case"
//! let truncated_string = chop::prune(&words_in_string, 21, "");
//! // => "Lazy Load with XML..."
//! let sliced_string = chop::slice(&truncated_string, 5, -2);
//! // => "Load with XML."
//! let snaked_string = case::snake_case(&sliced_string);
//! // => "load_with_xml"
//! ```
//!
//! Using traits (all methods start from the underscore symbol):
//! ```rust
//! use voca_rs::Voca;
//! "LazyLoad with XMLHttpRequest and snake_case"
//! ._words()
//! // => ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
//! .join(" ")
//! // => "Lazy Load with XML Http Request and snake case"
//! ._prune(21, "")
//! // => "Lazy Load with XML..."
//! ._slice(5, -2)
//! // => "Load with XML."
//! ._snake_case();
//! // => "load_with_xml"
//! ```

extern crate regex;
extern crate stfu8;
extern crate unicode_segmentation;

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
    fn _camel_case(&self) -> String;
    fn _capitalize(&self, param: bool) -> String;
    fn _decapitalize(&self, param: bool) -> String;
    fn _kebab_case(&self) -> String;
    fn _shouty_kebab_case(&self) -> String;
    fn _lower_case(&self) -> String;
    fn _pascal_case(&self) -> String;
    fn _snake_case(&self) -> String;
    fn _shouty_snake_case(&self) -> String;
    fn _swap_case(&self) -> String;
    fn _title_case(&self) -> String;
    fn _train_case(&self) -> String;
    fn _upper_case(&self) -> String;
    fn _lower_first(&self) -> String;
    fn _upper_first(&self) -> String;
    // chop
    fn _after(&self, param: &str) -> String;
    fn _after_last(&self, param: &str) -> String;
    fn _before(&self, param: &str) -> String;
    fn _before_last(&self, param: &str) -> String;
    fn _char_at(&self, param: usize) -> String;
    fn _code_point_at(&self, param: usize) -> Vec<u16>;
    fn _first(&self, param: usize) -> String;
    fn _foreign_key(&self) -> String;
    fn _grapheme_at(&self, param: usize) -> String;
    fn _last(&self, param: usize) -> String;
    fn _prune(&self, param1: usize, param2: &str) -> String;
    fn _removeprefix(&self, param2: &str) -> String;
    fn _slice(&self, param1: isize, param2: isize) -> String;
    fn _substr(&self, param1: usize, param2: usize) -> String;
    fn _substring(&self, param1: usize, param2: usize) -> String;
    fn _truncate(&self, param1: usize, param2: &str) -> String;
    fn _max_code_point(&self) -> String;
    fn _min_code_point(&self) -> String;
    // count
    fn _count(&self) -> usize;
    fn _count_graphemes(&self) -> usize;
    fn _count_substrings(&self, param1: &str) -> usize;
    fn _count_where(&self, param2: fn(&str) -> bool) -> usize;
    fn _count_words(&self, param1: &str) -> usize;
    fn _count_unique_words(&self, param1: &str) -> usize;
    // escape
    fn _escape_html(&self) -> String;
    fn _escape_regexp(&self) -> String;
    fn _unescape_html(&self) -> String;
    // index
    fn _index_all(&self, param1: &str, param2: usize) -> Vec<usize>;
    fn _index_of(&self, param1: &str, param2: usize) -> i8;
    fn _last_index_of(&self, param1: &str, param2: usize) -> i8;
    fn _search(&self, param1: &str, param2: usize) -> i8;
    // manipulate
    fn _expand_tabs(&self, param1: usize) -> String;
    fn _expand_spaces(&self, param1: usize) -> String;
    fn _insert(&self, param1: &str, param2: usize) -> String;
    fn _latinise(&self) -> String;
    fn _pad(&self, param1: usize, param2: &str) -> String;
    fn _pad_left(&self, param1: usize, param2: &str) -> String;
    fn _pad_right(&self, param1: usize, param2: &str) -> String;
    fn _repeat(&self, param1: usize) -> String;
    fn _replace(&self, param1: &str, param2: &str) -> String;
    fn _replace_all(&self, param1: &str, param2: &str) -> String;
    fn _reverse(&self) -> String;
    fn _reverse_grapheme(&self) -> String;
    fn _slugify(&self) -> String;
    fn _splice(&self, param1: isize, param2: usize, param3: &str) -> String;
    fn _trim(&self, param1: &str) -> String;
    fn _trim_right(&self, param1: &str) -> String;
    fn _trim_left(&self, param1: &str) -> String;
    fn _zfill(&self, param1: usize) -> String;
    fn _tr(&self, param1: &str, param2: &str) -> String;
    fn _word_wrap(&self, param1: usize, param2: &str, param3: &str) -> String;
    // query
    fn _is_foreign_key(&self) -> bool;
    fn _ends_with(&self, param1: &str) -> bool;
    fn _includes(&self, param1: &str, param2: usize) -> bool;
    fn _is_alpha(&self) -> bool;
    fn _is_alphadigit(&self) -> bool;
    fn _is_blank(&self) -> bool;
    fn _is_camel_case(&self) -> bool;
    fn _is_capitalize(&self) -> bool;
    fn _is_decapitalize(&self) -> bool;
    fn _is_digit(&self) -> bool;
    fn _is_empty(&self) -> bool;
    fn _is_lowercase(&self) -> bool;
    fn _is_lower_first(&self) -> bool;
    fn _is_kebab_case(&self) -> bool;
    fn _is_numeric(&self) -> bool;
    fn _is_pascal_case(&self) -> bool;
    fn _is_shouty_kebab_case(&self) -> bool;
    fn _is_snake_case(&self) -> bool;
    fn _is_shouty_snake_case(&self) -> bool;
    fn _is_title(&self) -> bool;
    fn _is_train_case(&self) -> bool;
    fn _is_uppercase(&self) -> bool;
    fn _is_upper_first(&self) -> bool;
    fn _matches(&self, param1: &str, param2: usize) -> bool;
    fn _query(&self, param1: &str, param2: usize) -> bool;
    fn _starts_with(&self, param1: &str) -> bool;
    // split
    fn _chars(&self) -> Vec<&str>;
    fn _split(&self, param1: &str) -> Vec<&str>;
    fn _words(&self) -> Vec<&str>;
    fn _graphemes(&self) -> Vec<&str>;
    fn _code_points(&self) -> Vec<u16>;
    // strip
    fn _strip_bom(&self) -> String;
    fn _strip_tags(&self) -> String;
}

macro_rules! implement_string_for {
    ( $trt:ident; $($typ:ident), *) => {
        $(
            impl $trt for $typ {
                // case
                fn _camel_case(&self) -> String {
                    case::camel_case(&self)
                }
                fn _capitalize(&self, param: bool) -> String {
                    case::capitalize(&self, param)
                }
                fn _decapitalize(&self, param: bool) -> String {
                    case::decapitalize(&self, param)
                }
                fn _kebab_case(&self) -> String {
                    case::kebab_case(&self)
                }
                fn _shouty_kebab_case(&self) -> String {
                    case::shouty_kebab_case(&self)
                }
                fn _lower_case(&self) -> String {
                    case::lower_case(&self)
                }
                fn _pascal_case(&self) -> String {
                    case::pascal_case(&self)
                }
                fn _snake_case(&self) -> String {
                    case::snake_case(&self)
                }
                fn _shouty_snake_case(&self) -> String {
                    case::shouty_snake_case(&self)
                }
                fn _swap_case(&self) -> String {
                    case::swap_case(&self)
                }
                fn _title_case(&self) -> String {
                    case::title_case(&self)
                }
                fn _train_case(&self) -> String {
                    case::train_case(&self)
                }
                fn _upper_case(&self) -> String {
                    case::upper_case(&self)
                }
                fn _lower_first(&self) -> String {
                    case::lower_first(&self)
                }
                fn _upper_first(&self) -> String {
                    case::upper_first(&self)
                }
                // chop
                fn _after(&self, param: &str) -> String {
                    chop::after(&self, param)
                }
                fn _after_last(&self, param: &str) -> String {
                    chop::after_last(&self, param)
                }
                fn _before(&self, param: &str) -> String {
                    chop::before(&self, param)
                }
                fn _before_last(&self, param: &str) -> String {
                    chop::before_last(&self, param)
                }
                fn _char_at(&self, param: usize) -> String {
                    chop::char_at(&self, param)
                }
                fn _code_point_at(&self, param: usize) -> Vec<u16> {
                    chop::code_point_at(&self, param)
                }
                fn _first(&self, param: usize) -> String {
                    chop::first(&self, param)
                }
                fn _foreign_key(&self) -> String {
                    chop::foreign_key(&self)
                }
                fn _grapheme_at(&self, param: usize) -> String {
                    chop::grapheme_at(&self, param)
                }
                fn _last(&self, param: usize) -> String {
                    chop::last(&self, param)
                }
                fn _prune(&self, param1: usize, param2: &str) -> String {
                    chop::prune(&self, param1, param2)
                }
                fn _removeprefix(&self, param2: &str) -> String {
                    chop::removeprefix(&self, param2)
                }
                fn _slice(&self, param1: isize, param2: isize) -> String {
                    chop::slice(&self, param1, param2)
                }
                fn _substr(&self, param1: usize, param2: usize) -> String {
                    chop::substr(&self, param1, param2)
                }
                fn _substring(&self, param1: usize, param2: usize) -> String {
                    chop::substring(&self, param1, param2)
                }
                fn _truncate(&self, param1: usize, param2: &str) -> String {
                    chop::truncate(&self, param1, param2)
                }
                fn _max_code_point(&self) -> String {
                    chop::max(&self)
                }
                fn _min_code_point(&self) -> String {
                    chop::min(&self)
                }
                // count
                fn _count(&self) -> usize {
                    count::count(&self)
                }
                fn _count_graphemes(&self) -> usize {
                    count::count_graphemes(&self)
                }
                fn _count_substrings(&self, param1: &str) -> usize {
                    count::count_substrings(&self, param1)
                }
                fn _count_where(&self, param1: fn(&str) -> bool) -> usize {
                    count::count_where(&self, param1)
                }
                fn _count_words(&self, param1: &str) -> usize {
                    count::count_words(&self, param1)
                }
                fn _count_unique_words(&self, param1: &str) -> usize {
                    count::count_unique_words(&self, param1)
                }
                // escape
                fn _escape_html(&self) -> String {
                    escape::escape_html(&self)
                }
                fn _escape_regexp(&self) -> String {
                    escape::escape_regexp(&self)
                }
                fn _unescape_html(&self) -> String {
                    escape::unescape_html(&self)
                }
                // index
                fn _index_all(&self, param1: &str, param2: usize) -> Vec<usize> {
                    index::index_all(&self, param1, param2)
                }
                fn _index_of(&self, param1: &str, param2: usize) -> i8 {
                    index::index_of(&self, param1, param2)
                }
                fn _last_index_of(&self, param1: &str, param2: usize) -> i8 {
                    index::last_index_of(&self, param1, param2)
                }
                fn _search(&self, param1: &str, param2: usize) -> i8 {
                    index::search(&self, param1, param2)
                }
                // manipulate
                fn _expand_tabs(&self, param1: usize) -> String {
                    manipulate::expand_tabs(&self, param1)
                }
                fn _expand_spaces(&self, param1: usize) -> String {
                    manipulate::expand_spaces(&self, param1)
                }
                fn _insert(&self, param1: &str, param2: usize) -> String {
                    manipulate::insert(&self, param1, param2)
                }
                fn _latinise(&self) -> String {
                    manipulate::latinise(&self)
                }
                fn _pad(&self, param1: usize, param2: &str ) -> String {
                    manipulate::pad(&self, param1, param2)
                }
                fn _pad_left(&self, param1: usize, param2: &str ) -> String {
                    manipulate::pad_left(&self, param1, param2)
                }
                fn _pad_right(&self, param1: usize, param2: &str ) -> String {
                    manipulate::pad_right(&self, param1, param2)
                }
                fn _repeat(&self, param1: usize) -> String {
                    manipulate::repeat(&self, param1)
                }
                fn _replace(&self, param1: &str, param2: &str ) -> String {
                    manipulate::replace(&self, param1, param2)
                }
                fn _replace_all(&self, param1: &str, param2: &str) -> String {
                    manipulate::replace_all(&self, param1, param2)
                }
                fn _reverse(&self) -> String {
                    manipulate::reverse(&self)
                }
                fn _reverse_grapheme(&self) -> String {
                    manipulate::reverse_grapheme(&self)
                }
                fn _slugify(&self) -> String {
                    manipulate::slugify(&self)
                }
                fn _splice(&self, param1: isize, param2: usize, param3: &str) -> String {
                    manipulate::splice(&self, param1, param2, param3)
                }
                fn _trim(&self, param1: &str) -> String {
                    manipulate::trim(&self, param1)
                }
                fn _trim_right(&self, param1: &str) -> String {
                    manipulate::trim_right(&self, param1)
                }
                fn _trim_left(&self, param1: &str) -> String {
                    manipulate::trim_left(&self, param1)
                }
                fn _zfill(&self, param1: usize) -> String {
                    manipulate::zfill(&self, param1)
                }
                fn _tr(&self, param1: &str, param2: &str) -> String {
                    manipulate::tr(&self, param1, param2)
                }
                fn _word_wrap(&self, param1: usize, param2: &str, param3: &str) -> String {
                    manipulate::word_wrap(&self, param1, param2, param3)
                }
                // query
                fn _is_foreign_key(&self) -> bool {
                    query::is_foreign_key(&self)
                }
                fn _ends_with(&self, param1: &str) -> bool {
                    query::ends_with(&self, param1)
                }
                fn _includes(&self, param1: &str, param2: usize) -> bool {
                    query::includes(&self, param1, param2)
                }
                fn _is_alpha(&self) -> bool {
                    query::is_alpha(&self)
                }
                fn _is_alphadigit(&self) -> bool {
                    query::is_alphadigit(&self)
                }
                fn _is_blank(&self) -> bool {
                    query::is_blank(&self)
                }
                fn _is_camel_case(&self) -> bool {
                    query::is_camel_case(&self)
                }
                fn _is_capitalize(&self) -> bool {
                    query::is_capitalize(&self)
                }
                fn _is_decapitalize(&self) -> bool {
                    query::is_decapitalize(&self)
                }
                fn _is_digit(&self) -> bool {
                    query::is_digit(&self)
                }
                fn _is_empty(&self) -> bool {
                    query::is_empty(&self)
                }
                fn _is_lowercase(&self) -> bool {
                    query::is_lowercase(&self)
                }
                fn _is_lower_first(&self) -> bool {
                    query::is_lower_first(&self)
                }
                fn _is_kebab_case(&self) -> bool {
                    query::is_kebab_case(&self)
                }
                fn _is_numeric(&self) -> bool {
                    query::is_numeric(&self)
                }
                fn _is_pascal_case(&self) -> bool {
                    query::is_pascal_case(&self)
                }
                fn _is_shouty_kebab_case(&self) -> bool {
                    query::is_shouty_kebab_case(&self)
                }
                fn _is_snake_case(&self) -> bool {
                    query::is_snake_case(&self)
                }
                fn _is_shouty_snake_case(&self) -> bool {
                    query::is_shouty_snake_case(&self)
                }
                fn _is_title(&self) -> bool {
                    query::is_title(&self)
                }
                fn _is_train_case(&self) -> bool {
                    query::is_train_case(&self)
                }
                fn _is_uppercase(&self) -> bool {
                    query::is_uppercase(&self)
                }
                fn _is_upper_first(&self) -> bool {
                    query::is_upper_first(&self)
                }
                fn _matches(&self, param1: &str, param2: usize) -> bool {
                    query::matches(&self, param1, param2)
                }
                fn _query(&self, param1: &str, param2: usize) -> bool {
                    query::query(&self, param1, param2)
                }
                fn _starts_with(&self, param1: &str) -> bool {
                    query::starts_with(&self, param1)
                }
                // split
                fn _chars(&self) -> Vec<&str> {
                    split::chars(&self)
                }
                fn _split(&self, param1: &str) -> Vec<&str> {
                    split::split(&self, param1)
                }
                fn _words(&self) -> Vec<&str> {
                    split::words(&self)
                }
                fn _graphemes(&self) -> Vec<&str> {
                    split::graphemes(&self)
                }
                fn _code_points(&self) -> Vec<u16> {
                    split::code_points(&self)
                }
                // strip
                fn _strip_bom(&self) -> String {
                    strip::strip_bom(&self)
                }
                fn _strip_tags(&self) -> String {
                    strip::strip_tags(&self)
                }
            }
        )*
    }
}

implement_string_for![
    Voca;
    String, str
];
