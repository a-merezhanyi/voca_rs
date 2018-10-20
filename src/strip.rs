//! Strips specific characters from subject.

use chop;
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
            if chop::first(&subject, 1) == "\u{FEFF}" {
                chop::slice(&subject, 1, 0)
            } else {
                subject.to_string()
            }
        }
    }
}
