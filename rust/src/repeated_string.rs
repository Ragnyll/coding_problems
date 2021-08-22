/// Determines how many times a is repeated in the string_to_repeat repeated over a max size
/// chars_to_consider
pub fn repeated_string(string_to_repeat: &str, chars_to_consider: usize) -> usize {
    // determine how many times the whole string will be repeated
    let full_repeats = chars_to_consider / string_to_repeat.len();
    // determine the number of leftover chars. slice the string to consider to that
    let partial_repeats = chars_to_consider % string_to_repeat.len();

    let mut repeats = 0;
    // determine how many times 'a' occurs in the whole string and multiply it by the num times it
    // is able to fully repeat
    repeats += string_to_repeat.matches('a').count() * full_repeats;
    repeats += &string_to_repeat[0..partial_repeats].matches('a').count();

    repeats
}

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
