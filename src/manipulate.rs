//! Manipulate with the `subject`.

use split;
/// Inserts into `subject` a string `to_insert` at specified `position`.
///
/// # Arguments
///
/// * `subject` - The string where to insert.
/// * `to_insert` - The string to be inserted
/// * `position` - The position to insert.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::insert("ct", "a", 1);
/// // => "cat"
/// manipulate::insert("sunny", " day", 5);
/// // => "sunny day"
/// ```
pub fn insert(subject: &str, to_insert: &str, position: usize) -> String {
    let subject_len = subject.len();
    if subject_len == 0 || to_insert.len() == 0 {
        return subject.to_string();
    }
    let insert_position = if position >= subject_len {
        subject_len - 1
    } else {
        position
    };
    let prefix = split::chars(&subject)[..insert_position].join("");
    let sufix = split::chars(&subject)[insert_position..].join("");
    format!("{}{}{}", prefix, to_insert, sufix)
}

/// Repeats the `subject` number of `times`.
///
/// # Arguments
///
/// * `subject` - The string to repeat.
/// * `times` - The number of times to repeat.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::repeat("w", 3);
/// // => "www"
/// manipulate::repeat("world", 0);
/// // => ""
/// ```
pub fn repeat(subject: &str, times: usize) -> String {
    if subject.len() == 0 || times == 0 {
        return "".to_string();
    }

    subject.repeat(times)
}

/// Repeats the `subject` number of `times`.
///
/// # Arguments
///
/// * `subject` - The string to repeat.
/// * `times` - The number of times to repeat.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::reverse("winter");
/// // => "retniw"
/// ```
pub fn reverse(subject: &str) -> String {
    if subject.len() == 0 {
        return "".to_string();
    }

    subject.chars().rev().collect()
}

use unicode_segmentation::UnicodeSegmentation;
/// Repeats the `subject` number of `times`.
///
/// # Arguments
///
/// * `subject` - The string to repeat.
/// * `times` - The number of times to repeat.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::reverse_grapheme("café");
/// // => "éfac"
/// manipulate::reverse_grapheme("a̐éö̲");
/// // => "ö̲éa̐"
/// ```
pub fn reverse_grapheme(subject: &str) -> String {
    if subject.len() == 0 {
        return "".to_string();
    }

    UnicodeSegmentation::graphemes(subject, true)
        .rev()
        .collect::<Vec<&str>>()
        .join("")
}

/// Removes whitespaces from left and right sides of the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to trim.
/// * `whitespace` - The whitespace characters to trim. List all characters that you want to be stripped.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::trim(" Mother nature ", "");
/// // => "Mother nature"
/// manipulate::trim("-~-Earth~-~", "-~");
/// // => "Earth"
/// ```
pub fn trim(subject: &str, whitespace: &str) -> String {
    trim_left_or_right(&subject, &whitespace, true, true)
}

/// Removes whitespaces from the left side of the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to trim.
/// * `whitespace` - The whitespace characters to trim. List all characters that you want to be stripped.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::trim_left(" Mother nature ", "");
/// // => "Mother nature "
/// manipulate::trim_left("-~-Earth~-~", "-~");
/// // => "Earth~-~"
/// ```
pub fn trim_left(subject: &str, whitespace: &str) -> String {
    trim_left_or_right(&subject, &whitespace, true, false)
}

/// Removes whitespaces from the right side of the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to trim.
/// * `whitespace` - The whitespace characters to trim. List all characters that you want to be stripped.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::trim_right(" Mother nature ", "");
/// // => " Mother nature"
/// manipulate::trim_right("-~-Earth~-~", "-~");
/// // => "-~-Earth"
/// ```
// TODO: trim_left is deprecating in 1.33.0: superseded by trim_start
// TODO: trim_right is deprecating in 1.33.0: superseded by trim_end
// TODO: trim_left_matches is deprecating in 1.33.0: superseded by trim_start_matches
// TODO: trim_right_matches is deprecating in 1.33.0: superseded by trim_end_matches

pub fn trim_right(subject: &str, whitespace: &str) -> String {
    trim_left_or_right(&subject, &whitespace, false, true)
}

fn trim_left_or_right(subject: &str, whitespace: &str, to_left: bool, to_right: bool) -> String {
    if subject.len() == 0 {
        return subject.to_string();
    }
    if whitespace.len() == 0 {
        if to_left && to_right {
            return subject.trim().to_string();
        } else if to_left {
            return subject.trim_left().to_string();
        } else {
            return subject.trim_right().to_string();
        }
    }

    if to_left && to_right {
        return subject.trim_matches(|c| whitespace.contains(c)).to_owned();
    } else if to_left {
        return subject
            .trim_left_matches(|c| whitespace.contains(c))
            .to_owned();
    } else {
        return subject
            .trim_right_matches(|c| whitespace.contains(c))
            .to_owned();
    }
}
