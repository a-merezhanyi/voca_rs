//! Checks a `subject` against a query.

use regex::Regex;
/// Checks whether `subject` ends with `end`.
///
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `end` - The ending string.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::ends_with("say hello to my little friend", "little friend");
/// // => true
/// query::ends_with("say hello to my little friend", "little");
/// // => false
/// use voca_rs::Voca;
/// "say hello to my little friend".ends_with("little friend");
/// // => true
/// ```
pub fn ends_with(subject: &str, end: &str) -> bool {
    if subject.is_empty() || end.is_empty() {
        return true;
    }
    subject.ends_with(end)
}

/// Checks whether `subject` includes `search` starting from `position`.
///
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `search` - The ending string.
/// * `position` - The position to start searching.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::includes("starship", "star", 0);
/// // => true
/// query::includes("Zażółć gęślą jaźń", "gęślą", 7);
/// // => true
/// query::includes("galaxy", "g", 1);
/// // => false
/// use voca_rs::Voca;
/// "starship".includes("star", 0);
/// // => true
/// ```
pub fn includes(subject: &str, search: &str, position: usize) -> bool {
    let subject_len = crate::count::count(&subject);
    if subject_len < position {
        return false;
    }
    if subject_len == 0 || search.is_empty() {
        return true;
    }
    subject.to_owned()[subject.char_indices().nth(position).unwrap().0..]
        .to_string()
        .contains(&search)
}

/// Checks whether `subject` contains only alpha characters.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_alpha("");
/// // => false
/// query::is_alpha("cafe\u{0301}"); // or "café"
/// // => true
/// query::is_alpha("bart");
/// // => true
/// query::is_alpha("lisa!");
/// // => false
/// query::is_alpha("Zażółć and bart");
/// // => false
/// use voca_rs::Voca;
/// "bart".is_alpha();
/// // => true
/// ```
pub fn is_alpha(subject: &str) -> bool {
    if is_empty(&subject) {
        false
    } else {
        is_alpha_or_alphadigit(&subject, false)
    }
}

fn is_alpha_or_alphadigit(subject: &str, count_digits: bool) -> bool {
    let mut subject_is_ok = true;
    let subject_grapheme_len = crate::count::count_graphemes(&subject);
    let mut current_pos = 0;
    while current_pos < subject_grapheme_len {
        let current_char = crate::chop::grapheme_at(&subject, current_pos);
        if (!count_digits
            && (is_digit(&current_char)
                || is_blank(&current_char)
                || crate::utils::PUNCTUATION.contains(&current_char)))
            || (count_digits
                && (is_blank(&current_char) || crate::utils::PUNCTUATION.contains(&current_char)))
        {
            subject_is_ok = false;
            current_pos = subject_grapheme_len;
        } else {
            current_pos += 1;
        }
    }
    subject_is_ok
}

/// Checks whether `subject` contains contains only alpha and digit characters.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_alphadigit("");
/// // => false
/// query::is_alphadigit("cafe\u{0301}"); // or "café"
/// // => true
/// query::is_alphadigit("year2020");
/// // => true
/// query::is_alphadigit("1448");
/// // => true
/// query::is_alphadigit("40-20");
/// // => false
/// use voca_rs::Voca;
/// "year2020".is_alphadigit();
/// // => true
/// ```
pub fn is_alphadigit(subject: &str) -> bool {
    if is_empty(&subject) {
        false
    } else {
        is_alpha_or_alphadigit(&subject, true)
    }
}

/// Checks whether `subject` is empty or contains only whitespaces.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_blank("");
/// // => true
/// query::is_blank("   ");
/// // => true
/// query::is_blank("sun");
/// // => false
/// use voca_rs::Voca;
/// "   ".is_blank();
/// // => true
/// ```
pub fn is_blank(subject: &str) -> bool {
    if subject.is_empty() {
        return true;
    }

    subject.trim().is_empty()
}

/// Checks whether `subject` is camelCased.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_camel_case("");
/// // => true
/// query::is_camel_case("birdFlight");
/// // => true
/// query::is_camel_case("bird flight");
/// // => false
/// query::is_camel_case("-BIRD-FLIGHT-");
/// // => false
/// use voca_rs::Voca;
/// "birdFlight".is_camel_case();
/// // => true
/// ```
pub fn is_camel_case(subject: &str) -> bool {
    subject == crate::case::camel_case(&subject)
}

