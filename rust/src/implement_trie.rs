// A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.

// Implement the Trie class:

// Trie() Initializes the trie object.
// void insert(String word) Inserts the string word into the trie.
// boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
// boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.
//
//
// NOTE: this is not the optimal solution. The optimal solution is to do array based checks with
// the entire alphabet
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Link {
    children: HashMap<char, Link>,
    is_final: bool,
}

impl Link {
    /// Creates an empty link
    #[allow(dead_code)]
    fn new(is_final: bool) -> Link {
        Link {
            is_final,
            children: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Trie {
    #[allow(dead_code)]
    root: Link,
}

impl Trie {
    /// Creates an empty trie with no root
    #[allow(dead_code)]
    fn new() -> Trie {
        // the root node is a value with no content that can also be the empty val
        Trie {
            root: Link {
                is_final: true,
                children: HashMap::new(),
            },
        }
    }

    #[allow(dead_code)]
    fn insert(&mut self, word: String) {
        let mut chars = word.chars().peekable();
        let mut current_link = &mut self.root;

        while let Some(c) = chars.next() {
            let is_final = !chars.peek().is_some();
            if current_link.children.contains_key(&c) {
                current_link = current_link.children.get_mut(&c).unwrap();
                if is_final {
                    current_link.is_final = true
                }
            } else {
                current_link.children.insert(c, Link::new(is_final));
                current_link = current_link.children.get_mut(&c).unwrap();
            }
        }
    }

    #[allow(dead_code)]
    fn search(&mut self, word: String) -> bool {
        if word.is_empty() {
            return true;
        }
        let mut current_link = &mut self.root;
        let chars = word.chars();
        for c in chars {
            if !current_link.children.contains_key(&c) {
                return false;
            }
            current_link = current_link.children.get_mut(&c).unwrap();
        }
        true && current_link.is_final
    }

    #[allow(dead_code)]
    fn starts_with(&mut self, prefix: String) -> bool {
        if prefix.is_empty() {
            return true;
        }
        let mut current_link = &mut self.root;
        let chars = prefix.chars();
        for c in chars {
            if !current_link.children.contains_key(&c) {
                return false;
            }
            current_link = current_link.children.get_mut(&c).unwrap();
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Trie;

    #[test]
    fn debugger() {
        let mut trie = Trie::new();
        trie.insert(String::from("a"));
        trie.insert(String::from("abc"));

        assert_eq!(trie.search(String::from("")), true);
        assert_eq!(trie.search(String::from("a")), true);
        assert_eq!(trie.search(String::from("ad")), false);
        assert_eq!(trie.search(String::from("abc")), true);

        assert_eq!(trie.starts_with(String::from("")), true);
        assert_eq!(trie.starts_with(String::from("a")), true);
        assert_eq!(trie.starts_with(String::from("ab")), true);
        assert_eq!(trie.starts_with(String::from("abc")), true);
        assert_eq!(trie.starts_with(String::from("ad")), false);
        assert_eq!(trie.starts_with(String::from("adc")), false);
    }
}
