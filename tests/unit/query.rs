//! voca_rs::query testing
use voca_rs::Voca;

#[test]
fn ends_with() {
    assert!(voca_rs::query::ends_with("the world is yours", "is yours"));
    assert!(voca_rs::query::ends_with("Zażółć gęślą jaźń", "jaźń"));
    assert!(voca_rs::query::ends_with("the world is yours", ""));
    assert!(voca_rs::query::ends_with("", ""));
}
#[test]
fn _ends_with() {
    assert!("the world is yours"._ends_with("is yours"));
    assert!("Zażółć gęślą jaźń"._ends_with("jaźń"));
}
#[test]
fn includes() {
    assert!(voca_rs::query::includes("", "", 0));
    assert!(voca_rs::query::includes("a", "a", 0));
    assert!(voca_rs::query::includes("abc", "c", 2));
    assert!(voca_rs::query::includes(
        "the world is yours",
        "the world",
        0
    ));
    assert!(voca_rs::query::includes("Zażółć gęślą jaźń", "gęślą", 7));
    assert!(voca_rs::query::includes("the world is yours", "", 0));
    assert_eq!(voca_rs::query::includes("abc", "c", 20), false);
    assert_eq!(voca_rs::query::includes("abc", "z", 0), false);
}
#[test]
fn _includes() {
    assert!("Zażółć gęślą jaźń"._includes("gęślą", 7));
    assert!("abc"._includes("c", 2));
}
#[test]
fn is_alpha() {
    assert!(voca_rs::query::is_alpha("bart"));
    assert!(voca_rs::query::is_alpha("café"));
    assert!(voca_rs::query::is_alpha("cafe\u{0301}"));
    assert!(voca_rs::query::is_alpha("Zażółć"));
    assert_eq!(voca_rs::query::is_alpha(""), false);
    assert_eq!(voca_rs::query::is_alpha("T1000"), false);
    assert_eq!(voca_rs::query::is_alpha("\n\t"), false);
    assert_eq!(voca_rs::query::is_alpha("lisa!"), false);
    assert_eq!(voca_rs::query::is_alpha("lisa and bart"), false);
    assert_eq!(voca_rs::query::is_alpha("Zażółć gęślą jaźń"), false);
}
#[test]
fn _is_alpha() {
    assert!("bart"._is_alpha());
    assert!("café"._is_alpha());
    assert!("cafe\u{0301}"._is_alpha());
}
#[test]
fn is_alphadigit() {
    assert!(voca_rs::query::is_alphadigit("bart"));
    assert!(voca_rs::query::is_alphadigit("café"));
    assert!(voca_rs::query::is_alphadigit("cafe\u{0301}"));
    assert!(voca_rs::query::is_alphadigit("T1000"));
    assert!(voca_rs::query::is_alphadigit("1000"));
    assert!(voca_rs::query::is_alphadigit("Zażółć"));
    assert_eq!(voca_rs::query::is_alphadigit(""), false);
    assert_eq!(voca_rs::query::is_alphadigit("10-00"), false);
    assert_eq!(voca_rs::query::is_alphadigit("\n\t"), false);
    assert_eq!(voca_rs::query::is_alphadigit("lisa!"), false);
    assert_eq!(voca_rs::query::is_alphadigit("lisa and bart"), false);
    assert_eq!(voca_rs::query::is_alphadigit("Zażółć gęślą jaźń"), false);
}
#[test]
fn _is_alphadigit() {
    assert!("bart"._is_alphadigit());
    assert_eq!("10-00"._is_alphadigit(), false);
    assert!("T1000"._is_alphadigit());
}
#[test]
fn is_blank() {
    assert!(voca_rs::query::is_blank(""));
    assert!(voca_rs::query::is_blank("   "));
    assert!(voca_rs::query::is_blank("\n\t\r"));
    assert_eq!(voca_rs::query::is_blank("Zażółć gęślą jaźń"), false);
}
#[test]
fn _is_blank() {
    assert!("   "._is_blank());
    assert_eq!("Zażółć gęślą jaźń"._is_blank(), false);
}
#[test]
fn is_camel_case() {
    assert!(voca_rs::query::is_camel_case(""));
    assert!(voca_rs::query::is_camel_case("birdFlight"));
    assert_eq!(voca_rs::query::is_camel_case("bird flight"), false);
    assert_eq!(voca_rs::query::is_camel_case("-BIRD-FLIGHT-"), false);
    assert_eq!(voca_rs::query::is_camel_case("Zażółć gęślą jaźń"), false);
    assert!(voca_rs::query::is_camel_case("zażółćGęśląJaźń"));
}
#[test]
fn _is_camel_case() {
    assert!("birdFlight"._is_camel_case());
    assert_eq!("bird flight"._is_camel_case(), false);
    assert!("zażółćGęśląJaźń"._is_camel_case());
}
#[test]
fn is_capitalize() {
    assert!(voca_rs::query::is_capitalize(""));
    assert!(voca_rs::query::is_capitalize("John has a motorcycle"));
    assert_eq!(voca_rs::query::is_capitalize("the world is yours"), false);
    assert!(voca_rs::query::is_capitalize("The world is yours"));
    assert_eq!(voca_rs::query::is_capitalize("The World IS YourS"), false);
    assert!(voca_rs::query::is_capitalize("Zażółć gęślą jaźń"));
    assert_eq!(voca_rs::query::is_capitalize("ZAżółć GĘŚLĄ jAźń"), false);
    assert!(voca_rs::query::is_capitalize("Это вообще работает?"),);
    assert_eq!(voca_rs::query::is_capitalize("это Вообще РАБОТАЕТ?"), false);
}
#[test]
fn _is_capitalize() {
    assert!("John has a motorcycle"._is_capitalize());
    assert_eq!("the world is yours"._is_capitalize(), false);
}
#[test]
fn is_decapitalize() {
    assert!(voca_rs::query::is_decapitalize(""));
    assert_eq!(
        voca_rs::query::is_decapitalize("John has a motorcycle"),
        false
    );
    assert!(voca_rs::query::is_decapitalize("the world is yours"));
    assert_eq!(voca_rs::query::is_decapitalize("The world is yours"), false);
    assert_eq!(voca_rs::query::is_decapitalize("the World IS YourS"), false);
    assert!(voca_rs::query::is_decapitalize("zażółć gęślą jaźń"));
    assert_eq!(voca_rs::query::is_decapitalize("ZAżółć GĘŚLĄ jAźń"), false);
    assert!(voca_rs::query::is_decapitalize("это вообще работает?"));
    assert_eq!(
        voca_rs::query::is_decapitalize("Это вообще работает?"),
        false
    );
}
#[test]
fn _is_decapitalize() {
    assert_eq!("John has a motorcycle"._is_decapitalize(), false);
    assert!("the world is yours"._is_decapitalize());
}
#[test]
fn is_digit() {
    assert!(voca_rs::query::is_digit(""));
    assert!(voca_rs::query::is_digit("0"));
    assert!(voca_rs::query::is_digit("100"));
    assert!(voca_rs::query::is_digit("100500"));
    assert_eq!(voca_rs::query::is_digit("1.5"), false);
    assert_eq!(voca_rs::query::is_digit("0xFF"), false);
    assert_eq!(voca_rs::query::is_digit("ten"), false);
}
#[test]
fn _is_digit() {
    assert!("0"._is_digit());
    assert_eq!("1.5"._is_digit(), false);
}
#[test]
fn is_empty() {
    assert!(voca_rs::query::is_empty(""));
    assert_eq!(voca_rs::query::is_empty("Zażółć gęślą jaźń"), false);
    assert_eq!(voca_rs::query::is_empty("the world is yours"), false);
}
#[test]
fn _is_empty() {
    assert!(""._is_empty());
    assert_eq!("the world is yours"._is_empty(), false);
}
#[test]
fn is_foreign_key() {
    assert!(voca_rs::query::is_foreign_key(""));
    assert!(voca_rs::query::is_foreign_key("foo_bar_id"));
    assert_eq!(voca_rs::query::is_foreign_key("foo_bar"), false);
    assert_eq!(voca_rs::query::is_foreign_key("the world is yours"), false);
    assert_eq!(
        voca_rs::query::is_foreign_key("foo-bar-string-that-is-really-really-long"),
        false
    );
    assert_eq!(
        voca_rs::query::is_foreign_key("FooBarIsAReallyReallyLongString"),
        false
    );
    assert_eq!(
        voca_rs::query::is_foreign_key("foo_bar_string_that_is_really_really_long"),
        false
    );
    assert_eq!(
        voca_rs::query::is_foreign_key("voca::voca_rs::query::is_foreign_key"),
        false
    );
}
#[test]
fn _is_foreign_key() {
    assert!(""._is_foreign_key());
    assert!("foo_bar_id"._is_foreign_key());
    assert_eq!("foo_bar"._is_foreign_key(), false);
    assert_eq!("the world is yours"._is_foreign_key(), false);
    assert_eq!(
        "foo-bar-string-that-is-really-really-long"._is_foreign_key(),
        false
    );
}
#[test]
fn is_lowercase() {
    assert!(voca_rs::query::is_lowercase(""));
    assert!(voca_rs::query::is_lowercase("the world is yours"));
    assert_eq!(voca_rs::query::is_lowercase("Zażółć gęślą jaźń"), false);
    assert_eq!(voca_rs::query::is_lowercase("T1000"), false);
}
#[test]
fn _is_lowercase() {
    assert!("the world is yours"._is_lowercase());
    assert_eq!("T1000"._is_lowercase(), false);
}
#[test]
fn is_lower_first() {
    assert!(voca_rs::query::is_lower_first(""));
    assert!(voca_rs::query::is_lower_first("the world is yours"));
    assert!(voca_rs::query::is_lower_first("tHE World"));
    assert!(voca_rs::query::is_lower_first("żółć niedźwiedzia"));
    assert_eq!(voca_rs::query::is_lower_first("Zażółć gęślą jaźń"), false);
    assert_eq!(voca_rs::query::is_lower_first("T1000"), false);
}
#[test]
fn _is_lower_first() {
    assert!("the world is yours"._is_lower_first());
    assert_eq!("Zażółć gęślą jaźń"._is_lower_first(), false);
}
#[test]
fn is_kebab_case() {
    assert!(voca_rs::query::is_kebab_case(""));
    assert!(voca_rs::query::is_kebab_case("bird-flight"));
    assert!(voca_rs::query::is_kebab_case("is-kebab-case"));
    assert!(voca_rs::query::is_kebab_case("zażółć-gęślą-jaźń"));
    assert_eq!(voca_rs::query::is_kebab_case("-BIRD-FLIGHT-"), false);
    assert_eq!(voca_rs::query::is_kebab_case("tHE World"), false);
    assert_eq!(voca_rs::query::is_kebab_case("żółć niedźwiedzia"), false);
    assert_eq!(voca_rs::query::is_kebab_case("T1000"), false);
}
#[test]
fn _is_kebab_case() {
    assert!("bird-flight"._is_kebab_case());
    assert_eq!("-BIRD-FLIGHT-"._is_kebab_case(), false);
}
#[test]
fn is_numeric() {
    assert!(voca_rs::query::is_numeric(""));
    assert!(voca_rs::query::is_numeric("0"));
    assert!(voca_rs::query::is_numeric("+0"));
    assert!(voca_rs::query::is_numeric("0.0"));
    assert!(voca_rs::query::is_numeric("1000"));
    assert!(voca_rs::query::is_numeric("1.56"));
    assert!(voca_rs::query::is_numeric("-10.888"));
    assert!(voca_rs::query::is_numeric("350"));
    assert!(voca_rs::query::is_numeric("-20.5"));
    assert!(voca_rs::query::is_numeric("1.5E+2"));
    assert!(voca_rs::query::is_numeric("1.25E-3"));
    assert!(voca_rs::query::is_numeric("125e5"));
    assert!(voca_rs::query::is_numeric("125e-3"));
    assert!(voca_rs::query::is_numeric("0xFF"));
    assert!(voca_rs::query::is_numeric("0x22"));
    assert!(voca_rs::query::is_numeric("0x123ABC"));
    assert!(voca_rs::query::is_numeric("0x123ABC"));
    assert!(voca_rs::query::is_numeric("0x1ab9"));
    assert_eq!(voca_rs::query::is_numeric("0x123z"), false);
    assert_eq!(voca_rs::query::is_numeric("five"), false);
    assert_eq!(voca_rs::query::is_numeric(".."), false);
    assert_eq!(voca_rs::query::is_numeric(" "), false);
}
#[test]
fn _is_numeric() {
    assert!("0"._is_numeric());
    assert!("1.5E+2"._is_numeric());
    assert!("0x22"._is_numeric());
    assert_eq!("0x123z"._is_numeric(), false);
}
#[test]
fn is_pascal_case() {
    assert!(voca_rs::query::is_pascal_case(""));
    assert!(voca_rs::query::is_pascal_case("BirdFlight"));
    assert!(voca_rs::query::is_pascal_case("ЭтоПаскальКейс"));
    assert_eq!(voca_rs::query::is_pascal_case("birdFlight"), false);
    assert_eq!(voca_rs::query::is_pascal_case("bird flight"), false);
    assert_eq!(voca_rs::query::is_pascal_case("-BIRD-FLIGHT-"), false);
    assert_eq!(voca_rs::query::is_pascal_case("Zażółć gęślą jaźń"), false);
    assert!(voca_rs::query::is_pascal_case("ZażółćGęśląJaźń"));
    assert_eq!(voca_rs::query::is_pascal_case("zażółćGęśląJaźń"), false);
}
#[test]
fn _is_pascal_case() {
    assert!("BirdFlight"._is_pascal_case());
    assert_eq!("birdFlight"._is_pascal_case(), false);
}
#[test]
fn is_shouty_kebab_case() {
    assert!(voca_rs::query::is_shouty_kebab_case(""));
    assert!(voca_rs::query::is_shouty_kebab_case("BIRD-FLIGHT"));
    assert!(voca_rs::query::is_shouty_kebab_case("ЭТО-ОЙ-КЕБАБ-КЕЙС"));
    assert_eq!(voca_rs::query::is_shouty_kebab_case("birdFlight"), false);
    assert_eq!(voca_rs::query::is_shouty_kebab_case("bird flight"), false);
    assert_eq!(voca_rs::query::is_shouty_kebab_case("-BIRD-FLIGHT-"), false);
    assert_eq!(
        voca_rs::query::is_shouty_kebab_case("Zażółć gęślą jaźń"),
        false
    );
    assert!(voca_rs::query::is_shouty_kebab_case("ZAŻÓŁĆ-GĘŚLĄ-JAŹŃ"));
    assert_eq!(
        voca_rs::query::is_shouty_kebab_case("zażółćGęśląJaźń"),
        false
    );
}
#[test]
fn _is_shouty_kebab_case() {
    assert!("BIRD-FLIGHT"._is_shouty_kebab_case());
    assert_eq!("birdFlight"._is_shouty_kebab_case(), false);
}
#[test]
fn is_snake_case() {
    assert!(voca_rs::query::is_snake_case(""));
    assert!(voca_rs::query::is_snake_case("bird_flight"));
    assert!(voca_rs::query::is_snake_case("это_снэйк_кейс"));
    assert_eq!(voca_rs::query::is_snake_case("birdFlight"), false);
    assert_eq!(voca_rs::query::is_snake_case("bird flight"), false);
    assert_eq!(voca_rs::query::is_snake_case("-BIRD-FLIGHT-"), false);
    assert_eq!(voca_rs::query::is_snake_case("Zażółć gęślą jaźń"), false);
    assert!(voca_rs::query::is_snake_case("zażółć_gęślą_jaźń"));
    assert_eq!(voca_rs::query::is_snake_case("zażółćGęśląJaźń"), false);
}
#[test]
fn _is_snake_case() {
    assert!("bird_flight"._is_snake_case());
    assert_eq!("birdFlight"._is_snake_case(), false);
}
#[test]
fn is_shouty_snake_case() {
    assert!(voca_rs::query::is_shouty_snake_case(""));
    assert!(voca_rs::query::is_shouty_snake_case("BIRD_FLIGHT"));
    assert_eq!(voca_rs::query::is_shouty_snake_case("bird_flight"), false);
    assert!(voca_rs::query::is_shouty_snake_case("ЭТО_СНЭЙК_КЕЙС"));
    assert_eq!(voca_rs::query::is_shouty_snake_case("birdFlight"), false);
    assert_eq!(voca_rs::query::is_shouty_snake_case("bird flight"), false);
    assert_eq!(voca_rs::query::is_shouty_snake_case("-BIRD-FLIGHT-"), false);
    assert_eq!(
        voca_rs::query::is_shouty_snake_case("Zażółć gęślą jaźń"),
        false
    );
    assert!(voca_rs::query::is_shouty_snake_case("ZAŻÓŁĆ_GĘŚLĄ_JAŹŃ"));
    assert_eq!(
        voca_rs::query::is_shouty_snake_case("zażółćGęśląJaźń"),
        false
    );
}
#[test]
fn _is_shouty_snake_case() {
    assert!("BIRD_FLIGHT"._is_shouty_snake_case());
    assert_eq!("bird_flight"._is_shouty_snake_case(), false);
}
#[test]
fn is_title() {
    assert_eq!(voca_rs::query::is_title(""), false);
    assert_eq!(voca_rs::query::is_title("The World Is Yours"), true);
    assert_eq!(voca_rs::query::is_title("the world is yours"), false);
    assert!(voca_rs::query::is_title("This Is String Example...Wow!!!"));
    assert_eq!(
        voca_rs::query::is_title("This is string example....wow!!!"),
        false
    );
    assert!(voca_rs::query::is_title("Zażółć Gęślą Jaźń"));
    assert_eq!(voca_rs::query::is_title("Zażółć gęślą jaźń"), false);
    assert_eq!(voca_rs::query::is_title("T1000"), true);
}
#[test]
fn _is_title() {
    assert_eq!("The World Is Yours"._is_title(), true);
    assert_eq!("the world is yours"._is_title(), false);
}
#[test]
fn is_train_case() {
    assert!(voca_rs::query::is_train_case(""));
    assert!(voca_rs::query::is_train_case("Goodbye-Blue-Sky"));
    assert_eq!(voca_rs::query::is_train_case("bird_flight"), false);
    assert!(voca_rs::query::is_train_case("Это-Снэйк-Кейс"));
    assert_eq!(voca_rs::query::is_train_case("birdFlight"), false);
    assert_eq!(voca_rs::query::is_train_case("bird flight"), false);
    assert_eq!(voca_rs::query::is_train_case("-BIRD-FLIGHT-"), false);
    assert_eq!(voca_rs::query::is_train_case("Zażółć gęślą jaźń"), false);
    assert!(voca_rs::query::is_train_case("Zażółć-Gęślą-Jaźń"));
    assert_eq!(voca_rs::query::is_train_case("zażółćGęśląJaźń"), false);
}
#[test]
fn _is_train_case() {
    assert!(voca_rs::query::is_train_case("Goodbye-Blue-Sky"));
    assert_eq!(voca_rs::query::is_train_case("bird_flight"), false);
}
#[test]
fn is_uppercase() {
    assert!(voca_rs::query::is_uppercase(""));
    assert!(voca_rs::query::is_uppercase("THE WORLD IS YOURS"));
    assert_eq!(voca_rs::query::is_uppercase("Zażółć gęślą jaźń"), false);
    assert_eq!(voca_rs::query::is_uppercase("t1000"), false);
}
#[test]
fn _is_uppercase() {
    assert!("THE WORLD IS YOURS"._is_uppercase());
    assert_eq!("Zażółć gęślą jaźń"._is_uppercase(), false);
}
#[test]
fn is_upper_first() {
    assert!(voca_rs::query::is_upper_first(""));
    assert!(voca_rs::query::is_upper_first("The world is yours"));
    assert!(voca_rs::query::is_upper_first("Zażółć gęślą jaźń"));
    assert!(voca_rs::query::is_upper_first("T1000"));
    assert!(voca_rs::query::is_upper_first("Żółć niedźwiedzia"));
    assert_eq!(voca_rs::query::is_upper_first("żółć niedźwiedzia"), false);
    assert_eq!(voca_rs::query::is_upper_first("tHE World"), false);
}
#[test]
fn _is_upper_first() {
    assert!(voca_rs::query::is_upper_first("The world is yours"));
    assert_eq!(voca_rs::query::is_upper_first("żółć niedźwiedzia"), false);
}
#[test]
fn matches() {
    assert!(voca_rs::query::matches("", "", 0));
    assert_eq!(voca_rs::query::matches("pluto", "a", 0), false);
    assert!(voca_rs::query::matches("pluto", r"plu.{2}", 0));
    assert_eq!(voca_rs::query::matches("apollo 11", r"\d{3}", 0), false);
    assert!(voca_rs::query::matches("pacific", "", 0));
    assert_eq!(voca_rs::query::matches("", "1", 0), false);
    assert!(voca_rs::query::matches("pacific ocean", "ocean", 0));
    assert!(voca_rs::query::matches(
        "pacific ocean",
        "^pacific ocean$",
        0
    ));
    assert!(voca_rs::query::matches("pacific ocean", r"\s", 0));
    assert!(voca_rs::query::matches("1500", r"\d", 0));
    assert!(voca_rs::query::matches("Zażółć gęślą jaźń", "gęślą", 0));
    assert!(voca_rs::query::matches("Zażółć gęślą jaźń", "gęślą", 11));
    assert_eq!(
        voca_rs::query::matches("Zażółć gęślą jaźń", "gęślą", 12),
        false
    );
    assert_eq!(
        // [^] is not a valid regex
        voca_rs::query::matches("Zażółć gęślą jaźń", "[^]", 0),
        false
    );
}
#[test]
fn _matches() {
    assert_eq!("pluto"._matches("a", 0), false);
    assert!("pluto"._matches(r"plu.{2}", 0));
}
#[test]
fn query() {
    assert!(voca_rs::query::query("", "", 0));
    assert!(voca_rs::query::query("a", "a", 0));
    assert!(voca_rs::query::query("abc", "c", 2));
    assert!(voca_rs::query::query("the world is yours", "the world", 0));
    assert!(voca_rs::query::query("the world is yours", "te wld", 0));
    assert!(voca_rs::query::query("the world is yours", "td", 0));
    assert!(voca_rs::query::query("Zażółć gęślą jaźń", "gęślą", 7));
    assert!(voca_rs::query::query("the world is yours", "", 0));
    assert_eq!(
        voca_rs::query::query("the world is yours", "asdd", 0),
        false
    );
    assert_eq!(voca_rs::query::query("the world is yours", "eht", 0), false);
    assert_eq!(voca_rs::query::query("abc", "c", 20), false);
    assert_eq!(voca_rs::query::query("abc", "z", 0), false);
}
#[test]
fn _query() {
    assert!("the world is yours"._query("the world", 0));
    assert_eq!("the world is yours"._query("eht", 0), false);
}
#[test]
fn starts_with() {
    assert!(voca_rs::query::starts_with(
        "the world is yours",
        "the world"
    ));
    assert!(voca_rs::query::starts_with("Zażółć gęślą jaźń", "Zażółć"));
    assert!(voca_rs::query::starts_with("the world is yours", ""));
    assert!(voca_rs::query::starts_with("", ""));
    assert_eq!(
        voca_rs::query::starts_with("the world is yours", "s"),
        false
    );
}
#[test]
fn _starts_with() {
    assert!("the world is yours"._starts_with("the world"));
    assert_eq!("the world is yours"._starts_with("s"), false);
}
