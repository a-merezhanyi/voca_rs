//! Splits `subject` into an chuncks according to given rules.

/// Splits `subject` into an array of characters.
pub fn chars(s: &str) -> Vec<&str> {
    if s.len() == 0 {
        return vec![];
    }
    s.split_terminator("").skip(1).collect::<Vec<_>>()
}

/// Splits `subject` into an array of words.
pub fn words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect::<Vec<_>>()
}
