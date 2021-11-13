use std::collections::HashSet;
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest_str_len = 0;
    let mut seen_chars = HashSet::new();
    let mut start_index: usize = 0;
    let mut end_index: usize = 0;
    let chars: Vec<char> = s.chars().collect();

    while end_index < s.len() {
        let unseen = seen_chars.insert(chars[end_index]);
        if unseen {
            end_index += 1;
            longest_str_len = std::cmp::max(longest_str_len, seen_chars.len());
        } else {
            seen_chars.remove(&chars[start_index]);
            start_index += 1;
        }
    }

    longest_str_len as i32
}

#[cfg(test)]
mod test {
    use super::length_of_longest_substring;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring(String::from("aab")), 2);
    }

    #[test]
    fn test_length_of_longest_substring_2() {
        assert_eq!(length_of_longest_substring(String::from("dvdf")), 3);
    }
}
