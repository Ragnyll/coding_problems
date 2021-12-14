// Given an array of strings words and an integer k, return the k most frequent strings.

// Return the answer sorted by the frequency from highest to lowest. Sort the words with the same frequency by their lexicographical order.
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Ordering;

struct WordPriority {
    word: String,
    occurences: u32,
}

impl WordPriority {
    /// Creates a new word priority with a single occurence
    #[allow(dead_code)]
    fn new(word: &str, occurences: u32) -> Self {
        WordPriority {
            word: String::from(word),
            occurences,
        }
    }

    #[allow(dead_code)]
    fn inc_occurences(&mut self) {
        self.occurences += 1
    }
}

impl PartialEq for WordPriority {
    fn eq(&self, other: &Self) -> bool {
        self.occurences == other.occurences && self.word.eq(&other.word)
    }
}

impl PartialOrd for WordPriority {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        if self.occurences == rhs.occurences {
            return Some(rhs.word.cmp(&self.word));
        }
        self.occurences.partial_cmp(&rhs.occurences)
    }
}
impl Eq for WordPriority {}

impl Ord for WordPriority {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.occurences.cmp(&rhs.occurences)
    }
}

#[allow(dead_code)]
fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    let num_words = words.len();
    let mut p_queue = BinaryHeap::with_capacity(num_words);
    let mut word_set = HashSet::with_capacity(num_words);

    for word in &words {
        word_set.insert(String::from(word));
    }

    for w in word_set {
        p_queue.push(WordPriority::new(
            &w,
            words.iter().filter(|&n| *n == w).count() as u32,
        ));
    }

    let mut res = vec![];
    for _ in 0..k {
        res.push(p_queue.pop().unwrap().word);
    }

    res
}

#[cfg(test)]
mod test {
    use super::{WordPriority, top_k_frequent};

    #[test]
    fn test_ordering() {
        let mut word1 = WordPriority::new(&"dog", 1);
        let mut word2 = WordPriority::new(&"cat", 1);

        // pure occurences
        word1.inc_occurences();
        assert_eq!(true, word1 > word2);

        // lexicographical ordering
        // cat comes first lexicographically so should be greater
        word2.inc_occurences();
        assert_eq!(true, word2 > word1);
    }

    #[test]
    fn test_no_duplicates() {
        let words = vec![String::from("a"), String::from("b"), String::from("c")];

        assert_eq!(
            top_k_frequent(words, 3),
            vec![String::from('a'), String::from('b'), String::from('c')]
        );
    }

    #[test]
    fn test_top_k_frequent() {
        let words = vec![
            String::from("a"),
            String::from("a"),
            String::from("a"),
            String::from("b"),
            String::from("b"),
            String::from("b"),
            String::from("b"),
            String::from("c"),
        ];

        assert_eq!(
            top_k_frequent(words, 2),
            vec![String::from('b'), String::from('a')]
        );
    }
}
