//! Counts the characters in `subject`.

/// Counts the characters in `subject`.
///
/// # Arguments
///
/// * `subject` - The string to count characters.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// count::count("rain");
/// // => 4
/// count::count("błąd");
/// // => 4
/// use voca_rs::Voca;
/// "rain"._count();
/// // => 4
/// ```
pub fn count(subject: &str) -> usize {
    match subject.len() {
        0 => 0,
        _ => crate::split::chars(subject).len(),
    }
}

/// Counts the graphemes in `subject` taking care of surrogate pairs and combining marks.
///
/// # Arguments
///
/// * `subject` - The string to count graphemes.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// count::count_graphemes("cafe\u{0301}"); // or "café"
/// // => 4
/// count::count_graphemes("b\u{0142}\u{0105}d"); // or "błąd"
/// // => 4
/// count::count_graphemes("a̐éö̲");
/// // => 3
/// count::count_graphemes("rain");
/// // => 4
/// use voca_rs::Voca;
/// "cafe\u{0301}"._count_graphemes(); // or "café"
/// // => 4
/// ```
pub fn count_graphemes(subject: &str) -> usize {
    match subject.len() {
        0 => 0,
        _ => crate::split::graphemes(subject).len(),
    }
}

/// Counts the number of `substring` appearances in `subject`.
///
/// # Arguments
///
/// * `subject` - The string where to count.
/// * `substring` - The substring to be counted.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// count::count_substrings("bad boys, bad boys whatcha gonna do?", "boys");
/// // => 2
/// count::count_substrings("Cafe\u{0301} del Mar", "Café"); // or "Café del Mar"
/// // => 1
/// count::count_substrings("every dog has its day", "cat");
/// // => 0
/// use voca_rs::Voca;
/// "bad boys, bad boys whatcha gonna do?"._count_substrings("boys");
/// // => 2
/// ```
pub fn count_substrings(subject: &str, substring: &str) -> usize {
    fn match_substring(subject: &str, substring: &str) -> usize {
        match substring.len() {
            0 => 0,
            _ => subject.matches(substring).count(),
        }
    }

    match subject.len() {
        0 => 0,
        _ => match_substring(subject, substring),
    }
}

/// Counts the characters in `subject` for which `predicate` returns true. This function respects unicode.
///
/// # Arguments
///
/// * `subject` - The string to count characters.
/// * `predicate` - The predicate function invoked on each character with a parameter `(string)`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// count::count_where("hola!", voca_rs::query::is_alpha);
/// // => 4
/// count::count_where("2022", |s: &str| -> bool { s == "2" });
/// // => 3
/// use voca_rs::Voca;
/// "hola!"._count_where(voca_rs::query::is_alpha);
/// // => 4
/// ```
pub fn count_where(subject: &str, f: fn(&str) -> bool) -> usize {
    match subject.len() {
        0 => 0,
        _ => {
            let mut res = 0;
            for c in crate::split::graphemes(subject).iter() {
                if f(c) {
                    res += 1;
                }
            }
            res
        }
    }
}

/// Counts the number of words in `subject`.
///
/// # Arguments
///
/// * `subject` - The string where to count.
/// * `pattern` - The pattern to watch words.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// count::count_words("Gravity - can cross dimensions!", "");
/// // => 4
/// count::count_words("GravityCanCrossDimensions", "");
/// // => 4
/// count::count_words("Cafe\u{0301}-del-Mar-andBossaNova1", "-");
/// // => 4
/// use voca_rs::Voca;
/// "Gravity - can cross dimensions!"._count_words("");
/// // => 4
/// ```
pub fn count_words(subject: &str, pattern: &str) -> usize {
    fn match_substring(subject: &str, pattern: &str) -> usize {
        match pattern.len() {
            0 => crate::split::words(subject).iter().count(),
            _ => subject
                .split_terminator(pattern)
                .collect::<Vec<_>>()
                .iter()
                .count(),
        }
    }
    match subject.len() {
        0 => 0,
        _ => match_substring(subject, pattern),
    }
}

use std::collections::HashMap;
/// Counting occurrences of unique words in `subject`. This function respects unicode.
///
/// # Arguments
///
/// * `subject` - The string to count characters.
/// * `pattern` - The pattern to watch words.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// count::count_unique_words("hello world wonderful world", "");
/// // => 3
/// count::count_unique_words("Arabic: أنا قادر على أكل الزجاج و هذا لا يؤلمني. أنا قادر على أكل الزجاج و.", "");
/// // => ???????????????????????????????????
/// use voca_rs::Voca;
/// "Hebrew: אני יכול לאכול זכוכית וזה לא מזיק לי. אני יכול לאכול זכוכית."._count_words("");
/// // => 9
/// ```
pub fn count_unique_words(subject: &str, pattern: &str) -> usize {
    let mut unique_words = HashMap::new();
    let words = match pattern.len() {
        0 => crate::split::words(subject),
        _ => subject.split_terminator(pattern).collect::<Vec<_>>(),
    };
    if words.len() == 0 {
        return 0;
    };

    for word in words {
        unique_words.entry(word).or_insert(0);
    }
    unique_words.len()
}
