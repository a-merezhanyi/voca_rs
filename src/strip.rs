//! Strips specific characters from subject.

use split::graphemes;

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
/// use voca_rs::Voca;
/// "\u{FEFF}summertime sadness"._strip_bom();
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
/// use voca_rs::Voca;
/// "<span><a href=\"#\">Summer</a> is nice</span>"._strip_tags();
/// // => "Summer is nice"
/// ```
pub fn strip_tags(subject: &str) -> String {
    match subject.len() {
        0 => "".to_string(),
        _ => strip_html_tags(&subject),
    }
}

#[derive(Clone, Copy, PartialEq)]
enum StateMode {
    Output,
    Html,
    Exclamation,
    Comment,
}

fn unicode_string_range(subject: &str, start: usize, end: usize) -> String {
    graphemes(subject)[start..end]
        .iter()
        .map(|c| (*c).to_string())
        .collect::<String>()
}

fn strip_html_tags(subject: &str) -> String {
    // https://github.com/panzerdp/voca/blob/master/src/strip/strip_tags.js
    let length = subject.len();
    let mut state: StateMode = StateMode::Output;
    let mut depth = 0;
    let mut output = String::with_capacity(length);
    let mut quote = String::with_capacity(4);
    for (i, c) in crate::split::chars(subject).iter().enumerate() {
        let mut advance = false;
        match c.to_owned() {
            "<" => {
                if !quote.is_empty() {
                } else if crate::query::query(
                    unicode_string_range(subject, i, i + 2).as_str(),
                    "< ",
                    0,
                ) {
                    advance = true;
                } else if state == StateMode::Output {
                    advance = true;
                    state = StateMode::Html;
                } else if state == StateMode::Html {
                    depth += 1;
                } else {
                    advance = true;
                }
            }
            "!" => {
                if state == StateMode::Html
                    && crate::query::query(
                        unicode_string_range(subject, i, i + 2).as_str(),
                        "<!",
                        0,
                    )
                {
                    state = StateMode::Exclamation;
                } else {
                    advance = true;
                }
            }
            "-" => {
                if state == StateMode::Exclamation
                    && crate::query::query(
                        unicode_string_range(subject, i, i + 3).as_str(),
                        "!--",
                        0,
                    )
                {
                    state = StateMode::Comment;
                } else {
                    advance = true;
                }
            }
            "\"" | "'" => {
                if state == StateMode::Html {
                    let c_copy = c.to_string();
                    if quote.as_str() == c_copy {
                        quote = String::from("");
                    } else if quote.is_empty() {
                        quote = c_copy;
                    }
                } else {
                    advance = true;
                }
            }
            "E" | "e" => {
                if state == StateMode::Exclamation
                    && crate::query::query(
                        unicode_string_range(subject, i, i + 7).as_str(),
                        "doctype",
                        0,
                    )
                {
                    state = StateMode::Html;
                } else {
                    advance = true;
                }
            }
            ">" => {
                if depth > 0 {
                    depth -= 1;
                } else if !quote.is_empty() {
                } else if state == StateMode::Html
                    || state == StateMode::Exclamation
                    || state == StateMode::Comment
                        && crate::query::query(
                            unicode_string_range(subject, i, i + 3).as_str(),
                            "-->",
                            0,
                        )
                {
                    quote = String::from("");
                    state = StateMode::Output;
                } else {
                    advance = true;
                }
            }
            _ => {
                advance = true;
            }
        }
        if advance {
            match state {
                StateMode::Output => {
                    output.push_str(&c);
                }
                StateMode::Html => {}
                _ => {}
            }
        }
    }
    output.to_owned()
}
