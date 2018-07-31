extern crate voca_rs;

#[cfg(test)]
mod tests {
    use voca_rs::*;

    #[test]
    fn split_to_chars() {
        assert_eq!(
            split::chars(&"gravity".to_string()),
            ["g", "r", "a", "v", "i", "t", "y"]
        );
        assert_eq!(split::chars(&"  ".to_string()), [" ", " "]);
        assert_eq!(split::chars("\n\t"), ["\n", "\t"]);
    }
    #[test]
    #[should_panic]
    fn split_to_chars_panic() {
        assert_eq!(split::chars(&"gravity".to_string()), ["g", "r", "a"]);
    }

    #[test]
    fn split_words() {
        assert_eq!(
            split::words(&"gravity can cross dimensions".to_string()),
            ["gravity", "can", "cross", "dimensions"]
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
