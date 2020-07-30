//! Manipulate with the `subject`.

use index;
/// Returns a copy of `subject` expands spaces using the tab characters.
///
/// # Arguments
///
/// * `subject` - The string to expand.
/// * `tabsize` - The tab size.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::expand_spaces("This  is  good", 2);
/// // => "This\tis\tgood"
/// manipulate::expand_spaces("Café del  Mar", 2);
/// // => "Café del\tMar"
/// use voca_rs::Voca;
/// "This  is  good"._expand_spaces(2);
/// // => "This\tis\tgood"
/// ```
pub fn expand_spaces(subject: &str, tabsize: usize) -> String {
    if subject.is_empty() || tabsize == 0 {
        subject.to_string()
    } else {
        subject.replace(&" ".repeat(tabsize), "\t")
    }
}

/// Returns a copy of `subject` expands the tab characters using spaces.
///
/// # Arguments
///
/// * `subject` - The string to expand.
/// * `tabsize` - The tab size.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::expand_tabs("This is\tgood", 4);
/// // => "This is    good"
/// manipulate::expand_tabs("no\tspaces", 0);
/// // => "nospaces"
/// use voca_rs::Voca;
/// "This is\tgood"._expand_tabs(4);
/// // => "This is    good"
/// ```
pub fn expand_tabs(subject: &str, tabsize: usize) -> String {
    if subject.is_empty() {
        "".to_string()
    } else {
        subject.replace("\t", &" ".repeat(tabsize))
    }
}

/// Inserts into `subject` a string `to_insert` at specified `position`.
///
/// # Arguments
///
/// * `subject` - The string where to insert.
/// * `to_insert` - The string to be inserted
/// * `position` - The position to insert.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::insert("ct", "a", 1);
/// // => "cat"
/// manipulate::insert("sunny", " day", 5);
/// // => "sunny day"
/// use voca_rs::Voca;
/// "ct"._insert("a", 1);
/// // => "cat"
/// ```
pub fn insert(subject: &str, to_insert: &str, position: usize) -> String {
    let subject_len = subject.len();
    if subject_len == 0 || to_insert.is_empty() {
        return subject.to_string();
    }
    let insert_position = if position > subject_len {
        subject_len
    } else {
        position
    };
    let prefix = crate::split::chars(&subject)[..insert_position].join("");
    let sufix = crate::split::chars(&subject)[insert_position..].join("");
    format!("{}{}{}", prefix, to_insert, sufix)
}

use utils::unidecode;
/// Latinises the `subject` by removing diacritic characters.
///
/// # Arguments
///
/// * `subject` - The string to latinise.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::latinise("cafe\u{0301}");
/// // => "cafe"
/// manipulate::latinise("août décembre");
/// // => aout decembre
/// manipulate::latinise("как прекрасен этот мир");
/// // => kak prekrasen etot mir
/// use voca_rs::Voca;
/// "cafe\u{0301}"._latinise();
/// // => "cafe"
/// ```
pub fn latinise(subject: &str) -> String {
    if subject.is_empty() {
        "".to_string()
    } else {
        unidecode(subject)
    }
}

