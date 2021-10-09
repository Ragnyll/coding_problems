// Given the head of a singly linked list, reverse the list, and return the reversed list.
use std::collections::VecDeque;

#[allow(dead_code)]
pub fn reverse_list(ll: VecDeque<i32>) -> VecDeque<i32> {
    let mut reversed = VecDeque::new();
    for item in ll {
        reversed.push_front(item);
    }

    reversed
}

#[cfg(test)]
pub mod test {
    use std::collections::VecDeque;
    use super::reverse_list;

    #[test]
    fn test1() {
        // Input: head = [1,2,3,4,5]
        let mut test = VecDeque::new();
        test.push_back(1);
        test.push_back(2);
        test.push_back(3);
        test.push_back(4);
        test.push_back(5);
        // Output: [5,4,3,2,1]
        let mut expected = VecDeque::new();
        expected.push_back(5);
        expected.push_back(4);
        expected.push_back(3);
        expected.push_back(2);
        expected.push_back(1);

        assert_eq!(expected, reverse_list(test));
    }

    #[test]
    fn test2() {
        // Input: head = [1,2]
        let mut test = VecDeque::new();
        test.push_back(1);
        test.push_back(2);
        // Output: [2,1]
        let mut expected = VecDeque::new();
        expected.push_back(2);
        expected.push_back(1);

        assert_eq!(expected, reverse_list(test));
    }

    #[test]
    fn test3() {
        // Input: head = []
        // Output: []
        assert_eq!(VecDeque::new(), reverse_list(VecDeque::new()));
    }
}

