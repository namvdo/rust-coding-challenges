#[cfg(test)]
mod tests {
    use wc_command::{count_word_occurrences, count_words};

    #[test]
    fn test_count_words_empty() {
        let text = "";
        assert_eq!(count_words(text), 0)
    }

    #[test]
    fn test_count_words_single_line() {
        let text = "hello world";
        assert_eq!(count_words(text), 2);
    }

    #[test]
    fn test_count_words_multiple_lines() {
        let text = "Hello world\nThis is a test\nRust is awesome";
        assert_eq!(count_words(text), 9);
    }

    #[test]
    fn test_count_word_occurrences_empty() {
        let text = "";
        let word = "rust".to_string();
        assert_eq!(count_word_occurrences(text, word), 0);
    }

    #[test]
    fn test_count_word_occurrences_single_line() {
        let text = "performance and memory safety";
        let word = "performance".to_string();
        assert_eq!(count_word_occurrences(text, word), 1);
    }

    #[test]
    fn test_count_word_occurrences_case_sensitive() {
        let text = "Rust is performant, rust is powerful";
        let word = "rust".to_string();
        assert_eq!(count_word_occurrences(text, word), 1)
    }

    #[test]
    fn test_count_word_occurrences_multiple_occurrences() {
        let text = "Hello world\nThis is a test\nThis test is just a test";
        let word = "test".to_string();
        assert_eq!(count_word_occurrences(text, word), 3);
    }

    #[test]
    fn test_count_word_large_input() {
        let text = "hello world ".repeat(1_000_000);
        assert_eq!(count_words(&text), 2_000_000);
    }

    #[test]
    fn test_count_word_occurrences_large_input() {
        let text = "this is a test\nthis is another test\nfinally another test\n".repeat(1_000_000);
        let test = "test".to_string();
        let another = "another".to_string();
        assert_eq!(count_word_occurrences(&text, test), 3_000_000);
        assert_eq!(count_word_occurrences(&text, another), 2_000_000);
    }

    #[test]
    fn test_count_word_occurrens_one_letter_large_input() {
        let text = "a b c d e f g h i j k m n n n ".repeat(10_000_000);
        let n = "n".to_string();
        assert_eq!(count_word_occurrences(&text, n), 30_000_000);
    }

}
