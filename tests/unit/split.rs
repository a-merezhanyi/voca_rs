//! voca_rs::split testing
use voca_rs::Voca;

#[test]
fn to_chars() {
    assert_eq!(
        voca_rs::split::chars("gravity"),
        ["g", "r", "a", "v", "i", "t", "y"]
    );
    assert_eq!(voca_rs::split::chars("  "), [" ", " "]);
    assert_eq!(voca_rs::split::chars("a b"), ["a", " ", "b"]);
    assert_eq!(voca_rs::split::chars("ÜbER"), ["Ü", "b", "E", "R"]);
    assert_eq!(voca_rs::split::chars("\n\t"), ["\n", "\t"]);
    assert_eq!(voca_rs::split::chars(""), [""]);
}
#[test]
fn _to_chars() {
    assert_eq!("gravity"._chars(), ["g", "r", "a", "v", "i", "t", "y"]);
}
#[test]
fn by_pattern() {
    assert_eq!(
        voca_rs::split::split("gravity can cross dimensions", " "),
        ["gravity", "can", "cross", "dimensions"]
    );
    assert_eq!(
        voca_rs::split::split("*dying*star*", "*"),
        ["", "dying", "star"]
    );
    assert_eq!(voca_rs::split::split("dying star", ""), ["dying star"]);
    assert_eq!(voca_rs::split::split("Über Stern", ""), ["Über Stern"]);
    assert_eq!(voca_rs::split::split("", ""), [""]);
}
#[test]
fn _by_pattern() {
    assert_eq!(
        "gravity can cross dimensions"._split(" "),
        ["gravity", "can", "cross", "dimensions"]
    );
}
#[test]
fn words() {
    assert_eq!(
        voca_rs::split::words("gravity can cross dimensions"),
        ["gravity", "can", "cross", "dimensions"]
    );
    assert_eq!(
        voca_rs::split::words("gravity    dying\r\nstar\tfalling"),
        ["gravity", "dying", "star", "falling"]
    );
    assert_eq!(
        voca_rs::split::words("Zażółć gęślą jaźń"),
        ["Zażółć", "gęślą", "jaźń"]
    );
    assert_eq!(
        voca_rs::split::words("splitCamelCase"),
        ["split", "Camel", "Case"]
    );
    assert_eq!(
        voca_rs::split::words("split-some kind_of_mixed CaseHere"),
        ["split", "some", "kind", "of", "mixed", "Case", "Here"]
    );
    assert_eq!(
        voca_rs::split::words("LazyLoad with XMLHttpRequest and snake_case"),
        ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
    );
}
#[test]
fn _words() {
    assert_eq!(
        "gravity can cross dimensions"._words(),
        ["gravity", "can", "cross", "dimensions"]
    );
}
#[test]
fn to_graphemes() {
    assert_eq!(
        voca_rs::split::graphemes("a̐éö̲\r\n"),
        ["a̐", "é", "ö̲", "\r\n"]
    );
    assert_eq!(voca_rs::split::graphemes(""), [""]);
}
#[test]
fn _to_graphemes() {
    assert_eq!("a̐éö̲\r\n"._graphemes(), ["a̐", "é", "ö̲", "\r\n"]);
}
#[test]
fn code_points() {
    assert_eq!(voca_rs::split::code_points(""), []);
    assert_eq!(voca_rs::split::code_points("rain"), [114, 97, 105, 110]);
    assert_eq!(
        voca_rs::split::code_points("Un garçon de café"),
        [85, 110, 32, 103, 97, 114, 231, 111, 110, 32, 100, 101, 32, 99, 97, 102, 233]
    );
    assert_eq!(
        voca_rs::split::code_points("a̐éö̲\r\n"),
        [97, 784, 101, 769, 111, 776, 818, 13, 10]
    );
}
#[test]
fn _code_points() {
    assert_eq!("rain"._code_points(), [114, 97, 105, 110]);
}
