//! Extracts a character(s) from `subject`.

use split;
/// Access a character from `subject` at specified `position`.
///
/// # Arguments
///
/// * `subject` - The string to extract from.
/// * `position` - The position to get the character.
///
/// # Example
/// ```
/// use voca_rs::*;
/// chop::char_at("helicopter", 0);
/// // => "h"
/// chop::char_at("błąd", 1);
/// // => "ł"
/// ```
pub fn char_at(subject: &str, position: usize) -> String {
    if subject.len() == 0 {
        return subject.to_string();
    }

    split::chars(&subject)[position].to_string()
}
