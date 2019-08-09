//! Strips specific characters from subject.

use dissolve;
/// Strips the byte order mark (BOM) from the beginning of `subject`.
///
/// # Arguments
///
/// * `subject` - The string to strip from.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// strip::strip_bom("\u{FEFF}summertime sadness");
/// // => "summertime sadness"
/// strip::strip_bom("summertime sadness");
/// // => "summertime sadness"
/// ```
pub fn strip_bom(subject: &str) -> String {
    match subject.len() {
        0 => "".to_string(),
        _ => {
            if crate::chop::first(&subject, 1) == "\u{FEFF}" {
                crate::chop::slice(&subject, 1, 0)
            } else {
                subject.to_string()
            }
        }
    }
}

/// Strips all HTML tags from `subject`.
///
/// # Arguments
///
/// * `subject` - The string to strip from.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// strip::strip_tags("<span><a href=\"#\">Summer</a> is nice</span>");
/// // => "Summer is nice"
/// ```
pub fn strip_tags(subject: &str) -> String {
    match subject.len() {
        0 => "".to_string(),
        // TODO: Rewrite this function to remove "dissolve" module
        // use the only clean function #14
        _ => dissolve::strip_html_tags(&subject)
            .into_iter()
            .collect::<Vec<String>>()
            .join(""),
    }
}
