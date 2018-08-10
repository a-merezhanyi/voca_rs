extern crate voca_rs;

#[cfg(test)]
mod tests {
    use voca_rs::*;

    #[test]
    fn version() {
        assert_eq!(utils::version(), "0.1.0");
    }

    #[test]
    fn split_to_chars() {
        assert_eq!(
            split::chars(&"gravity".to_string()),
            ["g", "r", "a", "v", "i", "t", "y"]
        );
        assert_eq!(split::chars(&"  ".to_string()), [" ", " "]);
        assert_eq!(split::chars(&"a b".to_string()), ["a", " ", "b"]);
        assert_eq!(split::chars("\n\t"), ["\n", "\t"]);
    }
    #[test]
    #[should_panic]
    fn split_to_chars_panic() {
        assert_eq!(split::chars(&"gravity".to_string()), ["g", "r", "a"]);
    }

    #[test]
    fn split_by_pattern() {
        assert_eq!(
            split::split("gravity can cross dimensions", " "),
            ["gravity", "can", "cross", "dimensions"]
        );
        assert_eq!(split::split("*dying*star*", "*"), ["", "dying", "star"]);
        assert_eq!(split::split("dying star", ""), ["dying star"]);
    }
    #[test]
    #[should_panic]
    fn split_by_pattern_panic() {
        assert_eq!(split::chars(&"gravity".to_string()), ["g", "r", "a"]);
    }

    #[test]
    fn split_words() {
        assert_eq!(
            split::words(&"gravity can cross dimensions".to_string()),
            ["gravity", "can", "cross", "dimensions"]
        );
        assert_eq!(
            split::words(&"gravity    dying\r\nstar\tfalling".to_string()),
            ["gravity", "dying", "star", "falling"]
        );
        assert_eq!(
            split::words(&"Zażółć gęślą jaźń".to_string()),
            ["Zażółć", "gęślą", "jaźń"]
        );
    }
    #[test]
    #[should_panic]
    fn split_words_panic() {
        assert_eq!(
            split::words(&"gravity can cross dimensions".to_string()),
            ["gravity1", "can", "cross", "dimensions"]
        );
    }
}
