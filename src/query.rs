//! Checks a `subject` against a query.

use chop;
use count;
use regex::Regex;
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
    if subject.is_empty() || end.is_empty() {
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
    let subject_len = count::count(&subject);
    if subject_len < position {
        return false;
    }
    if subject_len == 0 || search.is_empty() {
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
    if is_empty(&subject) {
        false
    } else {
        is_alpha_or_alphadigit(&subject, false)
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
            current_pos += 1;
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
    if is_empty(&subject) {
        false
    } else {
        is_alpha_or_alphadigit(&subject, true)
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
    if subject.is_empty() {
        return true;
    }

    subject.trim().is_empty()
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
        })
        .count()
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
    if subject.is_empty() {
        return true;
    }

    false
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
/// query::is_numeric("0xFF");
/// // => true
/// query::is_numeric("1.5E+2");
/// // => true
/// query::is_numeric("five");
/// // => false
/// ```
pub fn is_numeric(subject: &str) -> bool {
    if subject.is_empty() {
        return true;
    }

    fn parse_str_num(n: &str) -> bool {
        match n.find('.') {
            Some(_) => n.parse::<f32>().is_ok(),
            None => n.parse::<i32>().is_ok(),
        }
    }

    let sbj = subject.to_lowercase();
    match subject.to_lowercase().find('e') {
        Some(_) => {
            let s: Vec<&str> = sbj.split('e').collect();
            parse_str_num(s[0]) && parse_str_num(s[1])
        }
        None => {
            if starts_with(&subject.to_lowercase(), "0x") {
                let s = sbj.trim_start_matches("0x");
                i32::from_str_radix(s, 16).is_ok()
            } else {
                parse_str_num(subject)
            }
        }
    }
}

/// Checks whether `subject` is a titlecased string and there is at least one character.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_title("This Is String Example...Wow!!!");
/// // => true
/// query::is_title("This is string example....wow!!!");
/// // => false
/// ```
pub fn is_title(subject: &str) -> bool {
    if subject.is_empty() {
        return false;
    }
    let words = split::words(&subject);
    let subject_len = words.len();
    words
        .iter()
        .filter(|w| {
            let mut res = String::with_capacity(w.len());
            for (i, c) in split::chars(w).iter().enumerate() {
                if (i == 0 && c == &c.to_uppercase()) || (i > 0 && c == &c.to_lowercase()) {
                    res.push_str(&c)
                }
            }
            res.len() == w.len()
        })
        .count()
        == subject_len
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
    if subject.is_empty() {
        return true;
    }

    let mut res = true;
    split::chars(subject).into_iter().for_each(|s| {
        s.chars().for_each(|c| {
            if (lowercase && c.is_uppercase()) || (!lowercase && c.is_lowercase()) {
                res = false;
            }
        })
    });
    res
}

/// Checks whether `subject` matches the regular expression `pattern`.
/// NOTE: Executes regular expressions only on valid UTF-8 while exposing match locations as byte indices into the search string (see case #4).
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `pattern` - The RegExp pattern to match, it is transformed to Regex::new(pattern).
/// * `position` - The position to start matching.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::matches("pluto", "a", 0);
/// // => false
/// query::matches("pluto", r"plu.{2}", 0);
/// // => true
/// query::matches("apollo 11", r"\d{3}", 0);
/// // => false
/// query::matches("Zażółć gęślą jaźń", "gęślą", 11);
/// // => true (because "gęślą" starts from 11 not 7)
/// ```
pub fn matches(subject: &str, pattern: &str, position: usize) -> bool {
    let subject_len = split::chars(&subject).len();
    if subject_len == 0 {
        return false;
    }
    if position >= subject_len {
        return false;
    }
    match pattern.len() {
        0 => true,
        _ => {
            println!("{} {} {}", subject, pattern, position);
            let re: Regex = Regex::new(pattern).unwrap();
            re.is_match_at(&subject, position)
        }
    }
}

/// Checks whether `subject` contains all characters from `search` starting from `position`. Respects an order of characters.
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
/// query::query("starship", "star", 0);
/// // => true
/// query::query("the world is yours", "te wld", 0);
/// // => true
/// query::query("galaxy", "g", 1);
/// // => false
/// ```
pub fn query(subject: &str, search: &str, position: usize) -> bool {
    let subject_len = count::count(&subject);
    if subject_len < position {
        return false;
    }
    if subject_len == 0 || search.is_empty() {
        return true;
    }
    let mut i: usize = 0;
    let q = split::chars(&search);
    let q_len = split::chars(&search).len();
    split::chars(&subject.to_owned()[subject.char_indices().nth(position).unwrap().0..])
        .into_iter()
        .filter(|c| {
            if i < q_len {
                if c == &q[i] {
                    i += 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .count()
        == count::count(&search)
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
/// // => false
/// ```
pub fn starts_with(subject: &str, start: &str) -> bool {
    if subject.is_empty() || start.is_empty() {
        return true;
    }
    subject.starts_with(start)
}
