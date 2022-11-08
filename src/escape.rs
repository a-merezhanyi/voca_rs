//! Escapes  special characters in `subject`.

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
/// use voca_rs::Voca;
/// "<p>wonderful world</p>"._escape_html();
/// // => &lt;p&gt;wonderful world&lt;/p&gt;
/// ```
// TODO: check for optimizations #10
// https://lise-henry.github.io/articles/optimising_strings.html
pub fn escape_html(subject: &str) -> String {
    match subject.len() {
        0 => "".to_string(),
        _ => subject
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .replace('`', "&#x60;"),
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
/// use voca_rs::Voca;
/// "(hours)[minutes]{seconds}"._escape_regexp();
/// // => \(hours\)\[minutes\]\{seconds\}
/// ```
pub fn escape_regexp(subject: &str) -> String {
    let key = "-[]/{}()*+?.\\^$|";
    match subject.len() {
        0 => "".to_string(),
        _ => {
            let mut res = String::new();
            for c in crate::split::chars(subject) {
                let push_char = if key.contains(c) {
                    format!("\\{}", c)
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
/// use voca_rs::Voca;
/// "&lt;p&gt;wonderful world&lt;/p&gt;"._unescape_html();
/// // => <p>wonderful world</p>
/// ```
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
