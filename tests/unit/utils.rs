//! voca_rs::utils testing

#[test]
fn version() {
    assert_eq!(voca_rs::utils::VERSION, "1.8.0");
}
#[test]
fn ascii_letters() {
    assert_eq!(
        voca_rs::utils::ASCII_LETTERS,
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
#[test]
fn ascii_lowercase() {
    assert_eq!(
        voca_rs::utils::ASCII_LOWERCASE,
        "abcdefghijklmnopqrstuvwxyz"
    );
}
#[test]
fn ascii_uppercase() {
    assert_eq!(
        voca_rs::utils::ASCII_UPPERCASE,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
}
#[test]
fn digits() {
    assert_eq!(voca_rs::utils::DIGITS, "0123456789");
}
#[test]
fn hexdigits() {
    assert_eq!(voca_rs::utils::HEXDIGITS, "0123456789abcdefABCDEF");
}
#[test]
fn octdigits() {
    assert_eq!(voca_rs::utils::OCTDIGITS, "01234567");
}
#[test]
fn punctuation() {
    assert_eq!(
        voca_rs::utils::PUNCTUATION,
        "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"
    );
}
#[test]
fn printable() {
    assert_eq!(voca_rs::utils::PRINTABLE, "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r");
}
#[test]
fn whitespace() {
    assert_eq!(voca_rs::utils::WHITESPACE, " \t\n\r");
}
