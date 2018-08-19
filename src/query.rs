//! Checks a `subject` against a query.

/// Checks whether `subject` ends with `end`.
///
/// # Arguments
///
/// * `string: &str` - The string to verify.
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
pub fn ends_with(string: &str, end: &str) -> bool {
    if string.len() == 0 || end.len() == 0 {
        return true;
    }
    string.ends_with(end)
}

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
/// query::starts_with("say hello to my little friend", "hello");
/// // => flase
/// ```
pub fn starts_with(string: &str, start: &str) -> bool {
    if string.len() == 0 || start.len() == 0 {
        return true;
    }
    string.starts_with(start)
}
