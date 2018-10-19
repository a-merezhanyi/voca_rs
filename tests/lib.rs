extern crate voca_rs;

#[cfg(test)]
mod tests {
    use voca_rs::*;

    /// voca_rs::utils testing
    #[test]
    fn utils_version() {
        assert_eq!(utils::VERSION, "0.4.0");
    }
    #[test]
    fn utils_ascii_letters() {
        assert_eq!(
            utils::ASCII_LETTERS,
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );
    }
    #[test]
    fn utils_ascii_lowercase() {
        assert_eq!(utils::ASCII_LOWERCASE, "abcdefghijklmnopqrstuvwxyz");
    }
    #[test]
    fn utils_ascii_uppercase() {
        assert_eq!(utils::ASCII_UPPERCASE, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    #[test]
    fn utils_digits() {
        assert_eq!(utils::DIGITS, "0123456789");
    }
    #[test]
    fn utils_hexdigits() {
        assert_eq!(utils::HEXDIGITS, "0123456789abcdefABCDEF");
    }
    #[test]
    fn utils_octdigits() {
        assert_eq!(utils::OCTDIGITS, "01234567");
    }
    #[test]
    fn utils_punctuation() {
        assert_eq!(utils::PUNCTUATION, "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~");
    }
    #[test]
    fn utils_printable() {
        assert_eq!(utils::PRINTABLE, "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r");
    }
    #[test]
    fn utils_whitespace() {
        assert_eq!(utils::WHITESPACE, " \t\n\r");
    }

    /// voca_rs::split testing
    #[test]
    fn split_to_chars() {
        assert_eq!(split::chars("gravity"), ["g", "r", "a", "v", "i", "t", "y"]);
        assert_eq!(split::chars("  "), [" ", " "]);
        assert_eq!(split::chars("a b"), ["a", " ", "b"]);
        assert_eq!(split::chars("ÜbER"), ["Ü", "b", "E", "R"]);
        assert_eq!(split::chars("\n\t"), ["\n", "\t"]);
        assert_eq!(split::chars(""), [""]);
    }
    #[test]
    fn split_by_pattern() {
        assert_eq!(
            split::split("gravity can cross dimensions", " "),
            ["gravity", "can", "cross", "dimensions"]
        );
        assert_eq!(split::split("*dying*star*", "*"), ["", "dying", "star"]);
        assert_eq!(split::split("dying star", ""), ["dying star"]);
        assert_eq!(split::split("Über Stern", ""), ["Über Stern"]);
        assert_eq!(split::split("", ""), [""]);
    }
    #[test]
    fn split_words() {
        assert_eq!(
            split::words("gravity can cross dimensions"),
            ["gravity", "can", "cross", "dimensions"]
        );
        assert_eq!(
            split::words("gravity    dying\r\nstar\tfalling"),
            ["gravity", "dying", "star", "falling"]
        );
        assert_eq!(
            split::words("Zażółć gęślą jaźń"),
            ["Zażółć", "gęślą", "jaźń"]
        );
        assert_eq!(split::words("splitCamelCase"), ["split", "Camel", "Case"]);
        assert_eq!(
            split::words("split-some kind_of_mixed CaseHere"),
            ["split", "some", "kind", "of", "mixed", "Case", "Here"]
        );
        assert_eq!(
            split::words("LazyLoad with XMLHttpRequest and snake_case"),
            ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
        );
    }
    #[test]
    fn split_to_graphemes() {
        assert_eq!(
            split::graphemes("a̐éö̲\r\n"),
            ["a̐", "é", "ö̲", "\r\n"]
        );
        assert_eq!(split::graphemes(""), [""]);
    }

    /// voca_rs::query testing
    #[test]
    fn query_ends_with() {
        assert!(query::ends_with("the world is yours", "is yours"));
        assert!(query::ends_with("Zażółć gęślą jaźń", "jaźń"));
        assert!(query::ends_with("the world is yours", ""));
        assert!(query::ends_with("", ""), true);
    }
    #[test]
    fn query_includes() {
        assert!(query::includes("the world is yours", "the world", 0));
        assert!(query::includes("Zażółć gęślą jaźń", "gęślą", 7));
        assert!(query::includes("the world is yours", "", 0));
        assert!(query::includes("", "", 0), true);
    }
    #[test]
    fn query_is_alpha() {
        assert_eq!(query::is_alpha(""), false);
        assert_eq!(query::is_alpha("bart"), true);
        assert_eq!(query::is_alpha("café"), true);
        assert_eq!(query::is_alpha("cafe\u{0301}"), true);
        assert_eq!(query::is_alpha("T1000"), false);
        assert_eq!(query::is_alpha("\n\t"), false);
        assert_eq!(query::is_alpha("lisa!"), false);
        assert_eq!(query::is_alpha("lisa and bart"), false);
        assert_eq!(query::is_alpha("Zażółć"), true);
        assert_eq!(query::is_alpha("Zażółć gęślą jaźń"), false);
    }
    #[test]
    fn query_is_alphadigit() {
        assert_eq!(query::is_alphadigit(""), false);
        assert_eq!(query::is_alphadigit("bart"), true);
        assert_eq!(query::is_alphadigit("café"), true);
        assert_eq!(query::is_alphadigit("cafe\u{0301}"), true);
        assert_eq!(query::is_alphadigit("T1000"), true);
        assert_eq!(query::is_alphadigit("1000"), true);
        assert_eq!(query::is_alphadigit("10-00"), false);
        assert_eq!(query::is_alphadigit("\n\t"), false);
        assert_eq!(query::is_alphadigit("lisa!"), false);
        assert_eq!(query::is_alphadigit("lisa and bart"), false);
        assert_eq!(query::is_alphadigit("Zażółć"), true);
        assert_eq!(query::is_alphadigit("Zażółć gęślą jaźń"), false);
    }
    #[test]
    fn query_is_blank() {
        assert!(query::is_blank(""), true);
        assert_eq!(query::is_blank("   "), true);
        assert_eq!(query::is_blank("\n\t\r"), true);
        assert_eq!(query::is_blank("Zażółć gęślą jaźń"), false);
    }
    #[test]
    fn query_is_digit() {
        assert!(query::is_digit(""));
        assert!(query::is_digit("0"));
        assert!(query::is_digit("100"));
        assert!(query::is_digit("100500"));
        assert_eq!(query::is_digit("1.5"), false);
        assert_eq!(query::is_digit("0xFF"), false);
        assert_eq!(query::is_digit("ten"), false);
    }
    #[test]
    fn query_is_empty() {
        assert!(query::is_empty(""), true);
        assert_eq!(query::is_empty("Zażółć gęślą jaźń"), false);
        assert_eq!(query::is_empty("the world is yours"), false);
    }
    #[test]
    fn query_is_lowercase() {
        assert!(query::is_lowercase(""), true);
        assert_eq!(query::is_lowercase("the world is yours"), true);
        assert_eq!(query::is_lowercase("Zażółć gęślą jaźń"), false);
        assert_eq!(query::is_lowercase("T1000"), false);
    }
    #[test]
    fn query_is_numeric() {
        assert!(query::is_numeric(""), true);
        assert_eq!(query::is_numeric("350"), true);
        assert_eq!(query::is_numeric("-20.5"), true);
        assert_eq!(query::is_numeric("five"), false);
        assert_eq!(query::is_numeric(".."), false);
    }
    #[test]
    fn query_is_uppercase() {
        assert!(query::is_uppercase(""), true);
        assert_eq!(query::is_uppercase("THE WORLD IS YOURS"), true);
        assert_eq!(query::is_uppercase("Zażółć gęślą jaźń"), false);
        assert_eq!(query::is_uppercase("t1000"), false);
    }
    #[test]
    fn query_starts_with() {
        assert!(query::starts_with("the world is yours", "the world"));
        assert!(query::starts_with(
            "Zażółć gęślą jaźń",
            "Zażółć"
        ));
        assert!(query::starts_with("the world is yours", ""));
        assert!(query::starts_with("", ""), true);
    }

    /// voca_rs::case testing
    #[test]
    fn case_camel_case() {
        assert_eq!(case::camel_case("The World - IS Yours"), "theWorldIsYours");
        assert_eq!(
            case::camel_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "zażółćGęśląJaźń"
        );
        assert_eq!(
            case::camel_case("say  ***    Hello\r\n   to--ME++"),
            "sayHelloToMe"
        );
        assert_eq!(case::camel_case(""), "");
    }
    #[test]
    fn case_pascal_case() {
        assert_eq!(case::pascal_case("The World - IS Yours"), "TheWorldIsYours");
        assert_eq!(
            case::pascal_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "ZażółćGęśląJaźń"
        );
        assert_eq!(
            case::pascal_case("say  ***    Hello\r\n   to--ME++"),
            "SayHelloToMe"
        );
        assert_eq!(case::pascal_case(""), "");
    }
    #[test]
    fn case_capitalize() {
        assert_eq!(
            case::capitalize("The World IS YourS", &true),
            "The world is yours"
        );
        assert_eq!(
            case::capitalize("ZAżółć GĘŚLĄ jAźń", &true),
            "Zażółć gęślą jaźń"
        );
        assert_eq!(
            case::capitalize("say Hello to ME", &false),
            "Say Hello to ME"
        );
        assert_eq!(case::capitalize("", &true), "");
    }
    #[test]
    fn case_decapitalize() {
        assert_eq!(
            case::decapitalize("The World IS YourS", &true),
            "the world is yours"
        );
        assert_eq!(
            case::decapitalize("ZAżółć GĘŚLĄ jAźń", &true),
            "zażółć gęślą jaźń"
        );
        assert_eq!(
            case::decapitalize("Say Hello to ME", &false),
            "say Hello to ME"
        );
        assert_eq!(case::decapitalize("", &true), "");
    }
    #[test]
    fn case_kebab_case() {
        assert_eq!(
            case::kebab_case("The World - IS Yours"),
            "the-world-is-yours"
        );
        assert_eq!(
            case::kebab_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "zażółć-gęślą-jaźń"
        );
        assert_eq!(
            case::kebab_case("say  ***    Hello\r\n   to--ME++"),
            "say-hello-to-me"
        );
        assert_eq!(case::kebab_case(""), "");
    }
    #[test]
    fn case_shouty_kebab_case() {
        assert_eq!(
            case::shouty_kebab_case("The World - IS Yours"),
            "THE-WORLD-IS-YOURS"
        );
        assert_eq!(
            case::shouty_kebab_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "ZAŻÓŁĆ-GĘŚLĄ-JAŹŃ"
        );
        assert_eq!(
            case::shouty_kebab_case("say  ***    Hello\r\n   to--ME++"),
            "SAY-HELLO-TO-ME"
        );
        assert_eq!(case::shouty_kebab_case(""), "");
    }
    #[test]
    fn case_lower_case() {
        assert_eq!(case::lower_case("The World IS YourS"), "the world is yours");
        assert_eq!(
            case::lower_case("Zażółć gęśLą jaźń"),
            "zażółć gęślą jaźń"
        );
        assert_eq!(case::lower_case(""), "");
    }
    #[test]
    fn case_snake_case() {
        assert_eq!(
            case::snake_case("The World - IS Yours"),
            "the_world_is_yours"
        );
        assert_eq!(
            case::snake_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "zażółć_gęślą_jaźń"
        );
        assert_eq!(
            case::snake_case("say  ***    Hello\r\n   to--ME++"),
            "say_hello_to_me"
        );
        assert_eq!(case::snake_case(""), "");
    }
    #[test]
    fn case_shouty_snake_case() {
        assert_eq!(
            case::shouty_snake_case("The World - IS Yours"),
            "THE_WORLD_IS_YOURS"
        );
        assert_eq!(
            case::shouty_snake_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "ZAŻÓŁĆ_GĘŚLĄ_JAŹŃ"
        );
        assert_eq!(
            case::shouty_snake_case("say  ***    Hello\r\n   to--ME++"),
            "SAY_HELLO_TO_ME"
        );
        assert_eq!(case::shouty_snake_case(""), "");
    }
    #[test]
    fn case_swap_case() {
        assert_eq!(
            case::swap_case("The World - IS Yours"),
            "tHE wORLD - is yOURS"
        );
        assert_eq!(
            case::swap_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "_zAŻÓŁĆ-gęślą_JAŹŃ-"
        );
        assert_eq!(
            case::swap_case("say über Hello to--ME++"),
            "SAY ÜBER hELLO TO--me++"
        );
        assert_eq!(case::swap_case(""), "");
    }
    #[test]
    fn case_upper_case() {
        assert_eq!(case::upper_case("The World IS YourS"), "THE WORLD IS YOURS");
        assert_eq!(
            case::upper_case("Zażółć gęślą jaźń"),
            "ZAŻÓŁĆ GĘŚLĄ JAŹŃ"
        );
        assert_eq!(case::upper_case(""), "");
    }
    #[test]
    fn case_title_case() {
        assert_eq!(
            case::title_case("The World - IS Yours"),
            "The World Is Yours"
        );
        assert_eq!(
            case::title_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "Zażółć Gęślą Jaźń"
        );
        assert_eq!(
            case::title_case("say über Hello to--ME++"),
            "Say Über Hello To Me"
        );
        assert_eq!(case::title_case(""), "");
    }

    /// voca_rs::manipulate testing
    #[test]
    fn manipulate_insert() {
        assert_eq!(manipulate::insert("", "", 0), "");
        assert_eq!(manipulate::insert("abc", "1", 1), "a1bc");
        assert_eq!(manipulate::insert("abc abc", ",", 3), "abc, abc");
        assert_eq!(manipulate::insert("Zażółć", "-!-", 3), "Zaż-!-ółć");
        assert_eq!(
            manipulate::insert("über\tdas\tFloß", "~~~", 6),
            "über\td~~~as\tFloß"
        );
        assert_eq!(manipulate::insert("приём", "!", 5), "приём!");
    }
    #[test]
    fn manipulate_repeat() {
        assert_eq!(manipulate::repeat("", 1), "");
        assert_eq!(manipulate::repeat("www", 0), "");
        assert_eq!(manipulate::repeat("abc", 3), "abcabcabc");
        assert_eq!(manipulate::repeat("abc abc", 3), "abc abcabc abcabc abc");
    }
    #[test]
    fn manipulate_reverse() {
        assert_eq!(manipulate::reverse(""), "");
        assert_eq!(manipulate::reverse("abc"), "cba");
        assert_eq!(manipulate::reverse("abc abc"), "cba cba");
        assert_eq!(manipulate::reverse("Zażółć"), "ćłóżaZ");
        assert_eq!(
            manipulate::reverse("über\tdas\tFloß"),
            "ßolF\tsad\trebü"
        );
        assert_eq!(manipulate::reverse("приём!"), "!мёирп");
    }
    #[test]
    fn manipulate_reverse_grapheme() {
        assert_eq!(manipulate::reverse_grapheme(""), "");
        assert_eq!(manipulate::reverse_grapheme("abc"), "cba");
        assert_eq!(manipulate::reverse_grapheme("abc abc"), "cba cba");
        assert_eq!(manipulate::reverse_grapheme("Zażółć"), "ćłóżaZ");
        assert_eq!(
            manipulate::reverse_grapheme("über\tdas\tFloß"),
            "ßolF\tsad\trebü"
        );
        assert_eq!(manipulate::reverse_grapheme("приём!"), "!мёирп");
        assert_eq!(manipulate::reverse_grapheme("café"), "éfac");
        assert_eq!(manipulate::reverse_grapheme("a̐éö̲"), "ö̲éa̐");
    }
    #[test]
    fn manipulate_splice() {
        assert_eq!(manipulate::splice("", 0, 0, ""), "");
        assert_eq!(manipulate::splice("test", 0, 0, ""), "test");
        assert_eq!(manipulate::splice("test", 0, 0, "-"), "-test");
        assert_eq!(manipulate::splice("test", 1, 0, "-"), "t-est");
        assert_eq!(manipulate::splice("test", 1, 1, "-"), "t-st");
        assert_eq!(manipulate::splice("test", 3, 0, "-"), "tes-t");
        assert_eq!(manipulate::splice("test", 4, 0, "-"), "test-");
        assert_eq!(manipulate::splice("test", 4, 1, "-"), "test-");
        assert_eq!(manipulate::splice("test", 4, 10, "-"), "test-");
        assert_eq!(manipulate::splice("test", -1, 0, "="), "tes=t");
        assert_eq!(manipulate::splice("test", -2, 1, "="), "te=t");
        assert_eq!(manipulate::splice("test", -10, 0, "."), ".test");
        assert_eq!(manipulate::splice("test", 100, 0, "."), "test.");
        assert_eq!(
            manipulate::splice("Zażółć", 6, 0, " gęślą jaźń"),
            "Zażółć gęślą jaźń"
        );
        assert_eq!(
            manipulate::splice("Zażółć gęślą", 6, 6, " jaźń"),
            "Zażółć jaźń"
        );
        assert_eq!(
            manipulate::splice("to jest błąd", 0, 7, "mój"),
            "mój błąd"
        );
        assert_eq!(
            manipulate::splice("Die Schildkröte fliegt.", -7, 0, "und Kröte "),
            "Die Schildkröte und Kröte fliegt."
        );
        assert_eq!(
            manipulate::splice("Привет", 6, 0, ", Ёлка!"),
            "Привет, Ёлка!"
        );
    }
    #[test]
    fn manipulate_trim() {
        assert_eq!(
            manipulate::trim("   The world - is yours\t   ", ""),
            "The world - is yours"
        );
        assert_eq!(
            manipulate::trim("--Zażółć gęślą jaźń---", "-"),
            "Zażółć gęślą jaźń"
        );
        assert_eq!(manipulate::trim("-~-Earth--~", "-~"), "Earth");
        assert_eq!(
            manipulate::trim("++--Die Schildkröte fliegt über das Floß.++", "+"),
            "--Die Schildkröte fliegt über das Floß."
        );
        assert_eq!(
            manipulate::trim("ё1ё2ёКак слышно, приём!ё1ё", "ёё12"),
            "Как слышно, приём!"
        );
        assert_eq!(manipulate::trim("", ""), "");
    }
    #[test]
    fn manipulate_trim_left() {
        assert_eq!(
            manipulate::trim_left("   The world - is yours\t   ", ""),
            "The world - is yours\t   "
        );
        assert_eq!(
            manipulate::trim_left("--Zażółć gęślą jaźń---", "-"),
            "Zażółć gęślą jaźń---"
        );
        assert_eq!(manipulate::trim_left("-~-Earth--~", "-~"), "Earth--~");
        assert_eq!(
            manipulate::trim_left("++--Die Schildkröte fliegt über das Floß.++", "+"),
            "--Die Schildkröte fliegt über das Floß.++"
        );
        assert_eq!(
            manipulate::trim_left("ё1ё2ёКак слышно, приём!ё1ё", "ёё12"),
            "Как слышно, приём!ё1ё"
        );
        assert_eq!(manipulate::trim_left("", ""), "");
    }
    #[test]
    fn manipulate_trim_right() {
        assert_eq!(
            manipulate::trim_right("   The world - is yours\t   ", ""),
            "   The world - is yours"
        );
        assert_eq!(
            manipulate::trim_right("--Zażółć gęślą jaźń---", "-"),
            "--Zażółć gęślą jaźń"
        );
        assert_eq!(manipulate::trim_right("-~-Earth--~", "-~"), "-~-Earth");
        assert_eq!(
            manipulate::trim_right("++--Die Schildkröte fliegt über das Floß.++", "+"),
            "++--Die Schildkröte fliegt über das Floß."
        );
        assert_eq!(
            manipulate::trim_right("ё1ё2ёКак слышно, приём!ё1ё", "ёё12"),
            "ё1ё2ёКак слышно, приём!"
        );
        assert_eq!(manipulate::trim_right("", ""), "");
    }

    /// voca_rs::count testing
    #[test]
    fn count_count() {
        assert_eq!(count::count(""), 0);
        assert_eq!(count::count("rain"), 4);
        assert_eq!(count::count("b\u{0142}\u{0105}d"), 4);
        assert_eq!(count::count("Die Schildkröte fliegt über das Floß."), 37);
        assert_eq!(count::count("Как слышно, приём!"), 18);
    }
    #[test]
    fn count_count_graphemes() {
        assert_eq!(count::count_graphemes(""), 0);
        assert_eq!(count::count_graphemes("rain"), 4);
        assert_eq!(count::count_graphemes("b\u{0142}\u{0105}d"), 4);
        assert_eq!(count::count_graphemes("błąd"), 4);
        assert_eq!(count::count_graphemes("a̐éö̲"), 3);
        assert_eq!(
            count::count_graphemes("Die Schildkröte fliegt über das Floß."),
            37
        );
        assert_eq!(count::count_graphemes("cafe\u{0301}"), 4);
    }
    #[test]
    fn count_count_substrings() {
        assert_eq!(count::count_substrings("", ""), 0);
        assert_eq!(count::count_substrings("******", "*"), 6);
        assert_eq!(count::count_substrings("******", "**"), 3);
        assert_eq!(count::count_substrings("******", "**-"), 0);
        assert_eq!(count::count_substrings("abc", ""), 0);
        assert_eq!(count::count_substrings("rain", "rain"), 1);
        assert_eq!(
            count::count_substrings("Die Schildkröte fliegt über das Floß.", "über"),
            1
        );
        assert_eq!(
            count::count_substrings("bad boys, bad boys whatcha gonna do?", "boys"),
            2
        );
        assert_eq!(count::count_substrings("Cafe\u{0301} del Mar", "Café"), 1);
        assert_eq!(
            count::count_substrings("Cafe\u{0301} del Mar Café del Mar cafe\u{0301}", "Café"),
            2
        );
        assert_eq!(count::count_substrings("every dog has its day", "cat"), 0);
    }
    #[test]
    fn count_count_words() {
        assert_eq!(count::count_words("", ""), 0);
        assert_eq!(count::count_words("ab c", ""), 2);
        assert_eq!(count::count_words("Gravity - can cross dimensions!", ""), 4);
        assert_eq!(count::count_words("GravityCanCrossDimensions", ""), 4);
        assert_eq!(
            count::count_words("Cafe\u{0301}-del-Mar-andBossaNova1", "-"),
            4
        );
        assert_eq!(count::count_words("Język /polski wywodzi się z` języka` praindoeuropejskiego za**pośrednictwem+języka-prasłowiańskiego.", ""), 11);
        assert_eq!(
            count::count_words("Στις--αρχές** (του) 21ου, αιώνα!'", ""),
            5
        );
        assert_eq!(
            count::count_words("Гравитация-Притягивает-ВСЕ!!", "-"),
            3
        );
    }

    /// voca_rs::chop testing
    #[test]
    fn chop_char_at() {
        assert_eq!(chop::char_at("", 0), "");
        assert_eq!(chop::char_at("rain", 0), "r");
        assert_eq!(chop::char_at("b\u{0142}\u{0105}d", 2), "ą");
        assert_eq!(
            chop::char_at("Die Schildkröte fliegt über das Floß.", 12),
            "ö"
        );
        assert_eq!(chop::char_at("Как слышно, приём!", 15), "ё");
    }
    #[test]
    fn chop_first() {
        assert_eq!(chop::first("", 0), "");
        assert_eq!(chop::first("a", 0), "");
        assert_eq!(chop::first("rain", 2), "ra");
        assert_eq!(chop::first("b\u{0142}\u{0105}d", 3), "błą");
        assert_eq!(chop::first("über das Floß.", 1), "ü");
        assert_eq!(chop::first("Как слышно, приём!", 3), "Как");
        assert_eq!(chop::first("e\u{0301}", 1), "e");
    }
    #[test]
    fn chop_grapheme_at() {
        assert_eq!(chop::grapheme_at("", 0), "");
        assert_eq!(chop::grapheme_at("b\u{0142}\u{0105}d", 1), "ł");
        assert_eq!(chop::grapheme_at("cafe\u{0301}", 3), "é");
        assert_eq!(chop::grapheme_at("über das Floß.", 0), "ü");
        assert_eq!(chop::grapheme_at("a̐éö̲", 0), "a̐");
    }
    #[test]
    fn chop_last() {
        assert_eq!(chop::last("", 0), "");
        assert_eq!(chop::last("a", 0), "");
        assert_eq!(chop::last("b\u{0142}\u{0105}d", 2), "ąd");
        assert_eq!(chop::last("helicopter", 1), "r");
        assert_eq!(chop::last("über das Floß.", 2), "ß.");
        assert_eq!(chop::last("e\u{0301}", 1), "\u{0301}");
    }
    #[test]
    fn chop_prune() {
        assert_eq!(chop::prune("", 0, ""), "");
        assert_eq!(chop::prune("Once upon a time", 7, ""), "Once...");
        assert_eq!(
            chop::prune("Die Schildkröte fliegt über das Floß.", 19, "~~"),
            "Die Schildkröte~~"
        );
        assert_eq!(chop::prune("once upon", 10, ""), "once upon");
        assert_eq!(
            chop::prune("Как слышно, приём!", 14, ""),
            "Как слышно..."
        );
    }
    #[test]
    fn chop_slice() {
        assert_eq!(chop::slice("", 0, 0), "");
        assert_eq!(chop::slice("helicopter", 1, 0), "elicopter");
        assert_eq!(chop::slice("b\u{0142}\u{0105}d", -2, 0), "ąd");
        assert_eq!(
            chop::slice("Die Schildkröte fliegt.", 4, -8),
            "Schildkröte"
        );
        assert_eq!(chop::slice("e\u{0301}", -1, 0), "\u{0301}");
    }
    #[test]
    fn chop_substr() {
        assert_eq!(chop::substr("", 0, 0), "");
        assert_eq!(chop::substr("helicopter", 1, 0), "elicopter");
        assert_eq!(chop::substr("b\u{0142}\u{0105}d", 1, 2), "łą");
        assert_eq!(chop::substr("über das Floß.", 9, 4), "Floß");
        assert_eq!(chop::substr("e\u{0301}", 1, 0), "\u{0301}");
    }
    #[test]
    fn chop_substring() {
        assert_eq!(chop::substring("", 0, 0), "");
        assert_eq!(chop::substring("helicopter", 1, 0), "elicopter");
        assert_eq!(chop::substring("b\u{0142}\u{0105}d", 2, 4), "ąd");
        assert_eq!(chop::substring("über das Floß.", 0, 1), "ü");
        assert_eq!(chop::substring("e\u{0301}", 1, 0), "\u{0301}");
    }
    #[test]
    fn chop_truncate() {
        assert_eq!(chop::truncate("", 0, ""), "");
        assert_eq!(chop::truncate("Once upon a time", 7, ""), "Once...");
        assert_eq!(
            chop::truncate("Die Schildkröte fliegt über das Floß.", 28, "(...)"),
            "Die Schildkröte fliegt (...)"
        );
        assert_eq!(chop::truncate("Once upon", 10, ""), "Once upon");
        assert_eq!(
            chop::truncate("Как слышно, приём!", 13, ""),
            "Как слышно..."
        );
    }

    /// voca_rs::index testing
    #[test]
    fn index_index_of() {
        assert_eq!(index::index_of("", "", 0), 0);
        assert_eq!(index::index_of("rain", "r", 0), 0);
        assert_eq!(index::index_of("rain", "a", 0), 1);
        assert_eq!(index::index_of("Rain, dear rain", "ear", 0), 7);
        assert_eq!(index::index_of("Rain, dear rain", "ain", 0), 1);
        assert_eq!(index::index_of("rain", "z", 0), -1);
        assert_eq!(index::index_of("b\u{0142}\u{0105}d", "ą", 0), 2);
        assert_eq!(
            index::index_of("Zażółć gęślą jaźń", "gęślą", 0),
            7
        );
        assert_eq!(
            index::index_of(
                "Die Schildkröte fliegt über das Floß.",
                "Schildkröte",
                4
            ),
            0
        );
        assert_eq!(
            index::index_of("Как слышно, приём!", "слышно", 0),
            4
        );
    }
    #[test]
    fn index_last_index_of() {
        assert_eq!(index::last_index_of("", "", 0), 0);
        assert_eq!(index::last_index_of("rain", "r", 0), 0);
        assert_eq!(index::last_index_of("rain", "a", 0), 1);
        assert_eq!(index::last_index_of("Rain, dear rain", "rain", 0), 11);
        assert_eq!(index::last_index_of("Rain, dear rain", "ain", 0), 12);
        assert_eq!(index::last_index_of("rain", "z", 0), -1);
        assert_eq!(index::last_index_of("b\u{0142}\u{0105}d", "ą", 0), 2);
        assert_eq!(
            index::last_index_of("Zażółć gęślą jaźń", "gęślą", 0),
            7
        );
        assert_eq!(
            index::last_index_of(
                "Die Schildkröte fliegt über das Floß.",
                "Schildkröte",
                4
            ),
            0
        );
        assert_eq!(
            index::last_index_of("Как слышно, приём!", "слышно", 0),
            4
        );
    }

    /// voca_rs::escape testing
    #[test]
    fn escape_escape_html() {
        assert_eq!(escape::escape_html(""), "");
        assert_eq!(
            escape::escape_html("<>&\"'`"),
            "&lt;&gt;&amp;&quot;&#x27;&#x60;"
        );
        assert_eq!(
            escape::escape_html(utils::PUNCTUATION),
            "!&quot;#$%&amp;&#x27;()*+,-./:;&lt;=&gt;?@[\\]^_&#x60;{|}~"
        );
        assert_eq!(
            escape::escape_html("<p>wonderful world</p>"),
            "&lt;p&gt;wonderful world&lt;/p&gt;"
        );
        assert_eq!(escape::escape_html("<span>"), "&lt;span&gt;");
    }
    #[test]
    fn escape_escape_regexp() {
        assert_eq!(escape::escape_regexp(""), "");
        assert_eq!(
            escape::escape_regexp("(hours)[minutes]{seconds}"),
            "\\(hours\\)\\[minutes\\]\\{seconds\\}"
        );
        assert_eq!(
            escape::escape_regexp("-[]/{}()*+?.\\^$|"),
            "\\-\\[\\]\\/\\{\\}\\(\\)\\*\\+\\?\\.\\\\\\^\\$\\|"
        );
    }
    #[test]
    fn escape_unescape_html() {
        assert_eq!(escape::escape_html(""), "");
        assert_eq!(
            escape::unescape_html("&lt;&gt;&amp;&quot;&#x27;&#x60;"),
            "<>&\"'`"
        );
        assert_eq!(
            escape::unescape_html("!&quot;#$%&amp;&#x27;()*+,-./:;&lt;=&gt;?@[\\]^_&#x60;{|}~"),
            utils::PUNCTUATION
        );
        assert_eq!(
            escape::unescape_html("&lt;p&gt;wonderful world&lt;/p&gt;"),
            "<p>wonderful world</p>"
        );
        assert_eq!(escape::unescape_html("&lt;span&gt;"), "<span>");
        assert_eq!(
            escape::unescape_html("&lt;p&gt;wonderful&lt;span&gt;world&lt;span/&gt;&lt;/p&gt;"),
            "<p>wonderful<span>world<span/></p>"
        );
    }
}
