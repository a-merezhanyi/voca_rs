extern crate voca_rs;

#[cfg(test)]
mod tests {
    use voca_rs::*;

    /// voca_rs::utils testing
    #[test]
    fn utils_version() {
        assert_eq!(utils::version(), "0.1.0");
    }

    /// voca_rs::split testing
    #[test]
    fn split_to_chars() {
        assert_eq!(split::chars("gravity"), ["g", "r", "a", "v", "i", "t", "y"]);
        assert_eq!(split::chars("  "), [" ", " "]);
        assert_eq!(split::chars("a b"), ["a", " ", "b"]);
        assert_eq!(split::chars("\n\t"), ["\n", "\t"]);
        assert_eq!(split::chars(""), [""]);
    }
    #[test]
    #[should_panic]
    fn split_to_chars_panic() {
        assert_eq!(split::chars("gravity"), ["g", "r", "a"]);
    }

    #[test]
    fn split_by_pattern() {
        assert_eq!(
            split::split("gravity can cross dimensions", " "),
            ["gravity", "can", "cross", "dimensions"]
        );
        assert_eq!(split::split("*dying*star*", "*"), ["", "dying", "star"]);
        assert_eq!(split::split("dying star", ""), ["dying star"]);
        assert_eq!(split::split("", ""), [""]);
    }
    #[test]
    #[should_panic]
    fn split_by_pattern_panic() {
        assert_eq!(split::chars("gravity"), ["g", "r", "a"]);
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
    #[should_panic]
    fn split_words_panic() {
        assert_eq!(
            split::words("gravity can cross dimensions"),
            ["gravity1", "can", "cross", "dimensions"]
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
    #[test]
    #[should_panic]
    fn split_to_graphemes_panic() {
        assert_eq!(split::graphemes("\r"), ["r"]);
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
    #[should_panic]
    fn query_dont_ends_with() {
        assert!(query::ends_with("a b c", "b"));
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
    #[test]
    #[should_panic]
    fn query_dont_starts_with() {
        assert!(query::starts_with("a b c", "b"));
    }

    /// voca_rs::case testing
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
    #[should_panic]
    fn case_capitalize_panic() {
        assert_eq!(case::capitalize("ABC", &true), "ABC");
    }

    #[test]
    fn case_lower_case() {
        assert_eq!(case::lower_case("The World IS YourS"), "the world is yours");
        assert_eq!(
            case::lower_case("Zażółć gęślą jaźń"),
            "zażółć gęślą jaźń"
        );
        assert_eq!(case::lower_case(""), "");
    }
    #[test]
    #[should_panic]
    fn case_lower_case_panic() {
        assert_eq!(case::lower_case("ABC"), "ABC");
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
    #[should_panic]
    fn case_upper_case_panic() {
        assert_eq!(case::upper_case("ABC"), "abc");
    }
}
