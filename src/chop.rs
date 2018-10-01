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
    let subject_length = split::chars(&subject).len();

    get_chars(&subject, subject_length - length, subject_length)
}

/// Extracts from `subject` a string from `start` position up to `end` position. The character at `end` position is not included.
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `start` - The position to start extraction. 0 means extract from the beginning of the `subject`. If negative use `subject.len() + start`.
/// * `end` - The position to end extraction. 0 means extract to the end of the `subject`. If negative use `subject.len() + end`.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::slice("miami", 1, 0);
/// // => "iami"
/// chop::slice("błąd", -2, 0);
/// // => "ąd"
/// chop::slice("florida", 1, 4);
/// // => "lor"
/// chop::slice("e\u{0301}", 1, 0); // or 'é'
/// // => "\u{0301}"
/// chop::slice("Die Schildkröte fliegt.", 4, -8);
/// // => "Schildkröte"
/// ```
pub fn slice(subject: &str, start: isize, end: isize) -> String {
    let subject_length = split::chars(&subject).len();
    let position_start = calulate_position(subject_length, start, true);
    let position_end = calulate_position(subject_length, end, false);

    fn calulate_position(length: usize, x: isize, start: bool) -> usize {
        if x < 0 {
            length - x.abs() as usize
        } else if x == 0 {
            if start {
                0
            } else {
                length
            }
        } else {
            x as usize
        }
    }

    get_chars(&subject, position_start, position_end)
}

/// Extracts from `subject` a string from `start` position a number of `length` characters.
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `start` - The position to start extraction. 0 means extract from the beginning of the `subject`.
/// * `length` - The number of characters to extract. 0 means extract to the end of `subject`.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::substr("beach", 1, 0);
/// // => "each"
/// chop::substr("błąd", 1, 2);
/// // => "łą"
/// ```
pub fn substr(subject: &str, start: usize, length: usize) -> String {
    let subject_length = split::chars(&subject).len();
    let position_end = match length {
        0 => subject_length,
        _ => start + length,
    };

    get_chars(&subject, start, position_end)
}

/// Extracts from `subject` a string from `start` position up to `end` position. The character at `end` position is not included.
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `start` - The position to start extraction. 0 means extract from the beginning of the `subject`.
/// * `end` - The position to end extraction. 0 means extract to the end of the `subject`.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::substring("beach", 1, 0);
/// // => "each"
/// chop::substring("błąd", 2, 4);
/// // => "ąd"
/// chop::substring("e\u{0301}", 1, 0); // or 'é'
/// // => "\u{0301}"
/// ```
pub fn substring(subject: &str, start: usize, end: usize) -> String {
    let subject_length = split::chars(&subject).len();
    let position_end = match end {
        0 => subject_length,
        _ => end,
    };

    get_chars(&subject, start, position_end)
}

/// Truncates `subject` to a new `length`.
///
/// # Arguments
///
/// * `subject` - The string to truncate.
/// * `length` - The length to truncate the string.
/// * `end` - The string to be added at the end.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::truncate("Once upon a time", 7, "");
/// // => "Once..."
/// chop::truncate("Die Schildkröte fliegt über das Floß.", 28, "(...)");
/// // => "Die Schildkröte fliegt (...)"
/// chop::truncate("Once upon", 10, "");
/// // => "Once upon"
/// ```
pub fn truncate(subject: &str, length: usize, end: &str) -> String {
    if length == 0 {
        return "".to_string();
    }
    let mut sufix = match end {
        "" => "...",
        _ => end,
    };
    let subject_length = split::chars(&subject).len();
    let end_length = split::chars(&sufix).len();
    let position_end = if subject_length < length {
        sufix = "";
        subject_length
    } else {
        length - end_length
    };
    format!("{}{}", get_chars(&subject, 0, position_end), sufix)
}
