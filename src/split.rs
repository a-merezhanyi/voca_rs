//! Splits `subject` into an chuncks according to given rules.

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
/// ```
pub fn chars(subject: &str) -> Vec<&str> {
    if subject.len() == 0 {
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
/// ```
pub fn split<'a>(subject: &'a str, pattern: &str) -> Vec<&'a str> {
    if subject.len() == 0 {
        return vec![""];
    }
    if pattern.len() == 0 {
        return vec![subject];
    }
    subject.split_terminator(pattern).collect::<Vec<_>>()
}

use unicode_segmentation::UnicodeSegmentation;
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
/// ```
pub fn graphemes(subject: &str) -> Vec<&str> {
    if subject.len() == 0 {
        return vec![""];
    }
    UnicodeSegmentation::graphemes(subject, true).collect::<Vec<&str>>()
}
