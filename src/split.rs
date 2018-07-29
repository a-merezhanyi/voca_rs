//! Splits `subject` into an chuncks according to given rules.

/// Splits `subject` into an array of words.
pub fn words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect::<Vec<_>>()
}
