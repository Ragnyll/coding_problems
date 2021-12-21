use std::cmp::Ordering;

/// returns if the sorted collection contains the target using a binary search
#[allow(dead_code)]
fn binary_search_recursive<T: Ord>(collection: &[T], target: T) -> bool {
    fn binary_search<T: Ord>(collection: &[T], target: T, left: usize, right: usize) -> bool {
        if left > right {
            return false;
        }

        let mid = (left + right) / 2;
        match target.cmp(&collection[mid]) {
            Ordering::Equal => return true,
            Ordering::Less => return binary_search(collection, target, left, mid - 1),
            Ordering::Greater => return binary_search(collection, target, mid + 1, right),
        };
    }

    binary_search(collection, target, 0, collection.len() - 1)
}

/// returns if the sorted collection contains the target using a binary search
#[allow(dead_code)]
fn binary_search_iterative<T: Ord>(collection: &[T], target: T) -> bool {
    let mut left = 0;
    let mut right = collection.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        match target.cmp(&collection[mid]) {
            Ordering::Equal => return true,
            Ordering::Less => right = mid - 1,
            Ordering::Greater => left = mid + 1,
        };
    }
    false
}

#[cfg(test)]
mod test {
    use super::{binary_search_recursive, binary_search_iterative};

    #[test]
    fn test_binary_search_recursive() {
        assert_eq!(binary_search_recursive(&vec![1, 2, 3, 4, 5, 7], 3), true);
        assert_eq!(binary_search_recursive(&vec![1, 2, 3, 4, 5, 7], 2), true);
        assert_eq!(binary_search_recursive(&vec![1, 2, 3, 4, 5, 7], 4), true);
        assert_eq!(binary_search_recursive(&vec![1, 2, 3, 4, 5, 7], 6), false);
    }

    #[test]
    fn test_binary_search_iterative() {
        assert_eq!(binary_search_iterative(&vec![1, 2, 3, 4, 5, 7], 3), true);
        assert_eq!(binary_search_iterative(&vec![1, 2, 3, 4, 5, 7], 2), true);
        assert_eq!(binary_search_iterative(&vec![1, 2, 3, 4, 5, 7], 4), true);
        assert_eq!(binary_search_iterative(&vec![1, 2, 3, 4, 5, 7], 6), false);
    }
}
