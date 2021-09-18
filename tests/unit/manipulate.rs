//! voca_rs::manipulate testing
use voca_rs::Voca;

#[test]
fn expand_tabs() {
    assert_eq!(voca_rs::manipulate::expand_tabs("", 0), "");
    assert_eq!(
        voca_rs::manipulate::expand_tabs("This is\tgood", 4),
        "This is    good"
    );
    assert_eq!(
        voca_rs::manipulate::expand_tabs("no\tspaces", 0),
        "nospaces"
    );
    assert_eq!(
        voca_rs::manipulate::expand_tabs("line1\n\tline2\n\t\tline3", 2),
        "line1\n  line2\n    line3"
    );
    assert_eq!(
        voca_rs::manipulate::expand_tabs("Zaż\tółć\t!", 4),
        "Zaż    ółć    !"
    );
    assert_eq!(
        voca_rs::manipulate::expand_tabs("über\tdas\tFloß", 1),
        "über das Floß"
    );
    assert_eq!(
        voca_rs::manipulate::expand_tabs("caffé\tlatté", 1),
        "caffé latté"
    );
}
#[test]
fn _expand_tabs() {
    assert_eq!("This is\tgood"._expand_tabs(4), "This is    good");
}
#[test]
fn expand_spaces() {
    assert_eq!(voca_rs::manipulate::expand_spaces("", 0), "");
    assert_eq!(
        voca_rs::manipulate::expand_spaces("как же  хорошо", 0),
        "как же  хорошо"
    );
    assert_eq!(
        voca_rs::manipulate::expand_spaces("This  is  good", 2),
        "This\tis\tgood"
    );
    assert_eq!(
        voca_rs::manipulate::expand_spaces("Café del  Mar", 2),
        "Café del\tMar"
    );
    assert_eq!(
        voca_rs::manipulate::expand_spaces("line1\n  line2\n    line3", 2),
        "line1\n\tline2\n\t\tline3"
    );
    assert_eq!(
        voca_rs::manipulate::expand_spaces("Zaż    ółć    !", 4),
        "Zaż\tółć\t!"
    );
    assert_eq!(
        voca_rs::manipulate::expand_spaces("über das Floß", 1),
        "über\tdas\tFloß"
    );
    assert_eq!(
        voca_rs::manipulate::expand_spaces("caffé latté", 1),
        "caffé\tlatté"
    );
}
#[test]
fn _expand_spaces() {
    assert_eq!("This  is  good"._expand_spaces(2), "This\tis\tgood");
}
#[test]
fn insert() {
    assert_eq!(voca_rs::manipulate::insert("", "", 0), "");
    assert_eq!(voca_rs::manipulate::insert("abc", "", 0), "abc");
    assert_eq!(voca_rs::manipulate::insert("abc", "-", 0), "-abc");
    assert_eq!(voca_rs::manipulate::insert("abc", "-", 1), "a-bc");
    assert_eq!(voca_rs::manipulate::insert("abc", "-", 2), "ab-c");
    assert_eq!(voca_rs::manipulate::insert("abc", "-", 3), "abc-");
    assert_eq!(voca_rs::manipulate::insert("abc", "-", 4), "abc-");
    assert_eq!(voca_rs::manipulate::insert("abc", "-", 10), "abc-");
    assert_eq!(voca_rs::manipulate::insert("abc abc", ",", 3), "abc, abc");
    assert_eq!(voca_rs::manipulate::insert("Zażółć", "-!-", 3), "Zaż-!-ółć");
    assert_eq!(
        voca_rs::manipulate::insert("über\tdas\tFloß", "~~~", 6),
        "über\td~~~as\tFloß"
    );
    assert_eq!(voca_rs::manipulate::insert("приём", "!", 5), "приём!");
}
#[test]
fn _insert() {
    assert_eq!("abc"._insert("-", 0), "-abc");
}
#[test]
fn latinise() {
    assert_eq!(voca_rs::manipulate::latinise(""), "");
    assert_eq!(voca_rs::manipulate::latinise("cafe\u{0301}"), "cafe");
    assert_eq!(
        voca_rs::manipulate::latinise("août décembre"),
        "aout decembre"
    );
    assert_eq!(
        voca_rs::manipulate::latinise("как прекрасен этот мир"),
        "kak prekrasen etot mir"
    );
    assert_eq!(voca_rs::manipulate::latinise("Æneid étude"), "AEneid etude");
    assert_eq!(voca_rs::manipulate::latinise("北亰"), "Bei Jing ");
    assert_eq!(voca_rs::manipulate::latinise("ᔕᓇᓇ"), "shanana");
    assert_eq!(voca_rs::manipulate::latinise("げんまい茶"), "genmaiCha ");
    assert_eq!(
        voca_rs::manipulate::latinise("Zażółć gęślą jaźń"),
        "Zazolc gesla jazn"
    );
    assert_eq!(
        voca_rs::manipulate::latinise("Die Schildkröte fliegt über das Floß."),
        "Die Schildkrote fliegt uber das Floss."
    );
    assert_eq!(
        voca_rs::manipulate::latinise("Există peste 13.800 de localități în România"),
        "Exista peste 13.800 de localitati in Romania"
    );
}
#[test]
fn _latinise() {
    assert_eq!("cafe\u{0301}"._latinise(), "cafe");
}
#[test]
fn slugify() {
    assert_eq!(voca_rs::manipulate::slugify(""), "");
    assert_eq!(voca_rs::manipulate::slugify("\n\n\n\n   ***\t\t"), "");
    assert_eq!(voca_rs::manipulate::slugify("****"), "");
    assert_eq!(
        voca_rs::manipulate::slugify("Italian cappuccino drink"),
        "italian-cappuccino-drink"
    );
    assert_eq!(voca_rs::manipulate::slugify("caffé latté"), "caffe-latte");
    assert_eq!(
        voca_rs::manipulate::slugify("хорошая погода"),
        "khoroshaia-pogoda"
    );
    assert_eq!(
        voca_rs::manipulate::slugify("Хорошая статья: 'XMLHttpRequest 101 Course' \\!/"),
        "khoroshaia-statia-xmlhttprequest-101-course"
    );
    assert_eq!(
        voca_rs::manipulate::slugify("/home/dmitri/projects/voca"),
        "home-dmitri-projects-voca"
    );

    assert_eq!(voca_rs::manipulate::slugify("cafe\u{0301}"), "cafe");
    assert_eq!(
        voca_rs::manipulate::slugify("août décembre"),
        "aout-decembre"
    );
    assert_eq!(
        voca_rs::manipulate::slugify("как прекрасен этот мир"),
        "kak-prekrasen-etot-mir"
    );
    assert_eq!(voca_rs::manipulate::slugify("Æneid étude"), "aeneid-etude");
    assert_eq!(voca_rs::manipulate::slugify("北亰"), "bei-jing");
    assert_eq!(voca_rs::manipulate::slugify("ᔕᓇᓇ ᔕᓇᓇ"), "shanana-shanana");
    assert_eq!(
        voca_rs::manipulate::slugify("げんまい茶 げん"),
        "genmaicha-gen"
    );
    assert_eq!(
        voca_rs::manipulate::slugify("Zażółć gęślą jaźń"),
        "zazolc-gesla-jazn"
    );
    assert_eq!(
        voca_rs::manipulate::slugify("Die Schildkröte fliegt über das Floß."),
        "die-schildkrote-fliegt-uber-das-floss"
    );
}
#[test]
fn _slugify() {
    assert_eq!(
        "Italian cappuccino drink"._slugify(),
        "italian-cappuccino-drink"
    );
}
#[test]
fn pad() {
    assert_eq!(voca_rs::manipulate::pad("", 0, ""), "");
    assert_eq!(voca_rs::manipulate::pad("abc", 1, ""), "abc");
    assert_eq!(voca_rs::manipulate::pad("abc", 5, ""), " abc ");
    assert_eq!(voca_rs::manipulate::pad("dog", 5, ""), " dog ");
    assert_eq!(voca_rs::manipulate::pad("bird", 6, "-"), "-bird-");
    assert_eq!(voca_rs::manipulate::pad("bird", 6, "-="), "-bird-");
    assert_eq!(voca_rs::manipulate::pad("bird", 6, "-=:"), "-bird-");
    assert_eq!(voca_rs::manipulate::pad("bird", 6, "-=:="), "-bird-");
    assert_eq!(voca_rs::manipulate::pad("bird", 7, "-=:=-"), "-bird-=");
    assert_eq!(voca_rs::manipulate::pad("bird", 8, "-=:=-"), "-=bird-=");
    assert_eq!(voca_rs::manipulate::pad("bird", 9, "-=:=-"), "-=bird-=:");
    assert_eq!(
        voca_rs::manipulate::pad("Café del Mar", 15, ""),
        " Café del Mar  "
    );
    assert_eq!(
        voca_rs::manipulate::pad("Café del Mar", 15, "-="),
        "-Café del Mar-="
    );
    assert_eq!(
        voca_rs::manipulate::pad("Zażółć gęślą jaźń", 25, ".:"),
        ".:.:Zażółć gęślą jaźń.:.:"
    );
    assert_eq!(
        voca_rs::manipulate::pad("Die Schildkröte fliegt", 29, "~-"),
        "~-~Die Schildkröte fliegt~-~-"
    );
    assert_eq!(
        voca_rs::manipulate::pad("Алё! Приём", 11, ""),
        "Алё! Приём "
    );
}
#[test]
fn _pad() {
    assert_eq!("dog"._pad(5, ""), " dog ");
}
#[test]
fn pad_left() {
    assert_eq!(voca_rs::manipulate::pad_left("", 0, ""), "");
    assert_eq!(voca_rs::manipulate::pad_left("abc", 1, ""), "abc");
    assert_eq!(voca_rs::manipulate::pad_left("abc", 5, ""), "  abc");
    assert_eq!(voca_rs::manipulate::pad_left("dog", 5, ""), "  dog");
    assert_eq!(voca_rs::manipulate::pad_left("bird", 6, "-"), "--bird");
    assert_eq!(voca_rs::manipulate::pad_left("bird", 6, "-="), "-=bird");
    assert_eq!(voca_rs::manipulate::pad_left("bird", 6, "-=:"), "-=bird");
    assert_eq!(voca_rs::manipulate::pad_left("bird", 6, "-=:="), "-=bird");
    assert_eq!(voca_rs::manipulate::pad_left("bird", 6, "-=:=-"), "-=bird");
    assert_eq!(
        voca_rs::manipulate::pad_left("Café del Mar", 15, ""),
        "   Café del Mar"
    );
    assert_eq!(
        voca_rs::manipulate::pad_left("Café del Mar", 15, "-="),
        "-=-Café del Mar"
    );
    assert_eq!(
        voca_rs::manipulate::pad_left("Zażółć gęślą jaźń", 25, ".:"),
        ".:.:.:.:Zażółć gęślą jaźń"
    );
    assert_eq!(
        voca_rs::manipulate::pad_left("Die Schildkröte fliegt", 29, "~-"),
        "~-~-~-~Die Schildkröte fliegt"
    );
    assert_eq!(
        voca_rs::manipulate::pad_left("Алё! Приём", 11, ""),
        " Алё! Приём"
    );
}
#[test]
fn _pad_left() {
    assert_eq!("dog"._pad_left(5, ""), "  dog");
}
#[test]
fn pad_right() {
    assert_eq!(voca_rs::manipulate::pad_right("", 0, ""), "");
    assert_eq!(voca_rs::manipulate::pad_right("abc", 1, ""), "abc");
    assert_eq!(voca_rs::manipulate::pad_right("abc", 5, ""), "abc  ");
    assert_eq!(voca_rs::manipulate::pad_right("dog", 5, ""), "dog  ");
    assert_eq!(voca_rs::manipulate::pad_right("bird", 6, "-"), "bird--");
    assert_eq!(voca_rs::manipulate::pad_right("bird", 6, "-="), "bird-=");
    assert_eq!(voca_rs::manipulate::pad_right("bird", 6, "-=:"), "bird-=");
    assert_eq!(voca_rs::manipulate::pad_right("bird", 6, "-=:="), "bird-=");
    assert_eq!(voca_rs::manipulate::pad_right("bird", 6, "-=:=-"), "bird-=");
    assert_eq!(
        voca_rs::manipulate::pad_right("Café del Mar", 15, ""),
        "Café del Mar   "
    );
    assert_eq!(
        voca_rs::manipulate::pad_right("Café del Mar", 15, "-="),
        "Café del Mar-=-"
    );
    assert_eq!(
        voca_rs::manipulate::pad_right("Zażółć gęślą jaźń", 25, ".:"),
        "Zażółć gęślą jaźń.:.:.:.:"
    );
    assert_eq!(
        voca_rs::manipulate::pad_right("Die Schildkröte fliegt", 29, "~-"),
        "Die Schildkröte fliegt~-~-~-~"
    );
    assert_eq!(
        voca_rs::manipulate::pad_right("Алё! Приём", 11, ""),
        "Алё! Приём "
    );
}
#[test]
fn _pad_right() {
    assert_eq!("dog"._pad_right(5, ""), "dog  ");
}
#[test]
fn repeat() {
    assert_eq!(voca_rs::manipulate::repeat("", 1), "");
    assert_eq!(voca_rs::manipulate::repeat("w", 3), "www");
    assert_eq!(voca_rs::manipulate::repeat("www", 0), "");
    assert_eq!(voca_rs::manipulate::repeat("abc", 3), "abcabcabc");
    assert_eq!(
        voca_rs::manipulate::repeat("abc abc", 3),
        "abc abcabc abcabc abc"
    );
}
#[test]
fn _repeat() {
    assert_eq!("abc"._repeat(3), "abcabcabc");
}
#[test]
fn replace() {
    assert_eq!(voca_rs::manipulate::replace("", "", ""), "");
    assert_eq!(voca_rs::manipulate::replace("abc", "", ""), "abc");
    assert_eq!(voca_rs::manipulate::replace("swan", "wa", "u"), "sun");
    assert_eq!(voca_rs::manipulate::replace("swan", "b", "a"), "swan");
    assert_eq!(
        voca_rs::manipulate::replace("domestic duck", "d", "D"),
        "Domestic duck"
    );
    assert_eq!(voca_rs::manipulate::replace("Zażółć", "ó", "o"), "Zażołć");
    assert_eq!(voca_rs::manipulate::replace("café", "é", "e"), "cafe");
    assert_eq!(
        voca_rs::manipulate::replace("Café del Mar cafe\u{0301}", "é", "e"),
        "Cafe del Mar café"
    );
    assert_eq!(
        voca_rs::manipulate::replace("Cafe\u{0301} del Mar Café del Mar cafe\u{0301}", "é", "e"),
        "Cafe del Mar Café del Mar cafe\u{0301}"
    );
    assert_eq!(
        voca_rs::manipulate::replace("Zażółć gęślą jaźń w gęślą oraz jaźń", "jaźń", "***"),
        "Zażółć gęślą *** w gęślą oraz jaźń"
    );
}
#[test]
fn _replace() {
    assert_eq!("swan"._replace("wa", "u"), "sun");
}
#[test]
fn replace_all() {
    assert_eq!(voca_rs::manipulate::replace_all("", "", ""), "");
    assert_eq!(voca_rs::manipulate::replace_all("abc", "", ""), "abc");
    assert_eq!(voca_rs::manipulate::replace_all("swan", "wa", "u"), "sun");
    assert_eq!(voca_rs::manipulate::replace_all("swan", "b", "a"), "swan");
    assert_eq!(
        voca_rs::manipulate::replace_all("Café del Mar café", "é", "e"),
        "Cafe del Mar cafe"
    );
    assert_eq!(
        voca_rs::manipulate::replace_all("domestic duck", "d", "D"),
        "Domestic Duck"
    );
    assert_eq!(
        voca_rs::manipulate::replace_all("Zażółć", "ó", "o"),
        "Zażołć"
    );
    assert_eq!(voca_rs::manipulate::replace_all("café", "é", "e"), "cafe");
    assert_eq!(
        voca_rs::manipulate::replace_all("Café del Mar cafe\u{0301}", "é", "e"),
        "Cafe del Mar cafe"
    );
    assert_eq!(
        voca_rs::manipulate::replace_all(
            "Cafe\u{0301} del Mar Café del Mar cafe\u{0301}",
            "é",
            "e"
        ),
        "Cafe del Mar Cafe del Mar cafe"
    );
    assert_eq!(
        voca_rs::manipulate::replace_all("Zażółć gęślą jaźń w gęślą oraz jaźń", "jaźń", "***"),
        "Zażółć gęślą *** w gęślą oraz ***"
    );
}
#[test]
fn _replace_all() {
    assert_eq!("swan"._replace_all("wa", "u"), "sun");
}
#[test]
fn reverse() {
    assert_eq!(voca_rs::manipulate::reverse(""), "");
    assert_eq!(voca_rs::manipulate::reverse("abc"), "cba");
    assert_eq!(voca_rs::manipulate::reverse("abc abc"), "cba cba");
    assert_eq!(voca_rs::manipulate::reverse("Zażółć"), "ćłóżaZ");
    assert_eq!(
        voca_rs::manipulate::reverse("über\tdas\tFloß"),
        "ßolF\tsad\trebü"
    );
    assert_eq!(voca_rs::manipulate::reverse("приём!"), "!мёирп");
}
#[test]
fn _reverse() {
    assert_eq!("abc"._reverse(), "cba");
}
#[test]
fn reverse_grapheme() {
    assert_eq!(voca_rs::manipulate::reverse_grapheme(""), "");
    assert_eq!(voca_rs::manipulate::reverse_grapheme("abc"), "cba");
    assert_eq!(voca_rs::manipulate::reverse_grapheme("abc abc"), "cba cba");
    assert_eq!(voca_rs::manipulate::reverse_grapheme("Zażółć"), "ćłóżaZ");
    assert_eq!(
        voca_rs::manipulate::reverse_grapheme("über\tdas\tFloß"),
        "ßolF\tsad\trebü"
    );
    assert_eq!(voca_rs::manipulate::reverse_grapheme("приём!"), "!мёирп");
    assert_eq!(voca_rs::manipulate::reverse_grapheme("café"), "éfac");
    assert_eq!(voca_rs::manipulate::reverse_grapheme("a̐éö̲"), "ö̲éa̐");
}
#[test]
fn _reverse_grapheme() {
    assert_eq!("a̐éö̲"._reverse_grapheme(), "ö̲éa̐");
}
#[test]
fn splice() {
    assert_eq!(voca_rs::manipulate::splice("", 0, 0, ""), "");
    assert_eq!(voca_rs::manipulate::splice("test", 0, 0, ""), "test");
    assert_eq!(voca_rs::manipulate::splice("test", 0, 0, "-"), "-test");
    assert_eq!(voca_rs::manipulate::splice("test", 1, 0, "-"), "t-est");
    assert_eq!(voca_rs::manipulate::splice("test", 1, 1, "-"), "t-st");
    assert_eq!(voca_rs::manipulate::splice("test", 3, 0, "-"), "tes-t");
    assert_eq!(voca_rs::manipulate::splice("test", 4, 0, "-"), "test-");
    assert_eq!(voca_rs::manipulate::splice("test", 4, 1, "-"), "test-");
    assert_eq!(voca_rs::manipulate::splice("test", 4, 10, "-"), "test-");
    assert_eq!(voca_rs::manipulate::splice("test", -1, 0, "="), "tes=t");
    assert_eq!(voca_rs::manipulate::splice("test", -2, 1, "="), "te=t");
    assert_eq!(voca_rs::manipulate::splice("test", -10, 0, "."), ".test");
    assert_eq!(voca_rs::manipulate::splice("test", 100, 0, "."), "test.");
    assert_eq!(
        voca_rs::manipulate::splice("Zażółć", 6, 0, " gęślą jaźń"),
        "Zażółć gęślą jaźń"
    );
    assert_eq!(
        voca_rs::manipulate::splice("Zażółć gęślą", 6, 6, " jaźń"),
        "Zażółć jaźń"
    );
    assert_eq!(
        voca_rs::manipulate::splice("to jest błąd", 0, 7, "mój"),
        "mój błąd"
    );
    assert_eq!(
        voca_rs::manipulate::splice("это моя ошибка", 4, 3, "не"),
        "это не ошибка"
    );
    assert_eq!(
        voca_rs::manipulate::splice("Die Schildkröte fliegt.", -7, 0, "und Kröte "),
        "Die Schildkröte und Kröte fliegt."
    );
    assert_eq!(
        voca_rs::manipulate::splice("Привет", 6, 0, ", Ёлка!"),
        "Привет, Ёлка!"
    );
}
#[test]
fn _splice() {
    assert_eq!("test"._splice(1, 0, "-"), "t-est");
}
#[test]
fn trim() {
    assert_eq!(
        voca_rs::manipulate::trim("   The world - is yours\t   ", ""),
        "The world - is yours"
    );
    assert_eq!(
        voca_rs::manipulate::trim("--Zażółć gęślą jaźń---", "-"),
        "Zażółć gęślą jaźń"
    );
    assert_eq!(voca_rs::manipulate::trim("-~-Earth--~", "-~"), "Earth");
    assert_eq!(
        voca_rs::manipulate::trim("++--Die Schildkröte fliegt über das Floß.++", "+"),
        "--Die Schildkröte fliegt über das Floß."
    );
    assert_eq!(
        voca_rs::manipulate::trim("ё1ё2ёКак слышно, приём!ё1ё", "ёё12"),
        "Как слышно, приём!"
    );
    assert_eq!(voca_rs::manipulate::trim("", ""), "");
}
#[test]
fn _trim() {
    assert_eq!(
        "   The world - is yours\t   "._trim(""),
        "The world - is yours"
    );
}
#[test]
fn trim_left() {
    assert_eq!(
        voca_rs::manipulate::trim_left("   The world - is yours\t   ", ""),
        "The world - is yours\t   "
    );
    assert_eq!(
        voca_rs::manipulate::trim_left("--Zażółć gęślą jaźń---", "-"),
        "Zażółć gęślą jaźń---"
    );
    assert_eq!(
        voca_rs::manipulate::trim_left("-~-Earth--~", "-~"),
        "Earth--~"
    );
    assert_eq!(
        voca_rs::manipulate::trim_left("++--Die Schildkröte fliegt über das Floß.++", "+"),
        "--Die Schildkröte fliegt über das Floß.++"
    );
    assert_eq!(
        voca_rs::manipulate::trim_left("ё1ё2ёКак слышно, приём!ё1ё", "ёё12"),
        "Как слышно, приём!ё1ё"
    );
    assert_eq!(voca_rs::manipulate::trim_left("", ""), "");
}
#[test]
fn _trim_left() {
    assert_eq!(
        "   The world - is yours\t   "._trim_left(""),
        "The world - is yours\t   "
    );
}
#[test]
fn trim_right() {
    assert_eq!(
        voca_rs::manipulate::trim_right("   The world - is yours\t   ", ""),
        "   The world - is yours"
    );
    assert_eq!(
        voca_rs::manipulate::trim_right("--Zażółć gęślą jaźń---", "-"),
        "--Zażółć gęślą jaźń"
    );
    assert_eq!(
        voca_rs::manipulate::trim_right("-~-Earth--~", "-~"),
        "-~-Earth"
    );
    assert_eq!(
        voca_rs::manipulate::trim_right("++--Die Schildkröte fliegt über das Floß.++", "+"),
        "++--Die Schildkröte fliegt über das Floß."
    );
    assert_eq!(
        voca_rs::manipulate::trim_right("ё1ё2ёКак слышно, приём!ё1ё", "ёё12"),
        "ё1ё2ёКак слышно, приём!"
    );
    assert_eq!(voca_rs::manipulate::trim_right("", ""), "");
}
#[test]
fn _trim_right() {
    assert_eq!(
        "   The world - is yours\t   "._trim_right(""),
        "   The world - is yours"
    );
}
#[test]
fn zfill() {
    assert_eq!(voca_rs::manipulate::zfill("", 0), "");
    assert_eq!(voca_rs::manipulate::zfill("abc", 0), "abc");
    assert_eq!(voca_rs::manipulate::zfill("abc", 1), "abc");
    assert_eq!(voca_rs::manipulate::zfill("abc", 2), "abc");
    assert_eq!(voca_rs::manipulate::zfill("abc", 3), "abc");
    assert_eq!(voca_rs::manipulate::zfill("abc", 4), "0abc");
    assert_eq!(voca_rs::manipulate::zfill("abc", 5), "00abc");
    assert_eq!(voca_rs::manipulate::zfill("abc", 6), "000abc");
    assert_eq!(voca_rs::manipulate::zfill("Café", 4), "Café");
    assert_eq!(voca_rs::manipulate::zfill("Café", 5), "0Café");
    assert_eq!(voca_rs::manipulate::zfill("Café", 7), "000Café");
    assert_eq!(
        voca_rs::manipulate::zfill("Café del Mar", 15),
        "000Café del Mar"
    );
    assert_eq!(
        voca_rs::manipulate::zfill("Zażółć gęślą jaźń", 25),
        "00000000Zażółć gęślą jaźń"
    );
    assert_eq!(
        voca_rs::manipulate::zfill("Die Schildkröte fliegt", 29),
        "0000000Die Schildkröte fliegt"
    );
    assert_eq!(voca_rs::manipulate::zfill("Алё! Приём", 11), "0Алё! Приём");
}
#[test]
fn _zfill() {
    assert_eq!("abc"._zfill(5), "00abc");
}
#[test]
fn tr() {
    assert_eq!(voca_rs::manipulate::tr("", "", ""), "");
    assert_eq!(voca_rs::manipulate::tr("asd", "", ""), "asd");
    assert_eq!(voca_rs::manipulate::tr("asd", "", "a"), "asd");
    assert_eq!(
        voca_rs::manipulate::tr("test strtr", "t", "T"),
        "TesT sTrTr"
    );
    assert_eq!(
        voca_rs::manipulate::tr("test strtr", "test", "TEST"),
        "TEST STrTr"
    );
    assert_eq!(
        voca_rs::manipulate::tr("test strtr me", "tesm", "TES"),
        "TEST STrTr E"
    );
    assert_eq!(voca_rs::manipulate::tr("hello", "el", "ip"), "hippo");
    assert_eq!(voca_rs::manipulate::tr("légèreté", "éè", "ee"), "legerete");
    assert_eq!(
        voca_rs::manipulate::tr("Die Schildkröte fliegt über das Floß.", "üb", "un"),
        "Die Schildkröte fliegt uner das Floß."
    );
    assert_eq!(
        voca_rs::manipulate::tr("Как слышно, приём!", "ё", "е"),
        "Как слышно, прием!"
    );
}
#[test]
fn _tr() {
    assert_eq!("test strtr"._tr("t", "T"), "TesT sTrTr");
}
#[test]
fn word_wrap() {
    assert_eq!(voca_rs::manipulate::word_wrap("", 0, "", ""), "");
    assert_eq!(
        voca_rs::manipulate::word_wrap("Café del Mar", 12, "", ""),
        "Café del Mar"
    );
    assert_eq!(
        voca_rs::manipulate::word_wrap("Hello world", 11, "", ""),
        "Hello world"
    );
    assert_eq!(
        voca_rs::manipulate::word_wrap("Hello world", 5, "", ""),
        "Hello\nworld"
    );
    assert_eq!(
        voca_rs::manipulate::word_wrap("Yes. The fire rises.", 4, "", ""),
        "Yes.\nThe\nfire\nrises."
    );
    assert_eq!(
        voca_rs::manipulate::word_wrap("Cafe del Mar", 4, "", ""),
        "Cafe\ndel\nMar"
    );
    assert_eq!(
        voca_rs::manipulate::word_wrap("And I think to myself what a wonderful world.", 10, "", ""),
        "And I\nthink to\nmyself\nwhat a\nwonderful\nworld."
    );
    assert_eq!(
        voca_rs::manipulate::word_wrap("Hello world", 5, "<br/>", "__"),
        "__Hello<br/>__world"
    );
    assert_eq!(
        voca_rs::manipulate::word_wrap("Yes. The fire rises.", 4, "", "**"),
        "**Yes.\n**The\n**fire\n**rises."
    );
    assert_eq!(
        voca_rs::manipulate::word_wrap("Die Schildkröte fliegt über das Floß.", 15, "", ""),
        "Die Schildkröte\nfliegt über das\nFloß."
    );
}
#[test]
fn _word_wrap() {
    assert_eq!("Hello world"._word_wrap(5, "", ""), "Hello\nworld");
}
#[test]
fn finish() {
    assert_eq!(voca_rs::manipulate::finish("", ""), "");
    assert_eq!(voca_rs::manipulate::finish("foo bar", "bar"), "foo bar");
    assert_eq!(
        voca_rs::manipulate::finish("fóo bąr", " ço¨oł"),
        "fóo bąr ço¨oł"
    );
    assert_eq!(voca_rs::manipulate::finish("asd", ""), "asd");
    assert_eq!(voca_rs::manipulate::finish("", "asd"), "asd");
    assert_eq!(
        voca_rs::manipulate::finish("asd   QWE       zXc", "!"),
        "asd   QWE       zXc!"
    );
    assert_eq!(
        voca_rs::manipulate::finish("légèreté", "-éè"),
        "légèreté-éè"
    );
    assert_eq!(voca_rs::manipulate::finish("légèreté", "é"), "légèreté");
    assert_eq!(
        voca_rs::manipulate::finish("Как слышно, приём", "!"),
        "Как слышно, приём!"
    );
}
#[test]
fn start() {
    assert_eq!(voca_rs::manipulate::start("", ""), "");
    assert_eq!(voca_rs::manipulate::start("foo bar", "foo"), "foo bar");
    assert_eq!(
        voca_rs::manipulate::start("fóo bąr", "ço¨oł "),
        "ço¨oł fóo bąr"
    );
    assert_eq!(voca_rs::manipulate::start("asd", ""), "asd");
    assert_eq!(voca_rs::manipulate::start("", "asd"), "asd");
    assert_eq!(
        voca_rs::manipulate::start("asd   QWE       zXc", "!"),
        "!asd   QWE       zXc"
    );
    assert_eq!(voca_rs::manipulate::start("légèreté", "éè-"), "éè-légèreté");
    assert_eq!(voca_rs::manipulate::start("élégèreté", "é"), "élégèreté");
    assert_eq!(
        voca_rs::manipulate::start("Как слышно, приём!", "¡"),
        "¡Как слышно, приём!"
    );
}