/// Checks whether `subject` is capitalized and the rest of `subject` is converted to lower case.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_capitalize("");
/// // => true
/// query::is_capitalize("John has a motorcycle");
/// // => true
/// query::is_capitalize("the world is yours");
/// // => false
/// query::is_capitalize("Żółć niedźwiedzia");
/// // => true
/// use voca_rs::Voca;
/// "John has a motorcycle".is_capitalize();
/// // => true
/// ```
pub fn is_capitalize(subject: &str) -> bool {
    is_capitalize_or_decapitalize(&subject, true)
}

/// Checks whether `subject` is decapitalized and the rest of `subject` is converted to lower case.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_decapitalize("");
/// // => true
/// query::is_decapitalize("John has a motorcycle");
/// // => false
/// query::is_decapitalize("the world is yours");
/// // => true
/// query::is_decapitalize("Żółć niedźwiedzia");
/// // => false
/// use voca_rs::Voca;
/// "the world is yours".is_decapitalize();
/// // => true
/// ```
pub fn is_decapitalize(subject: &str) -> bool {
    is_capitalize_or_decapitalize(&subject, false)
}

fn is_capitalize_or_decapitalize(subject: &str, if_capitalize: bool) -> bool {
    match subject.len() {
        0 => true,
        _ => {
            let first_letter = crate::chop::first(&subject, 1);
            let the_rest = crate::chop::slice(&subject, 1, 0);
            let first_letter_to_check = if if_capitalize {
                is_uppercase(&first_letter)
            } else {
                is_lowercase(&first_letter)
            };
            first_letter_to_check && is_lowercase(&the_rest)
        }
    }
}
/// Checks whether `subject` contains only digit characters.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_digit("35");
/// // => true
/// query::is_digit("1.5");
/// // => false
/// query::is_digit("0xFF");
/// // => false
/// query::is_digit("ten");
/// // => false
/// use voca_rs::Voca;
/// "35".is_digit();
/// // => true
/// ```
pub fn is_digit(subject: &str) -> bool {
    let subject_len = subject.len();
    if subject_len == 0 {
        return true;
    }

    crate::split::chars(&subject)
        .iter()
        .filter(|c| {
            let mut current_char = String::new();
            current_char.push_str(c);
            crate::utils::DIGITS.contains(&current_char)
        })
        .count()
        == subject_len
}

/// Checks whether `subject` is empty.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_empty("");
/// // => true
/// query::is_empty("   ");
/// // => false
/// query::is_empty("sun");
/// // => false
/// use voca_rs::Voca;
/// "".is_empty();
/// // => true
/// ```
pub fn is_empty(subject: &str) -> bool {
    if subject.is_empty() {
        return true;
    }

    false
}

/// Checks whether `subject` is is a `foreign_key`.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_foreign_key("");
/// // => true
/// query::is_foreign_key("foo_bar_id");
/// // => true
/// query::is_foreign_key("foo_bar");
/// // => false
/// use voca_rs::Voca;
/// "foo_bar_id".is_foreign_key();
/// // => true
/// "foo_bar".is_foreign_key();
/// // => false
/// ```
pub fn is_foreign_key(subject: &str) -> bool {
    subject == crate::chop::foreign_key(&subject)
}

/// Checks whether `subject` has only lower case characters.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_lowercase("motorcycle");
/// // => true
/// query::is_lowercase("John");
/// // => false
/// query::is_lowercase("T1000");
/// // => false
/// use voca_rs::Voca;
/// "motorcycle".is_lowercase();
/// // => true
/// ```
pub fn is_lowercase(subject: &str) -> bool {
    is_upper_or_lowercase(subject, true)
}

/// Checks whether `subject` has the first character in lower case.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_lower_first("motorcycle");
/// // => true
/// query::is_lower_first("John");
/// // => false
/// query::is_lower_first("T1000");
/// // => false
/// query::is_lower_first("żółć niedźwiedzia");
/// // => true
/// use voca_rs::Voca;
/// "motorcycle".is_lower_first();
/// // => true
/// ```
pub fn is_lower_first(subject: &str) -> bool {
    match subject.len() {
        0 => true,
        _ => {
            let first_letter = crate::split::chars(subject)[0];
            is_upper_or_lowercase(first_letter, true)
        }
    }
}

/// Checks whether `subject` is kebab-cased.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_kebab_case("");
/// // => true
/// query::is_kebab_case("bird-flight");
/// // => true
/// query::is_kebab_case("bird flight");
/// // => false
/// query::is_kebab_case("-BIRD-FLIGHT-");
/// // => false
/// use voca_rs::Voca;
/// "bird-flight".is_kebab_case();
/// // => true
/// ```
pub fn is_kebab_case(subject: &str) -> bool {
    subject == crate::case::kebab_case(&subject)
}

