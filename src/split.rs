//! Splits `subject` into an chuncks according to given rules.

/// Splits `subject` into an array of characters.
///
/// # Arguments
///
/// * `string: &str` - The string to split into characters.
///
/// # Example
///
/// ```
/// split::chars("cloud");
/// // => ["c", "l", "o", "u", "d"]
/// ```
pub fn chars(string: &str) -> Vec<&str> {
    if string.len() == 0 {
        return vec![];
    }
    string.split_terminator("").skip(1).collect::<Vec<_>>()
}

/// Splits `subject` into an array of chunks by `separator`.
///
/// # Arguments
///
/// * `string: &str` - The string to split into characters.
/// * `pattern: &str` - The pattern to match the separator.
///
/// # Example
///
/// ```rust
/// split::split("rage against the dying of the light", " "");
/// // => ["rage", "against", "the", "dying", "of", "the", "light"]
/// ```
pub fn split(s: &'static str, d: &str) -> Vec<&'static str> {
    if s.len() == 0 {
        return vec![];
    }
    if d.len() == 0 {
        return vec![s];
    }
    s.split_terminator(d).collect::<Vec<_>>()
}

/// Splits `subject` into an array of words.
pub fn words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect::<Vec<_>>()
}