/// Pads `subject` to a new `length`.
///
/// # Arguments
///
/// * `subject` - The string to pad.
/// * `length` - The length to pad the string. No changes are made if `length` is less than `subject.len()`.
/// * `pad` - The string to be used for padding.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::pad("dog", 5, "");
/// // => " dog "
/// manipulate::pad("bird", 6, "-");
/// // => "-bird-"
/// manipulate::pad("Café del Mar", 15, "-=");
/// // => "-Café del Mar-="
/// use voca_rs::Voca;
/// "dog"._pad(5, "");
/// // => " dog "
/// ```
pub fn pad(subject: &str, length: usize, pad: &str) -> String {
    let subject_len = crate::count::count_graphemes(&subject);
    match subject_len {
        0 => "".to_string(),
        _ => {
            if subject_len >= length {
                subject.to_string()
            } else {
                pad_left_right(&subject, length, &pad, PadMode::Both)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PadMode {
    Both,
    Left,
    Right,
}

fn pad_left_right(subject: &str, length: usize, pad: &str, pad_mode: PadMode) -> String {
    let width = length - crate::count::count_graphemes(&subject);
    let to_add = if pad.is_empty() { " " } else { pad };
    let times = width / to_add.len();
    let str_to_add = to_add.repeat(times + 1);
    let string_to_add = crate::split::chars(&str_to_add);
    let padding = if pad_mode == PadMode::Left || pad_mode == PadMode::Right {
        string_to_add[..width].join("")
    } else {
        "".to_string()
    };

    match pad_mode {
        PadMode::Both => {
            let string_to_add_len = string_to_add.len();
            let middle = if string_to_add_len < width {
                string_to_add_len / 2
            } else {
                width / 2
            };
            let add = if width % 2 != 0 { 1 } else { 0 };
            let prefix = string_to_add[..middle].join("");
            let sufix = string_to_add[..middle + add].join("");
            format!("{}{}{}", prefix, subject, sufix)
        }
        PadMode::Left => format!("{}{}", padding, subject),
        PadMode::Right => format!("{}{}", subject, padding),
    }
}

/// Pads `subject` from left to a new `length`.
///
/// # Arguments
///
/// * `subject` - The string to pad.
/// * `length` - The length to pad the string. No changes are made if `length` is less than `subject.len()`.
/// * `pad` - The string to be used for padding.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::pad_left("dog", 5, "");
/// // => "  dog"
/// manipulate::pad_left("bird", 6, "-");
/// // => "--bird"
/// manipulate::pad_left("Café del Mar", 15, "-=");
/// // => "-=-Café del Mar"
/// use voca_rs::Voca;
/// "dog"._pad_left(5, "");
/// // => "  dog"
/// ```
pub fn pad_left(subject: &str, length: usize, pad: &str) -> String {
    let subject_len = crate::count::count_graphemes(&subject);
    match subject_len {
        0 => "".to_string(),
        _ => {
            if subject_len >= length {
                subject.to_string()
            } else {
                pad_left_right(&subject, length, &pad, PadMode::Left)
            }
        }
    }
}

/// Pads `subject` from right to a new `length`.
///
/// # Arguments
///
/// * `subject` - The string to pad.
/// * `length` - The length to pad the string. No changes are made if `length` is less than `subject.len()`.
/// * `pad` - The string to be used for padding.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::pad_right("dog", 5, "");
/// // => "dog  "
/// manipulate::pad_right("bird", 6, "-");
/// // => "bird--"
/// manipulate::pad_right("Café del Mar", 15, "-=");
/// // => "Café del Mar-=-"
/// use voca_rs::Voca;
/// "dog"._pad_right(5, "");
/// // => "dog  "
/// ```
pub fn pad_right(subject: &str, length: usize, pad: &str) -> String {
    let subject_len = crate::count::count_graphemes(&subject);
    match subject_len {
        0 => "".to_string(),
        _ => {
            if subject_len >= length {
                subject.to_string()
            } else {
                pad_left_right(&subject, length, &pad, PadMode::Right)
            }
        }
    }
}

/// Repeats the `subject` number of `times`.
///
/// # Arguments
///
/// * `subject` - The string to repeat.
/// * `times` - The number of times to repeat.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::repeat("w", 3);
/// // => "www"
/// manipulate::repeat("world", 0);
/// // => ""
/// use voca_rs::Voca;
/// "w"._repeat(3);
/// // => "www"
/// ```
pub fn repeat(subject: &str, times: usize) -> String {
    if subject.is_empty() || times == 0 {
        return "".to_string();
    }

    subject.repeat(times)
}

/// Replaces the matches of `pattern` with `replacement`.
///
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `pattern` - The pattern which match is replaced. Only the first occurrence replaced.
/// * `replacement` - The string which replaces `pattern` match.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::replace("swan", "wa", "u");
/// // => "sun"
/// manipulate::replace("domestic duck", "d", "D");
/// // => "Domestic duck"
/// manipulate::replace("Café del Mar cafe\u{0301}", "é", "e");
/// // => "Cafe del Mar café"
/// use voca_rs::Voca;
/// "swan"._replace("wa", "u");
/// // => "sun"
/// ```
pub fn replace(subject: &str, pattern: &str, replacement: &str) -> String {
    if subject.is_empty() || pattern.is_empty() {
        return subject.to_string();
    }
    match index::index_of(&subject, &pattern, 0) {
        -1 => subject.to_string(),
        x => splice(
            &subject,
            x as isize,
            crate::count::count(&pattern),
            &replacement,
        ),
    }
}

/// Replaces all matches of `pattern` with `replacement`.
///
/// # Arguments
///
/// * `subject` - The string to verify.
/// * `pattern` - The pattern which match is replaced. All matches are replaced.
/// * `replacement` - The string which replaces `pattern` match.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::replace_all("swan", "wa", "u");
/// // => "sun"
/// manipulate::replace_all("domestic duck", "d", "D");
/// // => "Domestic Duck"
/// manipulate::replace_all("Café del Mar café", "é", "e");
/// // => "Cafe del Mar cafe"
/// use voca_rs::Voca;
/// "swan"._replace_all("wa", "u");
/// // => "sun"
/// ```
pub fn replace_all(subject: &str, pattern: &str, replacement: &str) -> String {
    if subject.is_empty() || pattern.is_empty() {
        return subject.to_string();
    }
    subject.replace(pattern, replacement)
}

/// Reverses the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to reverse.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::reverse("winter");
/// // => "retniw"
/// use voca_rs::Voca;
/// "winter"._reverse();
/// // => "retniw"
/// ```
pub fn reverse(subject: &str) -> String {
    if subject.is_empty() {
        return "".to_string();
    }

    subject.chars().rev().collect()
}

use unicode_segmentation::UnicodeSegmentation;
/// Reverses the `subject` taking care of surrogate pairs and combining marks.
///
/// # Arguments
///
/// * `subject` - The string to reverse.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::reverse_grapheme("café");
/// // => "éfac"
/// manipulate::reverse_grapheme("a̐éö̲");
/// // => "ö̲éa̐"
/// use voca_rs::Voca;
/// "café"._reverse_grapheme();
/// // => "éfac"
/// ```
pub fn reverse_grapheme(subject: &str) -> String {
    if subject.is_empty() {
        return "".to_string();
    }

    UnicodeSegmentation::graphemes(subject, true)
        .rev()
        .collect::<Vec<&str>>()
        .join("")
}

/// Slugifies the `subject`. Cleans the `subject` by replacing diacritics with corresponding latin characters.
///
/// # Arguments
///
/// * `subject` - The string to latinise.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::slugify("Italian cappuccino drink");
/// // => "italian-cappuccino-drink"
/// manipulate::slugify("caffé latté");
/// // => caffe-latte
/// manipulate::slugify("Хорошая статья: 'XMLHttpRequest 101 Course' \\!/");
/// // => khoroshaia-statia-xmlhttprequest-101-course
/// use voca_rs::Voca;
/// "Italian cappuccino drink"._slugify();
/// // => "italian-cappuccino-drink"
/// ```
pub fn slugify(subject: &str) -> String {
    if subject.is_empty() {
        "".to_string()
    } else {
        crate::split::words(unidecode(subject).replace("'", "").to_lowercase().trim()).join("-")
    }
}

/// Changes `subject` by deleting `delete_count` of characters starting at position `start`. Places a new string `to_add` instead of deleted characters.
///
/// # Arguments
///
/// * `subject` - The string where to insert.
/// * `start` - The position to start changing the string. For a negative position will start from the end of the string.
/// * `delete_count` - The number of characters to delete from string.
/// * `to_add` - The string to be added instead of deleted characters.
///
/// # Example
/// ```
/// use voca_rs::*;
/// manipulate::splice("new year", 0, 4, "");
/// // => "year"
/// manipulate::splice("to jest błąd", 0, 7, "mój");
/// // => "mój błąd"
/// manipulate::splice("Die Schildkröte fliegt.", -7, 0, "und Kröte ");
/// // => "Die Schildkröte und Kröte fliegt."
/// manipulate::splice("Привет", 6, 0, ", Ёлка!");
/// // => "Привет, Ёлка!"
/// use voca_rs::Voca;
/// "new year"._splice(0, 4, "");
/// // => "year"
/// ```
pub fn splice(subject: &str, start: isize, delete_count: usize, to_add: &str) -> String {
    let subject_len = crate::count::count(&subject);
    fn calculate_start_position(start: isize, subject_len: usize) -> usize {
        if start < 0 {
            if start.abs() as usize > subject_len {
                0
            } else {
                subject_len - start.abs() as usize
            }
        } else if (start as usize) >= subject_len {
            subject_len
        } else {
            start as usize
        }
    }

    match delete_count {
        0 => match to_add.len() {
            0 => subject.to_string(),
            _ => {
                let insert_position = calculate_start_position(start, subject_len);
                if insert_position >= subject_len {
                    format!("{}{}", &subject, &to_add)
                } else {
                    insert(&subject, &to_add, insert_position)
                }
            }
        },
        _ => {
            let start_position = calculate_start_position(start, subject_len);
            let end_position = if delete_count > subject_len - start_position {
                subject_len
            } else {
                start_position + delete_count
            };

            format!(
                "{}{}{}",
                crate::chop::first(&subject, start_position),
                &to_add,
                crate::chop::slice(&subject, end_position as isize, 0)
            )
        }
    }
}

/// Removes whitespaces from left and right sides of the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to trim.
/// * `whitespace` - The whitespace characters to trim. List all characters that you want to be stripped.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::trim(" Mother nature ", "");
/// // => "Mother nature"
/// manipulate::trim("-~-Earth~-~", "-~");
/// // => "Earth"
/// use voca_rs::Voca;
/// " Mother nature "._trim("");
/// // => "Mother nature"
/// ```
pub fn trim(subject: &str, whitespace: &str) -> String {
    trim_left_or_right(&subject, &whitespace, true, true)
}

/// Removes whitespaces from the left side of the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to trim.
/// * `whitespace` - The whitespace characters to trim. List all characters that you want to be stripped.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::trim_left(" Mother nature ", "");
/// // => "Mother nature "
/// manipulate::trim_left("-~-Earth~-~", "-~");
/// // => "Earth~-~"
/// use voca_rs::Voca;
/// " Mother nature "._trim_left("");
/// // => "Mother nature "
/// ```
pub fn trim_left(subject: &str, whitespace: &str) -> String {
    trim_left_or_right(&subject, &whitespace, true, false)
}

/// Removes whitespaces from the right side of the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to trim.
/// * `whitespace` - The whitespace characters to trim. List all characters that you want to be stripped.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::trim_right(" Mother nature ", "");
/// // => " Mother nature"
/// manipulate::trim_right("-~-Earth~-~", "-~");
/// // => "-~-Earth"
/// /// use voca_rs::Voca;
/// " Mother nature "._trim_right("");
/// // => " Mother nature"
/// ```
pub fn trim_right(subject: &str, whitespace: &str) -> String {
    trim_left_or_right(&subject, &whitespace, false, true)
}

fn trim_left_or_right(subject: &str, whitespace: &str, to_left: bool, to_right: bool) -> String {
    if subject.is_empty() {
        return subject.to_string();
    }
    if whitespace.is_empty() {
        if to_left && to_right {
            return subject.trim().to_string();
        } else if to_left {
            return subject.trim_start().to_string();
        } else {
            return subject.trim_end().to_string();
        }
    }

    if to_left && to_right {
        subject.trim_matches(|c| whitespace.contains(c)).to_owned()
    } else if to_left {
        subject
            .trim_start_matches(|c| whitespace.contains(c))
            .to_owned()
    } else {
        subject
            .trim_end_matches(|c| whitespace.contains(c))
            .to_owned()
    }
}
/// Pads `subject` from left with zeros to a new `length`.
///
/// # Arguments
///
/// * `subject` - The string to pad.
/// * `length` - The length to pad the string. No changes are made if `length` is less than `subject.len()`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::zfill("123", 5);
/// // => "00123"
/// manipulate::zfill("Café", 7);
/// // => "000Café"
/// use voca_rs::Voca;
/// "123"._zfill(5);
/// // => "00123"
/// ```
pub fn zfill(subject: &str, length: usize) -> String {
    let subject_len = crate::count::count_graphemes(&subject);
    match subject_len {
        0 => "".to_string(),
        _ => {
            if subject_len >= length {
                subject.to_string()
            } else {
                pad_left_right(&subject, length, "0", PadMode::Left)
            }
        }
    }
}

/// Translates characters or replaces substrings in `subject`.
///
/// # Arguments
///
/// * `subject` - The string to translate.
/// * `from` - The string of characters to translate from.
/// * `to` - The string of characters to translate to.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::tr("hello", "el", "ip");
/// // => "hippo"
/// manipulate::tr("légèreté", "éè", "ee");
/// // => "legerete"
/// use voca_rs::Voca;
/// "hello"._tr("el", "ip");
/// // => "hippo"
/// ```
pub fn tr(subject: &str, from: &str, to: &str) -> String {
    if from.is_empty() || subject.is_empty() {
        return subject.to_owned();
    }
    let mut result = String::from(subject);
    let from_symbols = crate::split::graphemes(&from);
    let to_symbols = crate::split::graphemes(&to);
    let to_len = to_symbols.len();

    for (i, c) in from_symbols.iter().enumerate() {
        let new_c = if i < to_len { to_symbols[i] } else { "" };
        result = result.replace(c, new_c);
    }
    result
}

/// Wraps `subject` to a given number of characters using a string break character.
///
/// # Arguments
///
/// * `subject` - The string to wrap.
/// * `width` - The number of characters at which to wrap.
/// * `newline` - The string to add at the end of line. Default value is "\n" (if it's not given).
/// * `indent` - The string to intend the line. Default value is "" (if it's not given).
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::word_wrap("Hello world", 5, "", "");
/// // => "Hello\nworld"
/// manipulate::word_wrap("Hello world", 5, "<br/>", "__");
/// // => "__Hello<br/>__world"
/// use voca_rs::Voca;
/// "Hello world"._word_wrap(5, "", "");
/// // => "Hello\nworld"
/// ```
pub fn word_wrap(subject: &str, width: usize, newline: &str, indent: &str) -> String {
    let mut subject_len = crate::count::count_graphemes(&subject);
    if subject.is_empty() || (subject_len < width && indent.is_empty()) {
        return subject.to_owned();
    }
    let mut result = String::new();
    let mut string = String::from(subject);
    let length = width + 1;
    let new_line = if newline.is_empty() { "\n" } else { newline };
    let indent_sym = if indent.is_empty() { "" } else { indent };

    while subject_len > width {
        let mut subj_part = crate::chop::prune(&string, length, "+");
        subj_part = trim(&crate::chop::slice(&subj_part, 0, -1), "");
        let length_to_cut = crate::count::count_graphemes(&subj_part);
        string = trim(&crate::chop::slice(&string, length_to_cut as isize, 0), "");
        subject_len = crate::count::count_graphemes(&string);

        if subj_part == string || subj_part.is_empty() {
            break;
        }
        let mut is_finished = false;
        let mut subj_part_len = crate::count::count_graphemes(&subj_part);
        while !is_finished && subj_part_len < width {
            let first_char = crate::chop::first(&string, 1);

            if crate::utils::PUNCTUATION.contains(&first_char) {
                subj_part.push_str(&first_char);
                subj_part_len = crate::count::count_graphemes(&subj_part);
                string = trim(&crate::chop::slice(&string, 1, 0), "");
            } else {
                is_finished = true;
            }
        }
        let str_to_insert = format!("{}{}{}", indent_sym, subj_part, new_line);
        result.push_str(&str_to_insert);
    }
    format!("{}{}{}", result, indent_sym, string)
}