/// Checks whether `subject` is numeric.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_numeric("350");
/// // => true
/// query::is_numeric("-20.5");
/// // => true
/// query::is_numeric("0xFF");
/// // => true
/// query::is_numeric("1.5E+2");
/// // => true
/// query::is_numeric("five");
/// // => false
/// use voca_rs::Voca;
/// "350".is_numeric();
/// // => true
/// ```
pub fn is_numeric(subject: &str) -> bool {
    if subject.is_empty() {
        return true;
    }

    fn parse_str_num(n: &str) -> bool {
        match n.find('.') {
            Some(_) => n.parse::<f32>().is_ok(),
            None => n.parse::<i32>().is_ok(),
        }
    }

    let sbj = subject.to_lowercase();
    match subject.to_lowercase().find('e') {
        Some(_) => {
            let s: Vec<&str> = sbj.split('e').collect();
            parse_str_num(s[0]) && parse_str_num(s[1])
        }
        None => {
            if starts_with(&subject.to_lowercase(), "0x") {
                let s = sbj.trim_start_matches("0x");
                i32::from_str_radix(s, 16).is_ok()
            } else {
                parse_str_num(subject)
            }
        }
    }
}

/// Checks whether `subject` is PascalCased.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_pascal_case("");
/// // => true
/// query::is_pascal_case("BirdFlight");
/// // => true
/// query::is_pascal_case("bird flight");
/// // => false
/// query::is_pascal_case("-BIRD-FLIGHT-");
/// // => false
/// use voca_rs::Voca;
/// "BirdFlight".is_pascal_case();
/// // => true
/// ```
pub fn is_pascal_case(subject: &str) -> bool {
    subject == crate::case::pascal_case(&subject)
}

/// Checks whether `subject` is SHOUTY-KEBAB-CASED.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_shouty_kebab_case("");
/// // => true
/// query::is_shouty_kebab_case("BIRD-FLIGHT");
/// // => true
/// query::is_shouty_kebab_case("bird flight");
/// // => false
/// query::is_shouty_kebab_case("-BIRD-FLIGHT-");
/// // => false
/// use voca_rs::Voca;
/// "BIRD-FLIGHT".is_shouty_kebab_case();
/// // => true
/// ```
pub fn is_shouty_kebab_case(subject: &str) -> bool {
    subject == crate::case::shouty_kebab_case(&subject)
}

/// Checks whether `subject` is snake_cased.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_snake_case("");
/// // => true
/// query::is_snake_case("bird_flight");
/// // => true
/// query::is_snake_case("bird flight");
/// // => false
/// query::is_snake_case("-BIRD-FLIGHT-");
/// // => false
/// use voca_rs::Voca;
/// "bird_flight".is_snake_case();
/// // => true
/// ```
pub fn is_snake_case(subject: &str) -> bool {
    subject == crate::case::snake_case(&subject)
}

/// Checks whether `subject` is SHOUTY_SNAKE_CASED.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_shouty_snake_case("");
/// // => true
/// query::is_shouty_snake_case("BIRD_FLIGHT");
/// // => true
/// query::is_shouty_snake_case("bird flight");
/// // => false
/// query::is_shouty_snake_case("-BIRD-FLIGHT-");
/// // => false
/// use voca_rs::Voca;
/// "BIRD_FLIGHT".is_shouty_snake_case();
/// // => true
/// ```
pub fn is_shouty_snake_case(subject: &str) -> bool {
    subject == crate::case::shouty_snake_case(&subject)
}

/// Checks whether `subject` is a titlecased string and there is at least one character.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_title("This Is String Example...Wow!!!");
/// // => true
/// query::is_title("This is string example....wow!!!");
/// // => false
/// use voca_rs::Voca;
/// "This Is String Example...Wow!!!".is_title();
/// // => true
/// ```
pub fn is_title(subject: &str) -> bool {
    if subject.is_empty() {
        return false;
    }
    let words = crate::split::words(&subject);
    let subject_len = words.len();
    words
        .iter()
        .filter(|w| {
            let mut res = String::with_capacity(w.len());
            for (i, c) in crate::split::chars(w).iter().enumerate() {
                if (i == 0 && c == &c.to_uppercase()) || (i > 0 && c == &c.to_lowercase()) {
                    res.push_str(&c)
                }
            }
            res.len() == w.len()
        })
        .count()
        == subject_len
}

