//! Escapes  special characters in `subject`.

use split;
/// Escapes HTML special characters < > & ' " ` in `subject`.
///
/// # Arguments
///
/// * `subject` - The string to escape.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// escape::escape_html("<p>wonderful world</p>");
/// // => &lt;p&gt;wonderful world&lt;/p&gt;
pub fn escape_html(subject: &str) -> String {
    match subject.len() {
        0 => "".to_string(),
        _ => {
            let mut res = String::new();
            for c in split::chars(&subject) {
                match c {
                    "<" => res.push_str("&lt;"),
                    ">" => res.push_str("&gt;"),
                    "&" => res.push_str("&amp;"),
                    "\"" => res.push_str("&quot;"),
                    "'" => res.push_str("&#x27;"),
                    "`" => res.push_str("&#x60;"),
                    _ => res.push_str(&c),
                }
            }
            res
        }
    }
}
