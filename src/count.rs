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
    split::chars(subject).len()
}
