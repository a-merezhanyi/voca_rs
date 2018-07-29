extern crate voca_rs;

#[cfg(test)]
mod tests {
    use voca_rs::*;

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
