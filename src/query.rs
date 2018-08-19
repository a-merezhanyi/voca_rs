//! Checks a `subject` against a query.

/// Checks whether `subject` starts with `start`.
///
/// # Arguments
///
/// * `string: &str` - The string to verify.
/// * `start: &str` - The starting string.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::starts_with("say hello to my little friend", "say hello");
/// // => true
/// ```
pub fn starts_with(string: &str, start: &str) -> bool {
    if string.len() == 0 || start.len() == 0 {
        return true;
    }
    string.starts_with(start)
}