/// Checks whether `subject` is Train-Cased.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_train_case("");
/// // => true
/// query::is_train_case("Goodbye-Blue-Sky");
/// // => true
/// query::is_train_case("bird flight");
/// // => false
/// query::is_train_case("-BIRD-FLIGHT-");
/// // => false
/// use voca_rs::Voca;
/// "Goodbye-Blue-Sky".is_train_case();
/// // => true
/// ```
pub fn is_train_case(subject: &str) -> bool {
    subject == crate::case::train_case(&subject)
}

/// Checks whether `subject` has only upper case characters.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_uppercase("ACDC");
/// // => true
/// query::is_uppercase("Morning");
/// // => false
/// use voca_rs::Voca;
/// "ACDC".is_uppercase();
/// // => true
/// ```
pub fn is_uppercase(subject: &str) -> bool {
    is_upper_or_lowercase(subject, false)
}

/// Checks whether `subject` has the first character in upper case.
///
/// # Arguments
///
/// * `subject` - The string to verify.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::is_upper_first("motorcycle");
/// // => false
/// query::is_upper_first("John");
/// // => true
/// query::is_upper_first("T1000");
/// // => true
/// query::is_upper_first("Żółć niedźwiedzia");
/// // => true
/// use voca_rs::Voca;
/// "John".is_upper_first();
/// // => true
/// ```
pub fn is_upper_first(subject: &str) -> bool {
    match subject.len() {
        0 => true,
        _ => {
            let first_letter = crate::split::chars(subject)[0];
            is_upper_or_lowercase(first_letter, false)
        }
    }
}

fn is_upper_or_lowercase(subject: &str, lowercase: bool) -> bool {
    if subject.is_empty() {
        return true;
    }

    let mut res = true;
    crate::split::chars(subject).into_iter().for_each(|s| {
        s.chars().for_each(|c| {
            if (lowercase && c.is_uppercase()) || (!lowercase && c.is_lowercase()) {
                res = false;
            }
        })
    });
    res
}

/// Checks whether `subject` matches the regular expression `pattern`.
/// NOTE: Executes regular expressions only on valid UTF-8 while exposing match locations as byte indices into the search string (see case #4).
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `pattern` - The RegExp pattern to match, it is transformed to Regex::new(pattern).
/// * `position` - The position to start matching.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::matches("pluto", "a", 0);
/// // => false
/// query::matches("pluto", r"plu.{2}", 0);
/// // => true
/// query::matches("apollo 11", r"\d{3}", 0);
/// // => false
/// query::matches("Zażółć gęślą jaźń", "gęślą", 11);
/// // => true (because "gęślą" starts from 11 not 7)
/// use voca_rs::Voca;
/// "pluto".matches_me(r"plu.{2}", 0);
/// // => true
/// ```
pub fn matches(subject: &str, pattern: &str, position: usize) -> bool {
    let subject_len = crate::split::chars(&subject).len();
    if subject_len == 0 {
        return false;
    }
    if position >= subject_len {
        return false;
    }
    match pattern.len() {
        0 => true,
        _ => {
            let re: Regex = match Regex::new(pattern) {
                Ok(re) => re,
                Err(_) => return false,
            };
            re.is_match_at(&subject, position)
        }
    }
}

/// Checks whether `subject` contains all characters from `search` starting from `position`. Respects an order of characters.
///
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `search` - The ending string.
/// * `position` - The position to start searching.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::query("starship", "star", 0);
/// // => true
/// query::query("the world is yours", "te wld", 0);
/// // => true
/// query::query("galaxy", "g", 1);
/// // => false
/// use voca_rs::Voca;
/// "starship".query("star", 0);
/// // => true
/// ```
pub fn query(subject: &str, search: &str, position: usize) -> bool {
    let subject_len = crate::count::count(&subject);
    if subject_len < position {
        return false;
    }
    if subject_len == 0 || search.is_empty() {
        return true;
    }
    let mut i: usize = 0;
    let q = crate::split::chars(&search);
    let q_len = crate::split::chars(&search).len();
    crate::split::chars(&subject.to_owned()[subject.char_indices().nth(position).unwrap().0..])
        .into_iter()
        .filter(|c| {
            if i < q_len {
                if c == &q[i] {
                    i += 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .count()
        == crate::count::count(&search)
}

/// Checks whether `subject` starts with `start`.
///
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `start` - The starting string.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// query::starts_with("say hello to my little friend", "say hello");
/// // => true
/// query::starts_with("say hello to my little friend", "hello");
/// // => false
/// use voca_rs::Voca;
/// "say hello to my little friend".starts_with_me("say hello");
/// // => true
/// ```
pub fn starts_with(subject: &str, start: &str) -> bool {
    if subject.is_empty() || start.is_empty() {
        return true;
    }
    subject.starts_with(start)
}
