//! voca_rs::index testing
use voca_rs::Voca;

#[test]
fn index_of() {
    assert_eq!(voca_rs::index::index_of("", "", 0), 0);
    assert_eq!(voca_rs::index::index_of("rain", "r", 0), 0);
    assert_eq!(voca_rs::index::index_of("rain", "a", 0), 1);
    assert_eq!(voca_rs::index::index_of("rain", "n", 3), 0);
    assert_eq!(voca_rs::index::index_of("rain", "a", 10), -1);
    assert_eq!(voca_rs::index::index_of("Rain, dear rain", "ear", 0), 7);
    assert_eq!(voca_rs::index::index_of("Rain, dear rain", "ain", 0), 1);
    assert_eq!(voca_rs::index::index_of("rain", "z", 0), -1);
    assert_eq!(voca_rs::index::index_of("b\u{0142}\u{0105}d", "ą", 0), 2);
    assert_eq!(
        voca_rs::index::index_of("Zażółć gęślą jaźń", "gęślą", 0),
        7
    );
    assert_eq!(
        voca_rs::index::index_of(
            "Die Schildkröte fliegt über das Floß.",
            "Schildkröte",
            4
        ),
        0
    );
    assert_eq!(
        voca_rs::index::index_of("Как слышно, приём!", "слышно", 0),
        4
    );
}
#[test]
fn index_of_me() {
    assert_eq!("Rain, dear rain".index_of("ear", 0), 7);
}
#[test]
fn index_all() {
    assert_eq!(voca_rs::index::index_all("", "", 0), []);
    assert_eq!(voca_rs::index::index_all("rain", "r", 0), [0]);
    assert_eq!(voca_rs::index::index_all("rain", "a", 0), [1]);
    assert_eq!(voca_rs::index::index_all("rain", "n", 3), [0]);
    assert_eq!(voca_rs::index::index_all("rain", "a", 10), []);
    assert_eq!(
        voca_rs::index::index_all("Rain, dear rain", "ear", 0),
        [1, 7, 8, 9, 11, 12]
    );
    assert_eq!(
        voca_rs::index::index_all("Rain, dear rain", "ain", 0),
        [1, 2, 3, 8, 12, 13, 14]
    );
    assert_eq!(voca_rs::index::index_all("rain", "z", 0), []);
    assert_eq!(voca_rs::index::index_all("b\u{0142}\u{0105}d", "ą", 0), [2]);
    assert_eq!(
        voca_rs::index::index_all("Zażółć gęślą jaźń", "aęą", 0),
        [1, 8, 11, 14]
    );
    assert_eq!(
        voca_rs::index::index_all("Die Schildkröte fliegt über das Floß.", "iöß", 4),
        [3, 8, 14, 31]
    );
    assert_eq!(
        voca_rs::index::index_all("Как меня слышно, приём!", "н", 0),
        [6, 13]
    );
}
#[test]
fn index_all_me() {
    assert_eq!(
        "Rain, dear rain".index_all("ear", 0),
        [1, 7, 8, 9, 11, 12]
    );
}
#[test]
fn last_index_of() {
    assert_eq!(voca_rs::index::last_index_of("", "", 0), 0);
    assert_eq!(voca_rs::index::last_index_of("rain", "r", 0), 0);
    assert_eq!(voca_rs::index::last_index_of("rain", "a", 0), 1);
    assert_eq!(voca_rs::index::last_index_of("rain", "n", 3), 0);
    assert_eq!(voca_rs::index::last_index_of("rain", "a", 10), -1);
    assert_eq!(voca_rs::index::last_index_of("Rain, dear rain", "rain", 0), 11);
    assert_eq!(voca_rs::index::last_index_of("Rain, dear rain", "ain", 0), 12);
    assert_eq!(voca_rs::index::last_index_of("rain", "z", 0), -1);
    assert_eq!(voca_rs::index::last_index_of("b\u{0142}\u{0105}d", "ą", 0), 2);
    assert_eq!(
        voca_rs::index::last_index_of("Zażółć gęślą jaźń", "gęślą", 0),
        7
    );
    assert_eq!(
        voca_rs::index::last_index_of(
            "Die Schildkröte fliegt über das Floß.",
            "Schildkröte",
            4
        ),
        0
    );
    assert_eq!(
        voca_rs::index::last_index_of("Как слышно, приём!", "слышно", 0),
        4
    );
}
#[test]
fn last_index_of_me() {
    assert_eq!("Rain, dear rain".last_index_of("rain", 0), 11);
}
#[test]
fn search() {
    assert_eq!(voca_rs::index::search("", "", 0), 0);
    assert_eq!(voca_rs::index::search("morning", "rn", 0), 2);
    assert_eq!(voca_rs::index::search("evening", r"\d", 0), -1);
    assert_eq!(voca_rs::index::search("we have a mission", "mission", 0), 10);
    assert_eq!(voca_rs::index::search("we have a mission", "a", 0), 4);
    assert_eq!(voca_rs::index::search("we have a mission", r"\s", 0), 2);
    assert_eq!(voca_rs::index::search("we have a mission", "", 0), 0);
    assert_eq!(voca_rs::index::search("we have a mission", "a", 6), 8);
    assert_eq!(voca_rs::index::search("we have a mission", "12", 0), -1);
    assert_eq!(voca_rs::index::search("we have a mission", r"\s^", 0), -1);
    assert_eq!(voca_rs::index::search("we have a mission", "we", 3), -1);
    assert_eq!(voca_rs::index::search("we have a mission", "mission", 100), -1);
    assert_eq!(
        voca_rs::index::search("Zażółć gęślą jaźń", "gęślą", 6),
        11
    );
    assert_eq!(
        voca_rs::index::search(
            "Die Schildkröte fliegt über das Floß.",
            "Schildkröte",
            4
        ),
        4
    );
    assert_eq!(
        voca_rs::index::search("Как слышно, приём!", "слышно", 0),
        7
    );
    assert_eq!(
        // [^] is not a valid regex
        voca_rs::index::search("Zażółć gęślą jaźń", "[^]", 0),
        -1
    );
}
#[test]
fn search_me() {
    assert_eq!("we have a mission".search("mission", 0), 10);
}