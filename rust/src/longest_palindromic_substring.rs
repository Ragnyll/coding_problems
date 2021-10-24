// Given a string s, return the longest palindromic substring in s.

#[allow(dead_code)]
fn longest_palindromic_substring(candidate: &str) -> String {
    let mut longest_substr: String = String::from("");

    let candidate_chars = candidate.as_bytes();

    // loop through each center character
    for i in 0..candidate.len() {
        let mut left_ptr: usize = i;
        let mut right_ptr: usize = i;

        while candidate_chars[left_ptr] == candidate_chars[right_ptr] {
            let current_substr = String::from(&candidate[left_ptr..right_ptr]);
            println!("current substr: {:?}", current_substr);
            println!("longest pd substr: {:?}", longest_substr);
            if current_substr.len() > longest_substr.len() {
                longest_substr = current_substr.clone();
            }
            if left_ptr > 0 {
                left_ptr -= 1;
            }
            if right_ptr < candidate.len() - 1 {
                right_ptr += 1;
            }

            if left_ptr == 0 && right_ptr == candidate.len() - 1 {
                break;
            }
        }
    }

    longest_substr
}

#[cfg(test)]
pub mod test {
    use super::longest_palindromic_substring;

    #[test]
    fn test_ex1() {
        // Input: s = "babad"
        // Output: "bab"
        // Note: "aba" is also a valid answer.
        assert_eq!("bab", longest_palindromic_substring(&"babad"));
    }

    #[test]
    fn test_ex2() {
        // Input: s = "cbbd"
        // Output: "bb"
        assert_eq!("bb", longest_palindromic_substring(&"cbbd"));
    }

    #[test]
    fn test_ex3() {
        // Input: s = "a"
        // Output: "a"
        assert_eq!("a", longest_palindromic_substring("a"));
    }

    #[test]
    fn test_ex4() {
        // Input: s = "ac"
        // Output: "a"
        assert_eq!("a", longest_palindromic_substring("ac"));
    }
}
