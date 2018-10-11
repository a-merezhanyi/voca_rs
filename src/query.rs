//! Checks a `subject` against a query.

use chop;
use count;
use split;
use utils;
/// Checks whether `subject` ends with `end`.
///
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `end` - The ending string.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::ends_with("say hello to my little friend", "little friend");
/// // => true
/// query::ends_with("say hello to my little friend", "little");
/// // => false
/// ```
pub fn ends_with(subject: &str, end: &str) -> bool {
    if subject.len() == 0 || end.len() == 0 {
        return true;
    }
    subject.ends_with(end)
}

/// Checks whether `subject` includes `search` starting from `position`.
///
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `search` - The ending string.
/// * `position` - The position to start searching.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::includes("starship", "star", 0);
/// // => true
/// query::includes("Zażółć gęślą jaźń", "gęślą", 7);
/// // => true
/// query::includes("galaxy", "g", 1);
/// // => false
/// ```
pub fn includes(subject: &str, search: &str, position: usize) -> bool {
    if subject.len() == 0 || search.len() == 0 {
        return true;
    }
    subject.to_owned()[subject.char_indices().nth(position).unwrap().0..]
        .to_string()
        .contains(&search)
}

/// Checks whether `subject` contains only alpha characters.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_alpha("");
/// // => false
/// query::is_alpha("cafe\u{0301}"); // or "café"
/// // => true
/// query::is_alpha("bart");
/// // => true
/// query::is_alpha("lisa!");
/// // => false
/// query::is_alpha("Zażółć and bart");
/// // => false
/// ```
pub fn is_alpha(subject: &str) -> bool {
    match is_empty(&subject) {
        true => false,
        _ => is_alpha_or_alphadigit(&subject, false),
    }
}

fn is_alpha_or_alphadigit(subject: &str, count_digits: bool) -> bool {
    let mut subject_is_ok = true;
    let subject_grapheme_len = count::count_graphemes(&subject);
    let mut current_pos = 0;
    while current_pos < subject_grapheme_len {
        let current_char = chop::grapheme_at(&subject, current_pos);
        if (!count_digits
            && (is_digit(&current_char)
                || is_blank(&current_char)
                || utils::PUNCTUATION.contains(&current_char)))
            || (count_digits
                && (is_blank(&current_char) || utils::PUNCTUATION.contains(&current_char)))
        {
            subject_is_ok = false;
            current_pos = subject_grapheme_len;
        } else {
            current_pos = current_pos + 1;
        }
    }
    subject_is_ok
}

/// Checks whether `subject` contains contains only alpha and digit characters.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_alphadigit("");
/// // => false
/// query::is_alphadigit("cafe\u{0301}"); // or "café"
/// // => true
/// query::is_alphadigit("year2020");
/// // => true
/// query::is_alphadigit("1448");
/// // => true
/// query::is_alphadigit("40-20");
/// // => false
/// ```
pub fn is_alphadigit(subject: &str) -> bool {
    match is_empty(&subject) {
        true => false,
        _ => is_alpha_or_alphadigit(&subject, true),
    }
}

/// Checks whether `subject` is empty or contains only whitespaces.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_blank("");
/// // => true
/// query::is_blank("   ");
/// // => true
/// query::is_blank("sun");
/// // => false
/// ```
pub fn is_blank(subject: &str) -> bool {
    if subject.len() == 0 {
        return true;
    }

    return subject.trim().is_empty();
}

/// Checks whether `subject` contains only digit characters.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_digit("35");
/// // => true
/// query::is_digit("1.5");
/// // => false
/// query::is_digit("0xFF");
/// // => false
/// query::is_digit("ten");
/// // => false
/// ```
pub fn is_digit(subject: &str) -> bool {
    let subject_len = subject.len();
    if subject_len == 0 {
        return true;
    }

    split::chars(&subject)
        .iter()
        .filter(|c| {
            let mut current_char = String::new();
            current_char.push_str(c);
            utils::DIGITS.contains(&current_char)
        }).collect::<Vec<_>>()
        .len()
        == subject_len
}

/// Checks whether `subject` is empty.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_empty("");
/// // => true
/// query::is_empty("   ");
/// // => false
/// query::is_empty("sun");
/// // => false
/// ```
pub fn is_empty(subject: &str) -> bool {
    if subject.len() == 0 {
        return true;
    }

    return false;
}

/// Checks whether `subject` has only lower case characters.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_lowercase("motorcycle");
/// // => true
/// query::is_lowercase("John");
/// // => false
/// query::is_lowercase("T1000");
/// // => false
/// ```
pub fn is_lowercase(subject: &str) -> bool {
    is_upper_or_lowercase(subject, true)
}

/// Checks whether `subject` is numeric.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_numeric("350");
/// // => true
/// query::is_numeric("-20.5");
/// // => true
/// query::is_numeric("five");
/// // => false
/// ```
// TODO: add scientific numbers validation
// assert_eq!(query::is_numeric("1.5E+2"), true);
// probably via regexp
pub fn is_numeric(subject: &str) -> bool {
    if subject.len() == 0 {
        return true;
    }

    match subject.find('.') {
        Some(_) => match subject.parse::<f32>() {
            Ok(_) => true,
            Err(_) => false,
        },
        None => match subject.parse::<i32>() {
            Ok(_) => true,
            Err(_) => false,
        },
    }
}

/// Checks whether `subject` has only upper case characters.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_uppercase("ACDC");
/// // => true
/// query::is_uppercase("Morning");
/// // => false
/// ```
pub fn is_uppercase(subject: &str) -> bool {
    is_upper_or_lowercase(subject, false)
}

fn is_upper_or_lowercase(subject: &str, lowercase: bool) -> bool {
    if subject.len() == 0 {
        return true;
    }

    let mut res = true;
    split::chars(subject).into_iter().for_each(|s| {
        s.chars().for_each(|c| {
            if lowercase && c.is_uppercase() {
                res = false;
            } else if !lowercase && c.is_lowercase() {
                res = false;
            }
        })
    });
    res
}

/// Checks whether `subject` starts with `start`.
///
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `start` - The starting string.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::starts_with("say hello to my little friend", "say hello");
/// // => true
/// query::starts_with("say hello to my little friend", "hello");
/// // => flase
/// ```
pub fn starts_with(subject: &str, start: &str) -> bool {
    if subject.len() == 0 || start.len() == 0 {
        return true;
    }
    subject.starts_with(start)
}
