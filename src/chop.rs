//! Extracts a character(s) from `subject`.

use stfu8;

#[derive(Clone, Copy, PartialEq)]
enum PointType {
    Length,
    Position,
}

#[derive(Clone, Copy, PartialEq)]
enum CharType {
    Simple,
    Grapheme,
}

fn get_chars(subject: &str, start: usize, end: usize) -> String {
    match subject.len() {
        0 => subject.to_string(),
        _ => crate::split::chars(&subject)[start..end].join(""),
    }
}

fn get_subject_length(
    subject: &str,
    position: usize,
    point_type: PointType,
    char_type: CharType,
) -> usize {
    let subject_len = crate::count::count_graphemes(subject);
    let position_substruction = match point_type {
        PointType::Length => 0,
        PointType::Position => 1,
    };
    let is_out_of_bounds = match char_type {
        CharType::Simple => position > subject_len,
        CharType::Grapheme => position >= subject_len,
    };
    if is_out_of_bounds {
        subject_len - position_substruction
    } else {
        position
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ReturnType {
    Normal,
    Last,
}

fn return_after_or_after_last(subject: &str, search: &str, return_type: ReturnType) -> String {
    let start_position = match return_type {
        ReturnType::Normal => crate::index::index_of(&subject, &search, 0),
        ReturnType::Last => crate::index::last_index_of(&subject, &search, 0),
    } as isize;
    if start_position == -1 {
        return "".to_owned();
    }
    let the_length = crate::count::count(&search) as isize;
    crate::chop::slice(&subject, start_position + the_length, 0)
}
/// Returns everything after the given `search`.
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `search` - The substring to look for.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::after("This is my name", "This is");
/// // => " my name"
/// chop::after("S̃o̊m̋ȩ̈ gḷ̉y̌p̆ẖs a̋řẹ̆̇ hër̵ē̱", "gḷ̉y̌p̆ẖs");
/// // => " a̋řẹ̆̇ hër̵ē̱"
/// use voca_rs::Voca;
/// "This is my name"._after("This is");
/// // => " my name"
/// ```
pub fn after(subject: &str, search: &str) -> String {
    match subject.len() {
        0 => "".to_string(),
        _ => return_after_or_after_last(&subject, &search, ReturnType::Normal),
    }
}
/// Returns everything after the last given `search`.
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `search` - The substring to look for.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::after_last("To be, or not to be, that is the question", "be,");
/// // => " that is the question"
/// chop::after_last("S̃o̊m̋ȩ̈ gḷ̉y̌p̆ẖs a̋řẹ̆̇ hër̵ē̱, but a̋řẹ̆̇ nŏt tẖër̵ē̱", "a̋řẹ̆̇");
/// // => " nŏt tẖër̵ē̱"
/// use voca_rs::Voca;
/// "To be, or not to be, that is the question"._after_last("be,");
/// // => " that is the question"
/// ```
pub fn after_last(subject: &str, search: &str) -> String {
    match subject.len() {
        0 => "".to_string(),
        _ => return_after_or_after_last(&subject, &search, ReturnType::Last),
    }
}
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
/// use voca_rs::Voca;
/// "helicopter"._char_at(0);
/// // => "h"
/// ```
pub fn char_at(subject: &str, position: usize) -> String {
    let the_position = get_subject_length(subject, position, PointType::Position, CharType::Simple);
    get_chars(&subject, the_position, the_position + 1)
}

/// Get the Unicode code point value of the character at `position`.
/// NOTE: Unicode escape must not be a surrogate
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `position` - The position to get the code point number.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::code_point_at("rain", 1);
/// // => [97]
/// chop::code_point_at("cafe\u{0301}", 4);
/// // => [101, 769]
/// use voca_rs::Voca;
/// "rain"._code_point_at(1);
/// // => [97]
/// ```
pub fn code_point_at(subject: &str, position: usize) -> Vec<u16> {
    if subject.is_empty() {
        return vec![];
    }
    let grapheme = grapheme_at(&subject, position);
    crate::split::code_points(&grapheme)
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
/// use voca_rs::Voca;
/// "helicopter"._first(1);
/// // => "h"
/// ```
pub fn first(subject: &str, length: usize) -> String {
    let the_length = get_subject_length(subject, length, PointType::Length, CharType::Simple);
    match length {
        0 => "".to_string(),
        _ => get_chars(&subject, 0, the_length),
    }
}

/// Converts the `subject` to a `foreign_key`.
///
/// # Arguments
///
/// * `subject` - The string to convert to a `foreign_key`.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::foreign_key("foo_bar");
/// // => "foo_bar_id"
/// chop::foreign_key("fooBar3");
/// // => "foo_bar3_id"
/// chop::foreign_key("Test::Foo::Bar");
/// // => "bar_id"
/// use voca_rs::Voca;
/// "foo_bar"._foreign_key();
/// // => "foo_bar_id"
/// ```
pub fn foreign_key(subject: &str) -> String {
    /* https://docs.rs/crate/Inflector/0.11.4 */
    match subject.len() {
        0 => subject.to_string(),
        _ => {
            let safe_string = if subject.contains("::") {
                let split_string: Vec<&str> = subject.split("::").collect();
                split_string[split_string.len() - 1]
            } else {
                subject
            };
            let snake_cased: String = crate::case::snake_case(safe_string);
            if snake_cased.ends_with("_id") {
                snake_cased
            } else {
                format!("{}{}", snake_cased, "_id")
            }
        }
    }
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
/// use voca_rs::Voca;
/// "cafe\u{0301}"._grapheme_at(3); // or 'café'
/// // => "é"
/// ```
pub fn grapheme_at(subject: &str, position: usize) -> String {
    let subject_len = crate::count::count_graphemes(subject);
    match subject_len {
        0 => subject.to_string(),
        _ => {
            let the_position =
                get_subject_length(subject, position, PointType::Position, CharType::Grapheme);
            crate::split::graphemes(&subject)[the_position].to_string()
        }
    }
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
/// use voca_rs::Voca;
/// "helicopter"._last(1);
/// // => "r"
/// ```
pub fn last(subject: &str, length: usize) -> String {
    match length {
        0 => "".to_string(),
        _ => {
            let subject_length = crate::split::chars(&subject).len();
            let the_length =
                get_subject_length(subject, length, PointType::Length, CharType::Grapheme);
            get_chars(&subject, subject_length - the_length, subject_length)
        }
    }
}

/// Truncates `subject` to a new `length` and does not break the words. Guarantees that the truncated string is no longer than `length`.
///
/// # Arguments
///
/// * `subject` - The string to prune.
/// * `length` - The length to prune the string.
/// * `end` - The string to be added at the end. Default value is "...".
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::prune("Once upon a time", 7, "");
/// // => "Once..."
/// chop::prune("Die Schildkröte fliegt über das Floß.", 19, "~~");
/// // => "Die Schildkröte~~"
/// chop::prune("Once upon", 10, "");
/// // => "Once upon"
/// chop::prune("Как слышно, приём!", 14, "");
/// // => "Как слышно..."
/// use voca_rs::Voca;
/// "Once upon a time"._prune(7, "");
/// // => "Once..."
/// ```
pub fn prune(subject: &str, length: usize, end: &str) -> String {
    if length == 0 {
        return "".to_string();
    }
    let mut sufix = match end {
        "" => "...",
        _ => end,
    };
    let subject_chars = crate::split::chars(&subject);
    let subject_length = subject_chars.len();
    let end_length = crate::split::chars(&sufix).len();
    let position_end = if subject_length <= length {
        sufix = "";
        subject_length
    } else {
        let string_length = length - end_length;
        let mut char_indices = subject_chars.iter();
        let mut end_position = 0;
        let mut current_position = 0;
        #[derive(Clone, Copy, PartialEq)]
        enum WordMode {
            Spaces,
            Words,
        }
        let mut mode = WordMode::Words;
        while current_position <= string_length {
            let next_char = char_indices.next();
            match next_char {
                Some(c) => {
                    let mut current_char = String::new();
                    current_char.push_str(c);
                    if crate::utils::WHITESPACE.contains(&current_char)
                        || crate::utils::PUNCTUATION.contains(&current_char)
                    {
                        if mode == WordMode::Words {
                            end_position = if current_position > 0 {
                                current_position
                            } else {
                                0
                            };
                            mode = WordMode::Spaces;
                        }
                    } else if mode == WordMode::Spaces {
                        mode = WordMode::Words;
                    }
                }
                None => {
                    return subject.to_string();
                }
            }
            current_position += 1;
        }
        end_position
    };

    format!("{}{}", get_chars(&subject, 0, position_end), sufix)
}

/// Extracts from `subject` a string from `start` position up to `end` position. The character at `end` position is not included.
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `start` - The position to start extraction. 0 means extract from the beginning of the `subject`. If negative use `subject.len() - start`.
/// * `end` - The position to end extraction. 0 means extract to the end of the `subject`. If negative use `subject.len() - end`.
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
/// use voca_rs::Voca;
/// "miami"._slice(1, 0);
/// // => "iami"
/// ```
pub fn slice(subject: &str, start: isize, end: isize) -> String {
    let subject_length = crate::split::chars(&subject).len();
    let position_start = calculate_position(subject_length, start, true);
    let position_end = calculate_position(subject_length, end, false);

    fn calculate_position(length: usize, x: isize, start: bool) -> usize {
        if x < 0 {
            let pos = length as isize - x.abs();
            if pos < 0 {
                0
            } else {
                pos as usize
            }
        } else if x == 0 {
            if start {
                0
            } else {
                length
            }
        } else if x > length as isize {
            length
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
/// use voca_rs::Voca;
/// "beach"._substr(1, 0);
/// // => "each"
/// ```
pub fn substr(subject: &str, start: usize, length: usize) -> String {
    let subject_length = crate::split::chars(&subject).len();
    if start >= subject_length {
        return "".to_string();
    }
    let position_end = match length {
        0 => subject_length,
        _ => {
            let to_position = start + length;
            if to_position > subject_length {
                subject_length
            } else {
                to_position
            }
        }
    };
    if start >= position_end {
        return "".to_string();
    }
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
/// use voca_rs::Voca;
/// "beach"._substring(1, 0);
/// // => "each"
/// ```
pub fn substring(subject: &str, start: usize, end: usize) -> String {
    let subject_length = crate::split::chars(&subject).len();
    if start >= subject_length {
        return "".to_string();
    }
    let position_end = match end {
        0 => subject_length,
        _ => {
            if end > subject_length {
                subject_length
            } else {
                end
            }
        }
    };
    if start > position_end {
        return "".to_string();
    }

    get_chars(&subject, start, position_end)
}

/// Truncates `subject` to a new `length`.
///
/// # Arguments
///
/// * `subject` - The string to truncate.
/// * `length` - The length to truncate the string.
/// * `end` - The string to be added at the end. Default value is "...".
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
/// use voca_rs::Voca;
/// "Once upon a time"._truncate(7, "");
/// // => "Once..."
/// ```
pub fn truncate(subject: &str, length: usize, end: &str) -> String {
    if length == 0 {
        return "".to_string();
    }
    let mut sufix = match end {
        "" => "...",
        _ => end,
    };
    let subject_length = crate::split::chars(&subject).len();
    let end_length = crate::split::chars(&sufix).len();
    let position_end = if subject_length < length || length < end_length {
        sufix = "";
        subject_length
    } else {
        length - end_length
    };
    format!("{}{}", get_chars(&subject, 0, position_end), sufix)
}

/// Returns the max character from the `subject` by its code point.
/// NOTE: Unicode escape must not be a surrogate
///
/// # Arguments
///
/// * `subject` - The string to extract from.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::max("rain");
/// // => "r"
/// chop::max("cafe\u{0301}"); // or "café"
/// // => "\u{0301}"
/// chop::max("a̐éö̲"); // or "a\u{310}e\u{301}o\u{308}\u{332}"
/// // => "\u{332}"
/// use voca_rs::Voca;
/// "rain"._max_code_point();
/// // => "r"
/// ```
pub fn max(subject: &str) -> String {
    if subject.is_empty() {
        return "".to_owned();
    }
    min_max(&subject, MinMaxType::Max)
}

/// Returns the min character from the `subject` by its code point.
/// NOTE: Unicode escape must not be a surrogate
///
/// # Arguments
///
/// * `subject` - The string to extract from.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::min("rain");
/// // => "a"
/// chop::min("cafe\u{0301}"); // or "café"
/// // => "a"
/// chop::min("Über das Floß.");
/// // => " "
/// use voca_rs::Voca;
/// "rain"._min_code_point();
/// // => "a"
/// ```
pub fn min(subject: &str) -> String {
    if subject.is_empty() {
        return "".to_owned();
    }
    min_max(&subject, MinMaxType::Min)
}

#[derive(Clone, Copy, PartialEq)]
enum MinMaxType {
    Min,
    Max,
}

fn min_max(subject: &str, search_type: MinMaxType) -> String {
    if subject.is_empty() {
        return "".to_owned();
    }
    let code_points = crate::split::code_points(&subject);
    let min_max = match search_type {
        MinMaxType::Max => code_points.iter().max(),
        MinMaxType::Min => code_points.iter().min(),
    };
    match min_max {
        None => "".to_owned(),
        Some(x) => stfu8::encode_u16(&[*x]),
    }
}
