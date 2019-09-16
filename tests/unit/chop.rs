//! voca_rs::chop testing
use voca_rs::Voca;

#[test]
fn char_at() {
    assert_eq!(voca_rs::chop::char_at("", 0), "");
    assert_eq!(voca_rs::chop::char_at("rain", 0), "r");
    assert_eq!(voca_rs::chop::char_at("rain", 2), "i");
    assert_eq!(voca_rs::chop::char_at("rain", 3), "n");
    assert_eq!(voca_rs::chop::char_at("rain", 40), "n");
    assert_eq!(voca_rs::chop::char_at("b\u{0142}\u{0105}d", 2), "ą");
    assert_eq!(
        voca_rs::chop::char_at("Die Schildkröte fliegt über das Floß.", 12),
        "ö"
    );
    assert_eq!(
        voca_rs::chop::char_at("Как слышно, приём!", 15),
        "ё"
    );
}
#[test]
fn char_at_me() {
    assert_eq!("rain".char_at(0), "r");
}
#[test]
fn code_point_at() {
    assert_eq!(voca_rs::chop::code_point_at("", 0), []);
    assert_eq!(voca_rs::chop::code_point_at("rain", 1), [97]);
    assert_eq!(voca_rs::chop::code_point_at("café", 4), [233]);
    assert_eq!(voca_rs::chop::code_point_at("cafe\u{0301}", 4), [101, 769]);
    assert_eq!(voca_rs::chop::code_point_at("b\u{0142}\u{0105}d", 1), [322]);
    assert_eq!(voca_rs::chop::code_point_at("über das Floß.", 0), [252]);
    assert_eq!(
        voca_rs::chop::code_point_at("a̐éö̲", 3),
        [111, 776, 818]
    );
    assert_eq!(voca_rs::chop::code_point_at("cafe\u{0301}", 0), [99]);
    assert_eq!(voca_rs::chop::code_point_at("cafe\u{0301}", 2), [102]);
    assert_eq!(voca_rs::chop::code_point_at("cafe\u{0301}!", 3), [101, 769]);
    assert_eq!(voca_rs::chop::code_point_at("cafe\u{0301}!", 4), [33]);
    assert_eq!(voca_rs::chop::code_point_at("cafe\u{0301}!", 5), [33]);
    assert_eq!(voca_rs::chop::code_point_at("cafe\u{0301}!", 30), [33]);
}
#[test]
fn code_point_at_me() {
    assert_eq!(voca_rs::chop::code_point_at("rain", 1), [97]);
}
#[test]
fn first() {
    assert_eq!(voca_rs::chop::first("", 0), "");
    assert_eq!(voca_rs::chop::first("a", 0), "");
    assert_eq!(voca_rs::chop::first("rain", 2), "ra");
    assert_eq!(voca_rs::chop::first("rain", 4), "rain");
    assert_eq!(voca_rs::chop::first("rain", 20), "rain");
    assert_eq!(voca_rs::chop::first("b\u{0142}\u{0105}d", 3), "błą");
    assert_eq!(voca_rs::chop::first("über das Floß.", 1), "ü");
    assert_eq!(
        voca_rs::chop::first("Как слышно, приём!", 3),
        "Как"
    );
    assert_eq!(voca_rs::chop::first("e\u{0301}", 1), "e");
}
#[test]
fn first_me() {
    assert_eq!("rain".first(2), "ra");
}
#[test]
fn foreign_key() {
    assert_eq!(voca_rs::chop::foreign_key(""), "");
    assert_eq!(voca_rs::chop::foreign_key("foo_bar"), "foo_bar_id");
    assert_eq!(voca_rs::chop::foreign_key("Foo bar"), "foo_bar_id");
    assert_eq!(voca_rs::chop::foreign_key("Foo Bar"), "foo_bar_id");
    assert_eq!(voca_rs::chop::foreign_key("Foo::Bar"), "bar_id");
    assert_eq!(voca_rs::chop::foreign_key("Test::Foo::Bar"), "bar_id");
    assert_eq!(voca_rs::chop::foreign_key("FooBar"), "foo_bar_id");
    assert_eq!(voca_rs::chop::foreign_key("fooBar"), "foo_bar_id");
    assert_eq!(voca_rs::chop::foreign_key("fooBar3"), "foo_bar3_id");
}
#[test]
fn foreign_key_me() {
    assert_eq!("foo_bar".foreign_key(), "foo_bar_id");
}
#[test]
fn grapheme_at() {
    assert_eq!(voca_rs::chop::grapheme_at("", 0), "");
    assert_eq!(voca_rs::chop::grapheme_at("é", 0), "é");
    assert_eq!(voca_rs::chop::grapheme_at("b\u{0142}\u{0105}d", 1), "ł");
    assert_eq!(voca_rs::chop::grapheme_at("über das Floß.", 0), "ü");
    assert_eq!(voca_rs::chop::grapheme_at("a̐éö̲", 0), "a̐");
    assert_eq!(voca_rs::chop::grapheme_at("cafe\u{0301}", 0), "c");
    assert_eq!(voca_rs::chop::grapheme_at("cafe\u{0301}", 1), "a");
    assert_eq!(voca_rs::chop::grapheme_at("cafe\u{0301}", 2), "f");
    assert_eq!(voca_rs::chop::grapheme_at("cafe\u{0301}", 3), "é");
    assert_eq!(voca_rs::chop::grapheme_at("cafe\u{0301}", 4), "é");
    assert_eq!(voca_rs::chop::grapheme_at("cafe\u{0301}", 5), "é");
    assert_eq!(voca_rs::chop::grapheme_at("cafe\u{0301}", 30), "é");
    assert_eq!(voca_rs::chop::grapheme_at("cafe\u{0301}!", 3), "é");
    assert_eq!(voca_rs::chop::grapheme_at("cafe\u{0301}!", 4), "!");
    assert_eq!(voca_rs::chop::grapheme_at("cafe\u{0301}!", 5), "!");
    assert_eq!(voca_rs::chop::grapheme_at("cafe\u{0301}!", 30), "!");
}
#[test]
fn grapheme_at_me() {
    assert_eq!("a̐éö̲".grapheme_at(0), "a̐");
}
#[test]
fn last() {
    assert_eq!(voca_rs::chop::last("", 0), "");
    assert_eq!(voca_rs::chop::last("a", 0), "");
    assert_eq!(voca_rs::chop::last("a", 1), "a");
    assert_eq!(voca_rs::chop::last("a", 2), "a");
    assert_eq!(voca_rs::chop::last("aa", 2), "aa");
    assert_eq!(voca_rs::chop::last("ab", 3), "ab");
    assert_eq!(voca_rs::chop::last("ab", 20), "ab");
    assert_eq!(voca_rs::chop::last("b\u{0142}\u{0105}d", 2), "ąd");
    assert_eq!(voca_rs::chop::last("helicopter", 1), "r");
    assert_eq!(voca_rs::chop::last("über das Floß.", 2), "ß.");
    assert_eq!(voca_rs::chop::last("e\u{0301}", 1), "\u{0301}");
}
#[test]
fn last_me() {
    assert_eq!("helicopter".last(1), "r");
}
#[test]
fn prune() {
    assert_eq!(voca_rs::chop::prune("", 0, ""), "");
    assert_eq!(voca_rs::chop::prune("a", 0, ""), "");
    assert_eq!(voca_rs::chop::prune("a", 1, ""), "a");
    assert_eq!(voca_rs::chop::prune("a", 2, ""), "a");
    assert_eq!(voca_rs::chop::prune("ab", 2, ""), "ab");
    assert_eq!(voca_rs::chop::prune("ab", 3, ""), "ab");
    assert_eq!(voca_rs::chop::prune("ab", 20, ""), "ab");
    assert_eq!(voca_rs::chop::prune("Once upon a time", 7, ""), "Once...");
    assert_eq!(
        voca_rs::chop::prune("Die Schildkröte fliegt über das Floß.", 19, "~~"),
        "Die Schildkröte~~"
    );
    assert_eq!(voca_rs::chop::prune("once upon", 10, ""), "once upon");
    assert_eq!(
        voca_rs::chop::prune("Как слышно, приём!", 14, ""),
        "Как слышно..."
    );
}
#[test]
fn prune_me() {
    assert_eq!("Once upon a time".prune(7, ""), "Once...");
}
#[test]
fn slice() {
    assert_eq!(voca_rs::chop::slice("", 0, 0), "");
    assert_eq!(voca_rs::chop::slice("a", 0, 0), "a");
    assert_eq!(voca_rs::chop::slice("a", 0, 1), "a");
    assert_eq!(voca_rs::chop::slice("a", 0, 10), "a");
    assert_eq!(voca_rs::chop::slice("a", 2, 0), "");
    assert_eq!(voca_rs::chop::slice("ab", -2, 0), "ab");
    assert_eq!(voca_rs::chop::slice("ab", -3, 0), "ab");
    assert_eq!(voca_rs::chop::slice("ab", -20, 20), "ab");
    assert_eq!(voca_rs::chop::slice("a", -20, -10), "");
    assert_eq!(voca_rs::chop::slice("ab", -20, -1), "a");
    assert_eq!(voca_rs::chop::slice("helicopter", 1, 0), "elicopter");
    assert_eq!(voca_rs::chop::slice("b\u{0142}\u{0105}d", -2, 0), "ąd");
    assert_eq!(
        voca_rs::chop::slice("Die Schildkröte fliegt.", 4, -8),
        "Schildkröte"
    );
    assert_eq!(voca_rs::chop::slice("e\u{0301}", -1, 0), "\u{0301}");
    assert_eq!(
        voca_rs::chop::slice("b\u{0142}\u{0105}d", -20, 10),
        "błąd"
    );
    assert_eq!(voca_rs::chop::slice("b\u{0142}\u{0105}d", -2, 100), "ąd");
}
#[test]
fn slice_me() {
    assert_eq!("helicopter".slice(1, 0), "elicopter");
}
#[test]
fn substr() {
    assert_eq!(voca_rs::chop::substr("", 0, 0), "");
    assert_eq!(voca_rs::chop::substr("a", 0, 0), "a");
    assert_eq!(voca_rs::chop::substr("a", 10, 0), "");
    assert_eq!(voca_rs::chop::substr("a", 0, 1), "a");
    assert_eq!(voca_rs::chop::substr("a", 0, 10), "a");
    assert_eq!(voca_rs::chop::substr("a", 2, 0), "");
    assert_eq!(voca_rs::chop::substr("ab", 1, 0), "b");
    assert_eq!(voca_rs::chop::substr("abcd", 3, 1), "d");
    assert_eq!(voca_rs::chop::substr("abcd", 30, 1), "");
    assert_eq!(voca_rs::chop::substr("ab", 2, 0), "");
    assert_eq!(voca_rs::chop::substr("ab", 3, 0), "");
    assert_eq!(voca_rs::chop::substr("ab", 20, 20), "");
    assert_eq!(voca_rs::chop::substr("a", 20, 10), "");
    assert_eq!(voca_rs::chop::substr("helicopter", 1, 0), "elicopter");
    assert_eq!(voca_rs::chop::substr("b\u{0142}\u{0105}d", 1, 2), "łą");
    assert_eq!(voca_rs::chop::substr("über das Floß.", 9, 4), "Floß");
    assert_eq!(voca_rs::chop::substr("e\u{0301}", 1, 0), "\u{0301}");
}
#[test]
fn substr_me() {
    assert_eq!("helicopter".substr(1, 0), "elicopter");
}
#[test]
fn substring() {
    assert_eq!(voca_rs::chop::substring("", 0, 0), "");
    assert_eq!(voca_rs::chop::substring("a", 0, 0), "a");
    assert_eq!(voca_rs::chop::substring("a", 10, 0), "");
    assert_eq!(voca_rs::chop::substring("a", 0, 1), "a");
    assert_eq!(voca_rs::chop::substring("a", 0, 10), "a");
    assert_eq!(voca_rs::chop::substring("ab", 1, 0), "b");
    assert_eq!(voca_rs::chop::substring("abcd", 3, 1), "");
    assert_eq!(voca_rs::chop::substring("abcd", 3, 3), "");
    assert_eq!(voca_rs::chop::substring("abcd", 3, 4), "d");
    assert_eq!(voca_rs::chop::substring("abcd", 3, 10), "d");
    assert_eq!(voca_rs::chop::substring("abcd", 30, 1), "");
    assert_eq!(voca_rs::chop::substring("ab", 2, 0), "");
    assert_eq!(voca_rs::chop::substring("ab", 3, 0), "");
    assert_eq!(voca_rs::chop::substring("ab", 20, 20), "");
    assert_eq!(voca_rs::chop::substring("a", 20, 10), "");
    assert_eq!(voca_rs::chop::substring("helicopter", 1, 0), "elicopter");
    assert_eq!(voca_rs::chop::substring("b\u{0142}\u{0105}d", 2, 4), "ąd");
    assert_eq!(voca_rs::chop::substring("über das Floß.", 0, 1), "ü");
    assert_eq!(voca_rs::chop::substring("e\u{0301}", 1, 0), "\u{0301}");
}
#[test]
fn substring_me() {
    assert_eq!("helicopter".substring(1, 0), "elicopter");
}
#[test]
fn truncate() {
    assert_eq!(voca_rs::chop::truncate("", 0, ""), "");
    assert_eq!(voca_rs::chop::truncate("a", 1, ""), "a");
    assert_eq!(voca_rs::chop::truncate("a", 2, ""), "a");
    assert_eq!(voca_rs::chop::truncate("a", 3, ""), "a");
    assert_eq!(voca_rs::chop::truncate("a", 4, ""), "a");
    assert_eq!(voca_rs::chop::truncate("a", 5, ""), "a");
    assert_eq!(voca_rs::chop::truncate("a", 10, ""), "a");
    assert_eq!(
        voca_rs::chop::truncate("Once upon a time", 7, ""),
        "Once..."
    );
    assert_eq!(
        voca_rs::chop::truncate("Die Schildkröte fliegt über das Floß.", 28, "(...)"),
        "Die Schildkröte fliegt (...)"
    );
    assert_eq!(voca_rs::chop::truncate("Once upon", 10, ""), "Once upon");
    assert_eq!(
        voca_rs::chop::truncate("Как слышно, приём!", 13, ""),
        "Как слышно..."
    );
}
#[test]
fn truncate_me() {
    assert_eq!("Once upon a time".truncate(7, ""), "Once...");
}
#[test]
fn max() {
    assert_eq!(voca_rs::chop::max(""), "");
    assert_eq!(voca_rs::chop::max("rain"), "r");
    assert_eq!(voca_rs::chop::max("cafe\u{0301}"), "\u{0301}");
    assert_eq!(voca_rs::chop::max("café"), "\u{0301}");
    assert_eq!(voca_rs::chop::max("b\u{0142}\u{0105}d"), "ł");
    assert_eq!(voca_rs::chop::max("über das Floß."), "ü");
    assert_eq!(voca_rs::chop::max("a̐éö̲"), "\u{332}");
}
#[test]
fn max_me() {
    assert_eq!("rain".max_code_point(), "r");
}
#[test]
fn min() {
    assert_eq!(voca_rs::chop::min(""), "");
    assert_eq!(voca_rs::chop::min("rain"), "a");
    assert_eq!(voca_rs::chop::min("cafe\u{0301}"), "a");
    assert_eq!(voca_rs::chop::min("café"), "a");
    assert_eq!(voca_rs::chop::min("b\u{0142}\u{0105}d"), "b");
    assert_eq!(voca_rs::chop::min("Über das Floß."), " ");
    assert_eq!(voca_rs::chop::min("a̐éö̲"), "a");
}
#[test]
fn min_me() {
    assert_eq!("rain".min_code_point(), "a");
}
