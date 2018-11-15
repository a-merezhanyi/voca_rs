//! Returns the index of `search` in `subject`.

use count;
use split;
/// Returns an array of all occurrence index of `search` in `subject` or an empty array if not found. Case sensitive.
///
/// # Arguments
///
/// * `subject` - The string where to search.
/// * `search` - The string to search.
/// * `from_index` - The index to start searching
///
/// # Example
/// ```
/// use voca_rs::*;
/// index::index_all("morning", "n", 0);
/// // => [3, 5]
/// index::index_all("Zażółć gęślą jaźń", "aęą", 0);
/// // => [1, 8, 11, 14]
/// index::index_all("evening", "o", 0);
/// // => []
pub fn index_all(subject: &str, search: &str, from_index: usize) -> Vec<usize> {
    if subject.is_empty() || count::count(&subject) < from_index {
        return vec![];
    }
    let string_slice = &subject[subject.char_indices().nth(from_index).unwrap().0..];
    let mut res = Vec::new();
    for (i, c) in split::chars(string_slice).iter().enumerate() {
        if search.contains(c) {
            res.push(i)
        }
    }
    res
}

/// Returns the first occurrence index of `search` in `subject` or -1 if not found. Case sensitive.
///
/// # Arguments
///
/// * `subject` - The string where to search.
/// * `search` - The string to search.
/// * `from_index` - The index to start searching
///
/// # Example
/// ```
/// use voca_rs::*;
/// index::index_of("morning", "n", 0);
/// // => 3
/// index::index_of("Zażółć gęślą jaźń", "gęślą", 0);
/// // => 7
/// index::index_of("evening", "o", 0);
/// // => -1
pub fn index_of(subject: &str, search: &str, from_index: usize) -> i8 {
    match search.len() {
        0 => 0,
        _ => {
            if count::count(&subject) < from_index {
                return -1;
            }
            let string_slice = &subject[subject.char_indices().nth(from_index).unwrap().0..];
            match split::chars(string_slice)
                .iter()
                .enumerate()
                .position(|(pos, _)| {
                    match &string_slice[string_slice.char_indices().nth(pos).unwrap().0..]
                        .find(search)
                    {
                        Some(x) => *x == 0,
                        None => false,
                    }
                }) {
                Some(x) => x as i8,
                None => -1,
            }
        }
    }
}

/// Returns the last occurrence index of `search` in `subject` or -1 if not found. Case sensitive.
///
/// # Arguments
///
/// * `subject` - The string where to search.
/// * `search` - The string to search.
/// * `from_index` - The index to start searching
///
/// # Example
/// ```
/// use voca_rs::*;
/// index::last_index_of("morning", "n", 0);
/// // => 5
/// index::last_index_of("evening", "o", 0);
/// // => -1
pub fn last_index_of(subject: &str, search: &str, from_index: usize) -> i8 {
    match search.len() {
        0 => 0,
        _ => {
            if count::count(&subject) < from_index {
                return -1;
            }
            let string_slice = &subject[subject.char_indices().nth(from_index).unwrap().0..];
            let string_chars = split::chars(string_slice);
            match string_chars.iter().enumerate().rev().position(|(pos, _)| {
                match &string_slice[string_slice.char_indices().nth(pos).unwrap().0..].find(search)
                {
                    Some(x) => *x == 0,
                    None => false,
                }
            }) {
                Some(x) => (string_chars.len() - x - 1) as i8,
                None => -1,
            }
        }
    }
}
