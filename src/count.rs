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
pub fn count(subject: &str) -> usize {
    if subject.len() == 0 {
        return 0;
    }
    split::chars(subject).len()
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
/// count::count_graphemes("cafe\u{0301}"); // or 'café'
/// // => 4
/// count::count_graphemes("a̐éö̲");
/// // => 3
/// count::count_graphemes("rain");
/// // => 4
pub fn count_graphemes(subject: &str) -> usize {
    if subject.len() == 0 {
        return 0;
    }
    split::graphemes(subject).len()
}
