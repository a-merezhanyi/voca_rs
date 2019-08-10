//! voca_rs::count testing

#[test]
fn count() {
    assert_eq!(voca_rs::count::count(""), 0);
    assert_eq!(voca_rs::count::count("rain"), 4);
    assert_eq!(voca_rs::count::count("b\u{0142}\u{0105}d"), 4);
    assert_eq!(voca_rs::count::count("Die Schildkröte fliegt über das Floß."), 37);
    assert_eq!(voca_rs::count::count("Как слышно, приём!"), 18);
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
    assert_eq!(voca_rs::count::count_substrings("Cafe\u{0301} del Mar", "Café"), 1);
    assert_eq!(
        voca_rs::count::count_substrings("Cafe\u{0301} del Mar Café del Mar cafe\u{0301}", "Café"),
        2
    );
    assert_eq!(voca_rs::count::count_substrings("every dog has its day", "cat"), 0);
}
#[test]
fn words() {
    assert_eq!(voca_rs::count::count_words("", ""), 0);
    assert_eq!(voca_rs::count::count_words("ab c", ""), 2);
    assert_eq!(voca_rs::count::count_words("Gravity - can cross dimensions!", ""), 4);
    assert_eq!(voca_rs::count::count_words("GravityCanCrossDimensions", ""), 4);
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
