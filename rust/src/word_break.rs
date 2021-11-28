// Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.
// Note that the same word in the dictionary may be reused multiple times in the segmentation.

use std::collections::HashSet;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    // convert word_dict to a set for O(1) lookup
    let mut word_set = HashSet::with_capacity(word_dict.len() + 1);
    for word in word_dict {
        word_set.insert(word);
    }

    // initial state
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;

    for i in 1..s.len() + 1 {
        for j in 0..i {
            if dp[j] && word_set.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }

    return *dp.last().unwrap();
}

#[cfg(test)]
mod test {
    use super::word_break;

    #[test]
    fn leetcode_tests() {
        assert_eq!(
            word_break(
                String::from("leetcode"),
                vec![String::from("leet"), String::from("code")]
            ),
            true
        );
        assert_eq!(
            word_break(
                String::from("applepenapple"),
                vec![String::from("apple"), String::from("pen")]
            ),
            true
        );
        assert_eq!(
            word_break(
                String::from("catsandogs"),
                vec![
                    String::from("cats"),
                    String::from("dog"),
                    String::from("sand"),
                    String::from("and"),
                    String::from("cat")
                ]
            ),
            false
        );
        assert_eq!(
            word_break(
                String::from("catsandog"),
                vec![
                    String::from("cats"),
                    String::from("dog"),
                    String::from("sand"),
                    String::from("and"),
                    String::from("cat")
                ]
            ),
            false
        );

        assert_eq!(
            word_break(
                String::from("aaaaaaa"),
                vec![String::from("aaaa"), String::from("aaa")]
            ),
            true
        );
    }
}
