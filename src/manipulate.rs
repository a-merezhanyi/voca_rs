//! Manipulate with the `subject`.

use chop;
use count;
use index;
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

/// Replaces the matches of `pattern` with `replacement`.
///
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `pattern` - The pattern which match is replaced. Only the first occurrence replaced.
/// * `replacement` - The string which invocation result replaces `pattern` match.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::replace("swan", "wa", "u");
/// // => "sun"
/// manipulate::replace("domestic duck", "d", "D");
/// // => "Domestic duck"
/// manipulate::replace("Café del Mar cafe\u{0301}", "é", "e");
/// // => "Cafe del Mar café"
/// ```
pub fn replace(subject: &str, pattern: &str, replacement: &str) -> String {
    if subject.len() == 0 || pattern.len() == 0 {
        return subject.to_string();
    }
    match index::index_of(&subject, &pattern, 0) {
        -1 => subject.to_string(),
        x => splice(&subject, x as isize, count::count(&pattern), &replacement),
    }
}

/// Reverses the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to reverse.
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
/// Reverses the `subject` taking care of surrogate pairs and combining marks.
///
/// # Arguments
///
/// * `subject` - The string to reverse.
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

/// Changes `subject` by deleting `delete_count` of characters starting at position `start`. Places a new string `to_add` instead of deleted characters.
///
/// # Arguments
///
/// * `subject` - The string where to insert.
/// * `start` - The position to start changing the string. For a negative position will start from the end of the string.
/// * `delete_count` - The number of characters to delete from string.
/// * `to_add` - The string to be added instead of deleted characters.
///
/// # Example
/// ```
/// use voca_rs::*;
/// manipulate::splice("new year", 0, 4, "");
/// // => "year"
/// manipulate::splice("to jest błąd", 0, 7, "mój");
/// // => "mój błąd"
/// manipulate::splice("Die Schildkröte fliegt.", -7, 0, "und Kröte ");
/// // => "Die Schildkröte und Kröte fliegt."
/// manipulate::splice("Привет", 6, 0, ", Ёлка!");
/// // => "Привет, Ёлка!"
/// ```
// TODO: check boundaries
pub fn splice(subject: &str, start: isize, delete_count: usize, to_add: &str) -> String {
    let subject_len = count::count(&subject);
    fn calculate_start_position(start: isize, subject_len: usize) -> usize {
        if start < 0 {
            if start.abs() as usize > subject_len {
                0
            } else {
                subject_len - start.abs() as usize
            }
        } else {
            if (start as usize) >= subject_len {
                subject_len
            } else {
                start as usize
            }
        }
    }

    match delete_count {
        0 => match to_add.len() {
            0 => subject.to_string(),
            _ => {
                let insert_position = calculate_start_position(start, subject_len);
                if insert_position >= subject_len {
                    format!("{}{}", &subject, &to_add)
                } else {
                    insert(&subject, &to_add, insert_position)
                }
            }
        },
        _ => {
            let start_position = calculate_start_position(start, subject_len);
            let end_position = if delete_count > subject_len - start_position {
                subject_len
            } else {
                start_position + delete_count
            };

            format!(
                "{}{}{}",
                chop::first(&subject, start_position),
                &to_add,
                chop::slice(&subject, end_position as isize, 0)
            )
        }
    }
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
