//! Splits `subject` into an chuncks according to given rules.

use stfu8;
use unicode_segmentation::UnicodeSegmentation;
/// Splits `subject` into an array of characters.
///
/// # Arguments
///
/// * `subject` - The string to split into characters.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// split::chars("cloud");
/// // => ["c", "l", "o", "u", "d"]
/// use voca_rs::Voca;
/// "cloud"._chars();
/// // => ["c", "l", "o", "u", "d"]
/// ```
pub fn chars(subject: &str) -> Vec<&str> {
    if subject.is_empty() {
        return vec![""];
    }
    subject.split_terminator("").skip(1).collect::<Vec<_>>()
}

/// Splits `subject` into an array of chunks by `separator`.
///
/// # Arguments
///
/// * `subject` - The string to split into characters.
/// * `pattern` - The pattern to match the separator.
///
/// # Example
///
/// ```rust
/// use voca_rs::*;
/// split::split("rage against the dying of the light", "");
/// // => ["rage", "against", "the", "dying", "of", "the", "light"]
/// use voca_rs::Voca;
/// "rage against the dying of the light"._split("");
/// // => ["rage", "against", "the", "dying", "of", "the", "light"]
/// ```
pub fn split<'a>(subject: &'a str, pattern: &str) -> Vec<&'a str> {
    if subject.is_empty() {
        return vec![""];
    }
    if pattern.is_empty() {
        return vec![subject];
    }
    subject.split_terminator(pattern).collect::<Vec<_>>()
}

/// Splits `subject` into an array of words.
///
/// # Arguments
///
/// * `subject` - The string to split into characters.
///
/// # Example
///
/// ```rust
/// use voca_rs::*;
/// split::words("Sześć звёзд are dying");
/// // => ["Sześć", "звёзд", "are", "dying"]
/// split::words("LazyLoad with XMLHttpRequest and snake_case");
/// // => ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
/// use voca_rs::Voca;
/// "Sześć звёзд are dying"._words();
/// // => ["Sześć", "звёзд", "are", "dying"]
/// ```
pub fn words(subject: &str) -> Vec<&str> {
    fn split_camel_case(string: &str) -> Vec<&str> {
        // https://github.com/withoutboats/heck/blob/master/src/lib.rs
        #[derive(Clone, Copy, PartialEq)]
        enum WordMode {
            /// There have been no lowercase or uppercase characters in the current word.
            Boundary,
            /// The previous cased character in the current word is lowercase.
            Lowercase,
            /// The previous cased character in the current word is uppercase.
            Uppercase,
        }
        let mut words = Vec::new();
        let mut word_start = 0;
        let mut char_indices = string.char_indices().peekable();
        let mut mode = WordMode::Boundary;
        while let Some((c_idx, c)) = char_indices.next() {
            if let Some(&(next_idx, next)) = char_indices.peek() {
                let next_mode = if c.is_lowercase() {
                    WordMode::Lowercase
                } else if c.is_uppercase() {
                    WordMode::Uppercase
                } else {
                    mode
                };

                // not uppercase and next is uppercase
                if next_mode == WordMode::Lowercase && next.is_uppercase() {
                    words.push(&string[word_start..next_idx]);
                    word_start = next_idx;
                    mode = WordMode::Boundary;
                // Otherwise if current and previous are uppercase and next
                // is lowercase, word boundary before
                } else if mode == WordMode::Uppercase && c.is_uppercase() && next.is_lowercase() {
                    words.push(&string[word_start..c_idx]);
                    word_start = c_idx;
                    mode = WordMode::Boundary;
                // Otherwise no word boundary, just update the mode
                } else {
                    mode = next_mode;
                }
            }
        }
        words.push(&string[word_start..]);
        words
    }

    let splitting_punctuation = ['-', '_'];

    let split_by_whitespace_and_punctuation = subject
        .unicode_words()
        .flat_map(|w| w.split_terminator(|c| splitting_punctuation.contains(&c)))
        .filter(|w| !w.is_empty());

    let res = split_by_whitespace_and_punctuation.flat_map(split_camel_case);
    res.collect()
}

/// Splits `subject` into an array of graphemes
///
/// # Arguments
///
/// * `subject` - The string to split into characters.
///
/// # Example
///
/// ```rust
/// use voca_rs::*;
/// split::graphemes("a̐éö̲\r\n");
/// // => ["a̐", "é", "ö̲", "\r\n"]
/// use voca_rs::Voca;
/// "a̐éö̲\r\n"._graphemes();
/// // => ["a̐", "é", "ö̲", "\r\n"]
/// ```
pub fn graphemes(subject: &str) -> Vec<&str> {
    if subject.is_empty() {
        return vec![""];
    }
    UnicodeSegmentation::graphemes(subject, true).collect::<Vec<&str>>()
}

/// Returns an array of Unicode code point values from characters of `subject`.
/// NOTE: Unicode escape must not be a surrogate
///
/// # Arguments
///
/// * `subject` - The string to extract from.
///
/// # Example
///
/// ```rust
/// use voca_rs::*;
/// split::code_points("rain");
/// // => [114, 97, 105, 110]
/// split::code_points("Un garçon de café");
/// // => [85, 110, 32, 103, 97, 114, 231, 111, 110, 32, 100, 101, 32, 99, 97, 102, 233]
/// use voca_rs::Voca;
/// "rain"._code_points();
/// // => [114, 97, 105, 110]
/// ```
pub fn code_points(subject: &str) -> Vec<u16> {
    if subject.is_empty() {
        return vec![];
    }
    stfu8::decode_u16(subject).unwrap()
}
