//! voca_rs::utils testing

#[test]
fn version() {
    assert_eq!(voca_rs::utils::VERSION, "1.11.0");
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

// https://github.com/chowdhurya/rust-unidecode/blob/master/tests/unidecode.rs
// Tests that every character outputted by the unidecode() function is valid
// ASCII.
#[test]
fn test_all_ascii() {
    use std::char;

    let valid_unicode = (0x0..0xD7FF + 1).chain(0x0E000..0x10FFFF + 1);
    for i in valid_unicode {
        match char::from_u32(i) {
            Some(ch) => {
                for ascii_ch in voca_rs::utils::unidecode(&ch.to_string()).chars() {
                    let x = ascii_ch as u32;
                    if x > 127 {
                        panic!("Data contains non-ASCII character (Dec: {})", x);
                    }
                }
            }
            None => panic!("Test written incorrectly; invalid Unicode"),
        }
    }
}

// https://github.com/chowdhurya/rust-unidecode/blob/master/tests/unidecode.rs
// These tests were ported directly from the original `Text::Unidecode` Perl
// module.
#[test]
fn test_conversion() {
    assert_eq!(voca_rs::utils::unidecode("Æneid"), "AEneid");
    assert_eq!(voca_rs::utils::unidecode("étude"), "etude");
    assert_eq!(voca_rs::utils::unidecode("北亰"), "Bei Jing ");
    assert_eq!(voca_rs::utils::unidecode("ᔕᓇᓇ"), "shanana");
    assert_eq!(voca_rs::utils::unidecode("ᏔᎵᏆ"), "taliqua");
    assert_eq!(voca_rs::utils::unidecode("ܦܛܽܐܺ"), "ptu'i");
    assert_eq!(voca_rs::utils::unidecode("अभिजीत"), "abhijiit");
    assert_eq!(voca_rs::utils::unidecode("অভিজীত"), "abhijiit");
    assert_eq!(voca_rs::utils::unidecode("അഭിജീത"), "abhijiit");
    assert_eq!(voca_rs::utils::unidecode("മലയാലമ്"), "mlyaalm");
    assert_eq!(voca_rs::utils::unidecode("げんまい茶"), "genmaiCha ");
}

// https://github.com/chowdhurya/rust-unidecode/blob/master/tests/unidecode.rs
#[test]
fn test_unidecode_char() {
    assert_eq!(voca_rs::utils::unidecode_char('Æ'), "AE");
    assert_eq!(voca_rs::utils::unidecode_char('北'), "Bei ");
    assert_eq!(voca_rs::utils::unidecode_char('亰'), "Jing ");
    assert_eq!(voca_rs::utils::unidecode_char('ᔕ'), "sha");
}
