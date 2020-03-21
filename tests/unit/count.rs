//! voca_rs::count testing
use voca_rs::Voca;

#[test]
fn count() {
    assert_eq!(voca_rs::count::count(""), 0);
    assert_eq!(voca_rs::count::count("rain"), 4);
    assert_eq!(voca_rs::count::count("b\u{0142}\u{0105}d"), 4);
    assert_eq!(
        voca_rs::count::count("Die Schildkröte fliegt über das Floß."),
        37
    );
    assert_eq!(voca_rs::count::count("Как слышно, приём!"), 18);
}
#[test]
fn _count() {
    assert_eq!("rain"._count(), 4);
}
#[test]
fn graphemes() {
    assert_eq!(voca_rs::count::count_graphemes(""), 0);
    assert_eq!(voca_rs::count::count_graphemes("rain"), 4);
    assert_eq!(voca_rs::count::count_graphemes("b\u{0142}\u{0105}d"), 4);
    assert_eq!(voca_rs::count::count_graphemes("błąd"), 4);
    assert_eq!(voca_rs::count::count_graphemes("a̐éö̲"), 3);
    assert_eq!(
        voca_rs::count::count_graphemes("Die Schildkröte fliegt über das Floß."),
        37
    );
    assert_eq!(voca_rs::count::count_graphemes("cafe\u{0301}"), 4);
}
#[test]
fn _graphemes() {
    assert_eq!("rain"._count_graphemes(), 4);
}
#[test]
fn substrings() {
    assert_eq!(voca_rs::count::count_substrings("", ""), 0);
    assert_eq!(voca_rs::count::count_substrings("******", "*"), 6);
    assert_eq!(voca_rs::count::count_substrings("******", "**"), 3);
    assert_eq!(voca_rs::count::count_substrings("******", "**-"), 0);
    assert_eq!(voca_rs::count::count_substrings("abc", ""), 0);
    assert_eq!(voca_rs::count::count_substrings("rain", "rain"), 1);
    assert_eq!(
        voca_rs::count::count_substrings("Die Schildkröte fliegt über das Floß.", "über"),
        1
    );
    assert_eq!(
        voca_rs::count::count_substrings("bad boys, bad boys whatcha gonna do?", "boys"),
        2
    );
    assert_eq!(
        voca_rs::count::count_substrings("Cafe\u{0301} del Mar", "Café"),
        1
    );
    assert_eq!(
        voca_rs::count::count_substrings("Cafe\u{0301} del Mar Café del Mar cafe\u{0301}", "Café"),
        2
    );
    assert_eq!(
        voca_rs::count::count_substrings("every dog has its day", "cat"),
        0
    );
}
#[test]
fn _substrings() {
    assert_eq!("******"._count_substrings("*"), 6);
}

#[test]
fn count_where() {
    assert_eq!(
        voca_rs::count::count_where("hola!", voca_rs::query::is_alpha),
        4
    );
    assert_eq!(
        voca_rs::count::count_where("2022", |s: &str| -> bool { s == "2" }),
        3
    );
    assert_eq!(voca_rs::count::count_where("", voca_rs::query::is_alpha), 0);
    assert_eq!(
        voca_rs::count::count_where("abc", voca_rs::query::is_alpha),
        3
    );
    assert_eq!(
        voca_rs::count::count_where("africa654", voca_rs::query::is_alpha),
        6
    );
    assert_eq!(
        voca_rs::count::count_where("790", voca_rs::query::is_alpha),
        0
    );
    assert_eq!(
        voca_rs::count::count_where("790", voca_rs::query::is_alphadigit),
        3
    );
    assert_eq!(
        voca_rs::count::count_where(voca_rs::utils::PRINTABLE, voca_rs::query::is_digit),
        10
    );
    assert_eq!(
        voca_rs::count::count_where("****--**--**", |s: &str| -> bool { s == "*" }),
        8
    );
    assert_eq!(
        voca_rs::count::count_where("****--**--**", |_s: &str| -> bool { false }),
        0
    );
}
#[test]
fn _count_where() {
    assert_eq!("hola!"._count_where(voca_rs::query::is_alpha), 4);
}

#[test]
fn words() {
    assert_eq!(voca_rs::count::count_words("", ""), 0);
    assert_eq!(voca_rs::count::count_words("ab c", ""), 2);
    assert_eq!(
        voca_rs::count::count_words("Gravity - can cross dimensions!", ""),
        4
    );
    assert_eq!(
        voca_rs::count::count_words("GravityCanCrossDimensions", ""),
        4
    );
    assert_eq!(
        voca_rs::count::count_words("Cafe\u{0301}-del-Mar-andBossaNova1", "-"),
        4
    );
    assert_eq!(voca_rs::count::count_words("Język /polski wywodzi się z` języka` praindoeuropejskiego za**pośrednictwem+języka-prasłowiańskiego.", ""), 11);
    assert_eq!(
        voca_rs::count::count_words("Στις--αρχές** (του) 21ου, αιώνα!'", ""),
        5
    );
    assert_eq!(
        voca_rs::count::count_words("Гравитация-Притягивает-ВСЕ!!", "-"),
        3
    );
}
#[test]
fn _words() {
    assert_eq!("Gravity - can cross dimensions!"._count_words(""), 4);
}
