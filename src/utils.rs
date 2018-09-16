//! Utility functions and properties.

/// A property that contains the library <a href="http://semver.org/">semantic version number</a>.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::VERSION;
/// // => "0.1.0"
/// ```
pub const VERSION: &'static str = "0.1.0";

/// The lowercase letters `'abcdefghijklmnopqrstuvwxyz'`. This value is not locale-dependent and will not change.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::ASCII_LOWERCASE;
/// // => "abcdefghijklmnopqrstuvwxyz"
/// ```
pub const ASCII_LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";

/// The uppercase letters 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'. This value is not locale-dependent and will not change.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::ASCII_UPPERCASE;
/// // => "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
/// ```
pub const ASCII_UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
