//! Escapes  special characters in `subject`.

use split;
#[derive(Clone, Copy, PartialEq)]
enum CovertMode {
    Html,
    Special,
}
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
        _ => map_replace(&subject, CovertMode::Html),
    }
}

fn map_replace(subject: &str, map_type: CovertMode) -> String {
    let html_symbols = vec!["<", ">", "&", "'", "\"", "`"];
    let special_symbols = vec!["&lt;", "&gt;", "&amp;", "&#x27;", "&quot;", "&#x60;"];
    let mut res = String::new();
    let key;
    let value;
    match map_type {
        CovertMode::Html => {
            key = html_symbols;
            value = special_symbols
        }
        CovertMode::Special => {
            key = special_symbols;
            value = html_symbols
        }
    }
    match key.len() {
        0 => subject.to_string(),
        _ => {
            for c in split::chars(&subject) {
                match key.iter().position(|&x| x == c) {
                    Some(i) => res.push_str(value[i]),
                    _ => res.push_str(c),
                }
            }
            res
        }
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
