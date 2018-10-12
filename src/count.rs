//! Counts the characters in `subject`.

use split;
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
pub fn count(subject: &str) -> usize {
    match subject.len() {
        0 => 0,
        _ => split::chars(subject).len(),
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
pub fn count_graphemes(subject: &str) -> usize {
    match subject.len() {
        0 => 0,
        _ => split::graphemes(subject).len(),
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
pub fn count_substrings(subject: &str, substring: &str) -> usize {
    fn match_substring(subject: &str, substring: &str) -> usize {
        match substring.len() {
            0 => 0,
            _ => subject.matches(substring).count(),
        }
    }

    match subject.len() {
        0 => 0,
        _ => match_substring(&subject, &substring),
    }
}
