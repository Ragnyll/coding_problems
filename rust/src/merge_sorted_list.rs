// Merge two sorted linked lists and return it as a sorted list. The list should be made by splicing together the nodes of the first two lists.

// I'm using this as a single linked list
use std::collections::VecDeque;

#[allow(dead_code)]
pub fn merge_sorted_lists(ll_1: VecDeque<i32>, ll_2: VecDeque<i32>) -> VecDeque<i32> {
    let mut merged = VecDeque::new();
    if ll_1.is_empty() {
        return ll_2;
    }

    // make a clone to pop off values as the merge occurs
    let mut ll_2_clone = ll_2.clone();

    for item in ll_1 {
        while !ll_2_clone.is_empty() && item >= ll_2_clone[0] {
            // unwrap is fine because of the is_empty check
            merged.push_back(ll_2_clone.pop_front().unwrap());
        }
        merged.push_back(item);
    }
    merged.append(&mut ll_2_clone);

    merged
}

#[cfg(test)]
pub mod test {
    use std::collections::VecDeque;
    use super::merge_sorted_lists;

    #[test]
    fn test1() {
        assert_eq!(
            VecDeque::from(vec![1, 1, 2, 3, 4, 4]),
            merge_sorted_lists(VecDeque::from(vec![1, 2, 4]), VecDeque::from(vec![1, 3, 4]))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            VecDeque::from(vec![]),
            merge_sorted_lists(VecDeque::from(vec![]), VecDeque::from(vec![]))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            VecDeque::from(vec![0]),
            merge_sorted_lists(VecDeque::from(vec![]), VecDeque::from(vec![0]))
        );
    }
}
