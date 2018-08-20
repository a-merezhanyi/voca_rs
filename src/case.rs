//! Converts the `subject` to a selected case.

use split;
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

    let mut res = String::with_capacity(string.len());
    for c in split::chars(string).iter() {
        res.push_str(&c.to_lowercase());
    }

    res
}

/// Converts the `subject` to upper case.
///
/// # Arguments
///
/// * `string: &str` - The string to convert to upper case.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// case::upper_case("Green");
/// // => "GREEN"
/// case::upper_case("Say Hello to ME");
/// // => "SAY HELLO TO ME"
/// ```
pub fn upper_case(string: &str) -> String {
    if string.len() == 0 {
        return string.to_owned();
    }

    let mut res = String::with_capacity(string.len());
    for c in split::chars(string).iter() {
        res.push_str(&c.to_uppercase());
    }

    res
}
