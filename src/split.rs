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
/// use voca_rs::*;
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
/// use voca_rs::*;
/// split::split("rage against the dying of the light", "");
/// // => ["rage", "against", "the", "dying", "of", "the", "light"]
/// ```
pub fn split(string: &'static str, pattern: &str) -> Vec<&'static str> {
    if string.len() == 0 {
        return vec![];
    }
    if pattern.len() == 0 {
        return vec![string];
    }
    string.split_terminator(pattern).collect::<Vec<_>>()
}

/// Splits `subject` into an array of words.
///
/// # Arguments
///
/// * `string: &str` - The string to split into characters.
///
/// # Example
///
/// ```rust
/// use voca_rs::*;
/// split::words("Sześć звёзд are dying");
/// // => ["Sześć", "звёзд", "are", "dying"]
/// ```
pub fn words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect::<Vec<_>>()
}
