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
fn ends_with_me() {
    assert!("the world is yours".ends_with("is yours"));
    assert!("Zażółć gęślą jaźń".ends_with("jaźń"));
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
fn includes_me() {
    assert!("Zażółć gęślą jaźń".includes("gęślą", 7));
    assert!("abc".includes("c", 2));
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
fn is_alpha_me() {
    assert!("bart".is_alpha());
    assert!("café".is_alpha());
    assert!("cafe\u{0301}".is_alpha());
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
fn is_alphadigit_me() {
    assert!("bart".is_alphadigit());
    assert_eq!("10-00".is_alphadigit(), false);
    assert!("T1000".is_alphadigit());
}
#[test]
fn is_blank() {
    assert!(voca_rs::query::is_blank(""));
    assert!(voca_rs::query::is_blank("   "));
    assert!(voca_rs::query::is_blank("\n\t\r"));
    assert_eq!(voca_rs::query::is_blank("Zażółć gęślą jaźń"), false);
}
#[test]
fn is_blank_me() {
    assert!("   ".is_blank());
    assert_eq!("Zażółć gęślą jaźń".is_blank(), false);
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
fn is_camel_case_me() {
    assert!("birdFlight".is_camel_case());
    assert_eq!("bird flight".is_camel_case(), false);
    assert!("zażółćGęśląJaźń".is_camel_case());
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
fn is_capitalize_me() {
    assert!("John has a motorcycle".is_capitalize());
    assert_eq!("the world is yours".is_capitalize(), false);
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
fn is_decapitalize_me() {
    assert_eq!("John has a motorcycle".is_decapitalize(), false);
    assert!("the world is yours".is_decapitalize());
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
fn is_digit_me() {
    assert!("0".is_digit());
    assert_eq!("1.5".is_digit(), false);
}
#[test]
fn is_empty() {
    assert!(voca_rs::query::is_empty(""));
    assert_eq!(voca_rs::query::is_empty("Zażółć gęślą jaźń"), false);
    assert_eq!(voca_rs::query::is_empty("the world is yours"), false);
}
#[test]
fn is_empty_me() {
    assert!("".is_empty());
    assert_eq!("the world is yours".is_empty(), false);
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
fn is_foreign_key_me() {
    assert!("".is_foreign_key());
    assert!("foo_bar_id".is_foreign_key());
    assert_eq!("foo_bar".is_foreign_key(), false);
    assert_eq!("the world is yours".is_foreign_key(), false);
    assert_eq!(
        "foo-bar-string-that-is-really-really-long".is_foreign_key(),
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
fn is_lowercase_me() {
    assert!("the world is yours".is_lowercase());
    assert_eq!("T1000".is_lowercase(), false);
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
fn is_lower_first_me() {
    assert!("the world is yours".is_lower_first());
    assert_eq!("Zażółć gęślą jaźń".is_lower_first(), false);
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
fn is_kebab_case_me() {
    assert!("bird-flight".is_kebab_case());
    assert_eq!("-BIRD-FLIGHT-".is_kebab_case(), false);
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
fn is_numeric_me() {
    assert!("0".is_numeric());
    assert!("1.5E+2".is_numeric());
    assert!("0x22".is_numeric());
    assert_eq!("0x123z".is_numeric(), false);
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
fn is_pascal_case_me() {
    assert!("BirdFlight".is_pascal_case());
    assert_eq!("birdFlight".is_pascal_case(), false);
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
fn is_shouty_kebab_case_me() {
    assert!("BIRD-FLIGHT".is_shouty_kebab_case());
    assert_eq!("birdFlight".is_shouty_kebab_case(), false);
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
fn is_snake_case_me() {
    assert!("bird_flight".is_snake_case());
    assert_eq!("birdFlight".is_snake_case(), false);
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
fn is_shouty_snake_case_me() {
    assert!("BIRD_FLIGHT".is_shouty_snake_case());
    assert_eq!("bird_flight".is_shouty_snake_case(), false);
}
#[test]
fn is_title() {
    assert_eq!(voca_rs::query::is_title(""), false);
    assert!(voca_rs::query::is_title("The World Is Yours"), true);
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
fn is_title_me() {
    assert!("The World Is Yours".is_title(), true);
    assert_eq!("the world is yours".is_title(), false);
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
fn is_train_case_me() {
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
fn is_uppercase_me() {
    assert!("THE WORLD IS YOURS".is_uppercase());
    assert_eq!("Zażółć gęślą jaźń".is_uppercase(), false);
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
fn is_upper_first_me() {
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
fn matches_me() {
    assert_eq!("pluto".matches_me("a", 0), false);
    assert!("pluto".matches_me(r"plu.{2}", 0));
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
fn query_me() {
    assert!("the world is yours".query("the world", 0));
    assert_eq!("the world is yours".query("eht", 0), false);
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
fn starts_with_me() {
    assert!("the world is yours".starts_with_me("the world"));
    assert_eq!("the world is yours".starts_with_me("s"), false);
}
