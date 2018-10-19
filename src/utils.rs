//! Utility functions and properties.

/// A property that contains the library <a href="http://semver.org/">semantic version number</a>.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::VERSION;
/// // => "0.4.0"
/// ```
pub const VERSION: &'static str = "0.5.0";

/// The concatenation of the `ascii_lowercase` and `ascii_uppercase` constants described below. This value is not locale-dependent.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::ASCII_LETTERS;
/// // => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
/// ```
// TODO: make a combination of ASCII_LOWERCASE, ASCII_UPPERCASE
pub const ASCII_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// The lowercase letters `"abcdefghijklmnopqrstuvwxyz"`. This value is not locale-dependent and will not change.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::ASCII_LOWERCASE;
/// // => "abcdefghijklmnopqrstuvwxyz"
/// ```
pub const ASCII_LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";

/// The uppercase letters `"ABCDEFGHIJKLMNOPQRSTUVWXYZ"`. This value is not locale-dependent and will not change.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::ASCII_UPPERCASE;
/// // => "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
/// ```
// TODO: make a variation of ASCII_LOWERCASE
pub const ASCII_UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// The string "0123456789".
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::DIGITS;
/// // => "0123456789"
/// ```
pub const DIGITS: &'static str = "0123456789";

/// The string `"0123456789abcdefABCDEF"`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::HEXDIGITS;
/// // => "0123456789abcdefABCDEF"
/// ```
// TODO: make a combination of slices of DIGITS, ASCII_LOWERCASE, ASCII_UPPERCASE
pub const HEXDIGITS: &'static str = "0123456789abcdefABCDEF";

/// The string `"01234567"`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::OCTDIGITS;
/// // => "01234567"
/// ```
// TODO: make a slice of DIGITS
pub const OCTDIGITS: &'static str = "01234567";

/// The string `!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::PUNCTUATION;
/// // => !"#$%&'()*+,-./:;<=>?@[\]^_`{|}~
/// ```
pub const PUNCTUATION: &'static str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

/// String of ASCII characters which are considered printable. This is a combination of `digits`, `ascii_letters`, `punctuation`, and `whitespace`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::PRINTABLE;
/// // => 0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r
/// ```
// TODO: make a conbination of DIGITS, ASCII_LETTERS, PUNCTUATION, WHITESPACE
pub const PRINTABLE: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r";

/// The string `" \t\n\r"`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::WHITESPACE;
/// // => " \t\n\r"
/// ```
pub const WHITESPACE: &'static str = " \t\n\r";
