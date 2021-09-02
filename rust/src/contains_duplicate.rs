use std::collections::HashSet;

#[allow(dead_code)]
fn contains_duplicate(list: &[usize]) -> bool {
    let mut scanned_numbers = HashSet::new();
    for num in list.into_iter() {
        if scanned_numbers.contains(num) {
            return true
        }
        scanned_numbers.insert(num);
    }

    false
}

#[cfg(test)]
mod contains_duplicate_tests {
    use super::contains_duplicate;

    #[test]
    fn empty_list() {
        assert_eq!(contains_duplicate(&[]), false);
        assert_eq!(contains_duplicate(&vec![]), false);
    }

    #[test]
    fn no_duplicates() {
        assert_eq!(contains_duplicate(&[1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(&vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn one_duplicate() {
        assert_eq!(contains_duplicate(&[1, 2, 1]), true);
        assert_eq!(contains_duplicate(&vec![1, 2, 1]), true);
    }

    #[test]
    fn multiple_duplicates() {
        assert_eq!(contains_duplicate(&[1, 2, 3, 1, 2, 3]), true);
        assert_eq!(contains_duplicate(&vec![1, 2, 3, 1, 2, 3]), true);
    }
}
