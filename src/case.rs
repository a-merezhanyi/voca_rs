//! Converts the `subject` to a selected case.

/// Converts the `subject` to lower case.
///
/// # Arguments
///
/// * `string: &str` - The string to convert to lower case.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// case::lower_case("Green");
/// // => "green"
/// case::lower_case("Say Hello to ME");
/// // => "say hello to me"
/// ```
pub fn lower_case(string: &str) -> String {
    if string.len() == 0 {
        return string.to_owned();
    }
    string
        .chars()
        .filter_map(|c| c.to_lowercase().next())
        .collect()
}
