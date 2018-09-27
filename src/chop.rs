//! Extracts a character(s) from `subject`.

use split;
/// Access a character from `subject` at specified `position`.
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `position` - The position to get the character.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::char_at("helicopter", 0);
/// // => "h"
/// chop::char_at("błąd", 1);
/// // => "ł"
/// ```
pub fn char_at(subject: &str, position: usize) -> String {
    get_chars(&subject, position, position + 1)
}

fn get_chars(subject: &str, start: usize, end: usize) -> String {
    if subject.len() == 0 {
        return subject.to_string();
    }

    split::chars(&subject)[start..end].join("")
}

/// Extracts the first `length` characters from `subject`.
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `length` - The number of characters to extract.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::first("helicopter", 1);
/// // => "h"
/// chop::first("błąd", 2);
/// // => "bł"
/// chop::first("e\u{0301}", 1); // or 'é'
/// // => "e"
/// ```
pub fn first(subject: &str, length: usize) -> String {
    if length == 0 {
        return "".to_string();
    }

    get_chars(&subject, 0, length)
}

/// Get a grapheme from `subject` at specified `position`.
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `position` - The position to get the grapheme.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::grapheme_at("cafe\u{0301}", 3); // or 'café'
/// // => "é"
/// chop::grapheme_at("a̐éö̲", 0);
/// // => "a̐"
/// ```
pub fn grapheme_at(subject: &str, position: usize) -> String {
    if subject.len() == 0 {
        return subject.to_string();
    }

    split::graphemes(&subject)[position].to_string()
}

/// Extracts the last `length` characters from `subject`.
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `length` - The number of characters to extract.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::last("helicopter", 1);
/// // => "r"
/// chop::last("błąd", 2);
/// // => "ąd"
/// chop::last("e\u{0301}", 1); // or 'é'
/// // => "\u{0301}"
/// ```
pub fn last(subject: &str, length: usize) -> String {
    if length == 0 {
        return "".to_string();
    }
    let subject_lenght = split::chars(&subject).len();

    get_chars(&subject, subject_lenght - length, subject_lenght)
}
