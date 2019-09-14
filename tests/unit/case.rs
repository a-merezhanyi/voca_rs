//! voca_rs::case testing
use voca_rs::Voca;

#[test]
fn camel_case() {
    assert_eq!(
        voca_rs::case::camel_case("The World - IS Yours"),
        "theWorldIsYours"
    );
    assert_eq!(
        voca_rs::case::camel_case("_Zażółć-GĘŚLĄ_jaźń-"),
        "zażółćGęśląJaźń"
    );
    assert_eq!(
        voca_rs::case::camel_case("say  ***    Hello\r\n   to--ME++"),
        "sayHelloToMe"
    );
    assert_eq!(voca_rs::case::camel_case(""), "");
}
#[test]
fn camel_case_me() {
    assert_eq!("The World - IS Yours".camel_case(), "theWorldIsYours");
}
#[test]
fn pascal_case() {
    assert_eq!(
        voca_rs::case::pascal_case("The World - IS Yours"),
        "TheWorldIsYours"
    );
    assert_eq!(
        voca_rs::case::pascal_case("_Zażółć-GĘŚLĄ_jaźń-"),
        "ZażółćGęśląJaźń"
    );
    assert_eq!(
        voca_rs::case::pascal_case("say  ***    Hello\r\n   to--ME++"),
        "SayHelloToMe"
    );
    assert_eq!(voca_rs::case::pascal_case(""), "");
}
#[test]
fn pascal_case_me() {
    assert_eq!("The World - IS Yours".pascal_case(), "TheWorldIsYours");
}
#[test]
fn capitalize() {
    assert_eq!(
        voca_rs::case::capitalize("The World IS YourS", true),
        "The world is yours"
    );
    assert_eq!(
        voca_rs::case::capitalize("ZAżółć GĘŚLĄ jAźń", true),
        "Zażółć gęślą jaźń"
    );
    assert_eq!(
        voca_rs::case::capitalize("say Hello to ME", false),
        "Say Hello to ME"
    );
    assert_eq!(voca_rs::case::capitalize("", true), "");
}
#[test]
fn capitalize_me() {
    assert_eq!("The World IS YourS".capitalize(true), "The world is yours");
}
#[test]
fn decapitalize() {
    assert_eq!(
        voca_rs::case::decapitalize("The World IS YourS", true),
        "the world is yours"
    );
    assert_eq!(
        voca_rs::case::decapitalize("ZAżółć GĘŚLĄ jAźń", true),
        "zażółć gęślą jaźń"
    );
    assert_eq!(
        voca_rs::case::decapitalize("Say Hello to ME", false),
        "say Hello to ME"
    );
    assert_eq!(voca_rs::case::decapitalize("", true), "");
}
#[test]
fn decapitalize_me() {
    assert_eq!(
        "The World IS YourS".decapitalize(true),
        "the world is yours"
    );
}
#[test]
fn kebab_case() {
    assert_eq!(
        voca_rs::case::kebab_case("The World - IS Yours"),
        "the-world-is-yours"
    );
    assert_eq!(
        voca_rs::case::kebab_case("_Zażółć-GĘŚLĄ_jaźń-"),
        "zażółć-gęślą-jaźń"
    );
    assert_eq!(
        voca_rs::case::kebab_case("say  ***    Hello\r\n   to--ME++"),
        "say-hello-to-me"
    );
    assert_eq!(voca_rs::case::kebab_case(""), "");
}
#[test]
fn kebab_case_me() {
    assert_eq!("The World - IS Yours".kebab_case(), "the-world-is-yours");
}
#[test]
fn shouty_kebab_case() {
    assert_eq!(
        voca_rs::case::shouty_kebab_case("The World - IS Yours"),
        "THE-WORLD-IS-YOURS"
    );
    assert_eq!(
        voca_rs::case::shouty_kebab_case("_Zażółć-GĘŚLĄ_jaźń-"),
        "ZAŻÓŁĆ-GĘŚLĄ-JAŹŃ"
    );
    assert_eq!(
        voca_rs::case::shouty_kebab_case("say  ***    Hello\r\n   to--ME++"),
        "SAY-HELLO-TO-ME"
    );
    assert_eq!(voca_rs::case::shouty_kebab_case(""), "");
}
#[test]
fn shouty_kebab_case_me() {
    assert_eq!(
        "The World - IS Yours".shouty_kebab_case(),
        "THE-WORLD-IS-YOURS"
    );
}
#[test]
fn train_case() {
    assert_eq!(
        voca_rs::case::train_case("The World - IS Yours"),
        "The-World-Is-Yours"
    );
    assert_eq!(
        voca_rs::case::train_case("_Zażółć-GĘŚLĄ_jaźń-"),
        "Zażółć-Gęślą-Jaźń"
    );
    assert_eq!(
        voca_rs::case::train_case("say  ***    Hello\r\n   to--ME++"),
        "Say-Hello-To-Me"
    );
    assert_eq!(voca_rs::case::train_case(""), "");
}
#[test]
fn train_case_me() {
    assert_eq!("The World - IS Yours".train_case(), "The-World-Is-Yours");
}
#[test]
fn lower_case() {
    assert_eq!(
        voca_rs::case::lower_case("The World IS YourS"),
        "the world is yours"
    );
    assert_eq!(
        voca_rs::case::lower_case("Zażółć gęśLą jaźń"),
        "zażółć gęślą jaźń"
    );
    assert_eq!(voca_rs::case::lower_case(""), "");
}
#[test]
fn lower_case_me() {
    assert_eq!("The World IS YourS".lower_case(), "the world is yours");
}
#[test]
fn snake_case() {
    assert_eq!(
        voca_rs::case::snake_case("The World - IS Yours"),
        "the_world_is_yours"
    );
    assert_eq!(
        voca_rs::case::snake_case("_Zażółć-GĘŚLĄ_jaźń-"),
        "zażółć_gęślą_jaźń"
    );
    assert_eq!(
        voca_rs::case::snake_case("say  ***    Hello\r\n   to--ME++"),
        "say_hello_to_me"
    );
    assert_eq!(voca_rs::case::snake_case(""), "");
}
#[test]
fn snake_case_me() {
    assert_eq!("The World - IS Yours".snake_case(), "the_world_is_yours");
}
#[test]
fn shouty_snake_case() {
    assert_eq!(
        voca_rs::case::shouty_snake_case("The World - IS Yours"),
        "THE_WORLD_IS_YOURS"
    );
    assert_eq!(
        voca_rs::case::shouty_snake_case("_Zażółć-GĘŚLĄ_jaźń-"),
        "ZAŻÓŁĆ_GĘŚLĄ_JAŹŃ"
    );
    assert_eq!(
        voca_rs::case::shouty_snake_case("say  ***    Hello\r\n   to--ME++"),
        "SAY_HELLO_TO_ME"
    );
    assert_eq!(voca_rs::case::shouty_snake_case(""), "");
}
#[test]
fn shouty_snake_case_me() {
    assert_eq!(
        "The World - IS Yours".shouty_snake_case(),
        "THE_WORLD_IS_YOURS"
    );
}
#[test]
fn swap_case() {
    assert_eq!(
        voca_rs::case::swap_case("The World - IS Yours"),
        "tHE wORLD - is yOURS"
    );
    assert_eq!(
        voca_rs::case::swap_case("_Zażółć-GĘŚLĄ_jaźń-"),
        "_zAŻÓŁĆ-gęślą_JAŹŃ-"
    );
    assert_eq!(
        voca_rs::case::swap_case("say über Hello to--ME++"),
        "SAY ÜBER hELLO TO--me++"
    );
    assert_eq!(voca_rs::case::swap_case(""), "");
}
#[test]
fn swap_case_me() {
    assert_eq!("The World - IS Yours".swap_case(), "tHE wORLD - is yOURS");
}
#[test]
fn upper_case() {
    assert_eq!(
        voca_rs::case::upper_case("The World IS YourS"),
        "THE WORLD IS YOURS"
    );
    assert_eq!(
        voca_rs::case::upper_case("Zażółć gęślą jaźń"),
        "ZAŻÓŁĆ GĘŚLĄ JAŹŃ"
    );
    assert_eq!(voca_rs::case::upper_case(""), "");
}
#[test]
fn upper_case_me() {
    assert_eq!("The World IS YourS".upper_case(), "THE WORLD IS YOURS");
}
#[test]
fn title_case() {
    assert_eq!(
        voca_rs::case::title_case("The World - IS Yours"),
        "The World Is Yours"
    );
    assert_eq!(
        voca_rs::case::title_case("_Zażółć-GĘŚLĄ_jaźń-"),
        "Zażółć Gęślą Jaźń"
    );
    assert_eq!(
        voca_rs::case::title_case("say über Hello to--ME++"),
        "Say Über Hello To Me"
    );
    assert_eq!(voca_rs::case::title_case(""), "");
}
#[test]
fn title_case_me() {
    assert_eq!("The World - IS Yours".title_case(), "The World Is Yours");
}
#[test]
fn lower_first() {
    assert_eq!(voca_rs::case::lower_first("Fred"), "fred");
    assert_eq!(voca_rs::case::lower_first("FRED"), "fRED");
    assert_eq!(
        voca_rs::case::lower_first("The World IS YourS"),
        "the World IS YourS"
    );
    assert_eq!(
        voca_rs::case::lower_first("ZAżółć GĘŚLĄ jAźń"),
        "zAżółć GĘŚLĄ jAźń"
    );
    assert_eq!(
        voca_rs::case::lower_first("Über Hello to ME"),
        "über Hello to ME"
    );
    assert_eq!(voca_rs::case::lower_first(""), "");
}
#[test]
fn lower_first_me() {
    assert_eq!("Fred".lower_first(), "fred");
}
#[test]
fn upper_first() {
    assert_eq!(voca_rs::case::upper_first("Fred"), "Fred");
    assert_eq!(voca_rs::case::upper_first("FRED"), "FRED");
    assert_eq!(
        voca_rs::case::upper_first("the World IS YourS"),
        "The World IS YourS"
    );
    assert_eq!(
        voca_rs::case::upper_first("The World IS YourS"),
        "The World IS YourS"
    );
    assert_eq!(
        voca_rs::case::upper_first("zAżółć GĘŚLĄ jAźń"),
        "ZAżółć GĘŚLĄ jAźń"
    );
    assert_eq!(
        voca_rs::case::upper_first("über Hello to ME"),
        "Über Hello to ME"
    );
    assert_eq!(voca_rs::case::upper_first(""), "");
}
#[test]
fn upper_first_me() {
    assert_eq!("fred".upper_first(), "Fred");
}
