//! Converts the `subject` to a selected case.

/// Converts the first character of `subject` to upper case. If `restToLower` is `true`, convert the rest of `subject` to lower case.
///
/// # Arguments
///
/// * `subject: &str` - The string to capitalize.
/// * `rest_to_lower: &bool` - Convert the rest of `subject` to lower case.
/// # Example
///
/// ```
/// use voca_rs::*;
/// case::capitalize("green", &true);
/// // => "Green"
/// case::capitalize("Say Hello to ME", &true);
/// // => "Say hello to me"
/// ```
pub fn capitalize(subject: &str, rest_to_lower: &bool) -> String {
    if subject.len() == 0 {
        return subject.to_owned();
    }

    let mut res = String::with_capacity(subject.len());
    for (i, c) in split::chars(subject).iter().enumerate() {
        let s;
        if i == 0 {
            s = c.to_uppercase();
        } else {
            if *rest_to_lower {
                s = c.to_lowercase();
            } else {
                s = c.to_string();
            }
        }
        res.push_str(&s);
    }

    res
}

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
