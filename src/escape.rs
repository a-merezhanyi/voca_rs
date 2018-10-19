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
        _ => subject
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#x27;")
            .replace("`", "&#x60;"),
    }
}

/// Escapes the regular expression special characters - [ ] / { } ( ) * + ? . \ ^ $ | in `subject`.
///
/// # Arguments
///
/// * `subject` - The string to escape.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// escape::escape_regexp("(hours)[minutes]{seconds}");
/// // => \(hours\)\[minutes\]\{seconds\}
pub fn escape_regexp(subject: &str) -> String {
    let key = "-[]/{}()*+?.\\^$|";
    match subject.len() {
        0 => "".to_string(),
        _ => {
            let mut res = String::new();
            for c in split::chars(&subject) {
                let push_char = if key.contains(&c) {
                    format!("\\{}", &c)
                } else {
                    c.to_string()
                };
                res.push_str(&push_char);
            }
            res
        }
    }
}

/// Unescapes HTML special characters from &lt; &gt; &amp; &quot; &#x27; &#x60; to corresponding < > & " ' ` in `subject`.
///
/// # Arguments
///
/// * `subject` - The string to unescape.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// escape::unescape_html("&lt;p&gt;wonderful world&lt;/p&gt;");
/// // => <p>wonderful world</p>
pub fn unescape_html(subject: &str) -> String {
    match subject.len() {
        0 => "".to_string(),
        _ => subject
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
            .replace("&quot;", "\"")
            .replace("&#x27;", "'")
            .replace("&#x60;", "`"),
    }
}
