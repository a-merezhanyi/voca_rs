//! Manipulate with the `subject`.

/// Removes whitespaces from left and right sides of the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to trim.
/// * `whitespace` - The whitespace characters to trim. List all characters that you want to be stripped.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::trim(" Mother nature ", "");
/// // => "Mother nature"
/// manipulate::trim("-~-Earth~-~", "-~");
/// // => "Earth"
/// ```
pub fn trim(subject: &str, whitespace: &str) -> String {
    if subject.len() == 0 {
        return subject.to_string();
    }
    if whitespace.len() == 0 {
        return subject.trim().to_string();
    }

    subject.trim_matches(|c| whitespace.contains(c)).to_owned()
}
