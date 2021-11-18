/// Determine how many times a is repeated in the string_to_repeat repeated over a max size
/// chars_to_consider
fn repeated_string(string_to_repeat: &str, chars_to_consider: usize) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::repeated_string;

    #[test]
    fn repeated_string_perfect_fit() {
        assert_eq!(repeated_string("aba", 6), 4)
    }

    #[test]
    fn repeated_string_full_and_partial_fit() {
        assert_eq!(repeated_string("aba", 5), 3)
    }

    #[test]
    fn repeated_string_partial_fit() {
        assert_eq!(repeated_string("aba", 2), 1)
    }

    #[test]
    #[should_panic]
    fn repeated_string_empty_string_to_repeat_should_panic() {
        repeated_string("", 6);
    }

    #[test]
    fn repeated_string_no_chars_to_consider() {
        assert_eq!(repeated_string("aba", 0), 0)
    }
}
