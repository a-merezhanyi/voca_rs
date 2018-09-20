//! Checks a `subject` against a query.

use split;
/// Checks whether `subject` ends with `end`.
///
/// # Arguments
///
/// * `subject: &str` - The string to verify.
/// * `end: &str` - The ending string.
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
/// * `subject: &str` - The string to verify.
/// * `search: &str` - The ending string.
/// * `position: u8` - The position to start searching.
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

/// Checks whether `subject` is empty or contains only whitespaces.
///
/// # Arguments
///
/// * `subject: &str` - The string to verify.
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

/// Checks whether `subject` is empty.
///
/// # Arguments
///
/// * `subject: &str` - The string to verify.
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
/// * `subject: &str` - The string to verify.
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

/// Checks whether `subject` has only upper case characters.
///
/// # Arguments
///
/// * `subject: &str` - The string to verify.
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
/// * `subject: &str` - The string to verify.
/// * `start: &str` - The starting string.
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
