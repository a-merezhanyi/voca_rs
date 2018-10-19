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
        _ => map_replace(&subject, "html"),
    }
}

fn map_replace(subject: &str, map_type: &str) -> String {
    let html_symbols = vec!["<", ">", "&", "'", "\"", "`"];
    let special_symbols = vec!["&lt;", "&gt;", "&amp;", "&#x27;", "&quot;", "&#x60;"];
    let mut res = String::new();
    let key;
    let mut value = vec![];
    match map_type {
        "html" => {
            key = html_symbols;
            value = special_symbols
        }
        "un_html" => {
            key = special_symbols;
            value = html_symbols
        }
        _ => key = vec![],
    }
    match key.len() {
        0 => subject.to_string(),
        _ => {
            for c in split::chars(&subject) {
                match key.iter().position(|&x| x == c) {
                    Some(i) => {
                        println!("c={} i={}, {}", c, i, value[i]);
                        res.push_str(value[i])
                    }
                    _ => res.push_str(c),
                }
            }
            res
        }
    }
}
