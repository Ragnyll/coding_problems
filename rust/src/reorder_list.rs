// You are given the head of a singly linked-list. The list can be represented as:

// L0 → L1 → … → Ln - 1 → Ln

// Reorder the list to be on the following form:

// L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …

// You may not modify the values in the list's nodes. Only nodes themselves may be changed.// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// given a value n runs up to that index in the linked list and returns the node. n must be a valid node
#[allow(dead_code)]
fn goto_n(head: &Option<Box<ListNode>>, n: usize) -> Option<Box<ListNode>> {
    let mut cur_node: ListNode = head.as_ref().unwrap().as_ref().clone();
    for _ in 0..n {
        cur_node = cur_node.next.unwrap().as_ref().clone();
    }

    Some(Box::new(cur_node))
}

/// finds the size of a single linked list given the head
fn find_ll_len(head: &Option<Box<ListNode>>) -> usize {
    // scan to the end to get Ln. this will tell us the size so we know when to stop iterating
    let mut cur_node: ListNode = head.as_ref().unwrap().as_ref().clone();
    let mut l_n = 1;
    while cur_node.next.is_some() {
        l_n += 1;
        cur_node = cur_node.next.unwrap().as_ref().clone();
    }

    l_n
}

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut forward_ptr: usize = 1;
    // scan to the end to get Ln. this will tell us the size so we know when to stop iterating
    let mut backward_ptr = find_ll_len(&head) - 2;

    let mut cur_node: ListNode = head.as_ref().unwrap().as_ref().clone();
    while forward_ptr != backward_ptr {

        forward_ptr += 1;
        backward_ptr += 1;
    }
}

#[cfg(test)]
mod test {
    use super::{ListNode, reorder_list, goto_n, find_ll_len};

    // creates a small ll for testing purposes
    fn create_ll() -> Option<Box<ListNode>> {
        let ln2 = ListNode::new(3);
        let mut ln1 = ListNode::new(2);
        let mut ln0 = ListNode::new(1);

        ln1.next = Some(Box::new(ln2));
        ln0.next = Some(Box::new(ln1));

        Some(Box::new(ln0))
    }

    #[test]
    fn test_goto_n() {
        let ll0 = create_ll();

        let ll1 = ll0.clone();
        let ll2 = ll0.clone();

        assert_eq!(goto_n(&ll0, 2).unwrap().val, 3);
        assert_eq!(goto_n(&ll1, 1).unwrap().val, 2);
        assert_eq!(goto_n(&ll2, 0).unwrap().val, 1);
    }

    #[test]
    fn test_find_ll_len() {
        let ll0 = create_ll();

        assert_eq!(find_ll_len(&ll0), 3);
    }

    #[test]
    fn test_reorder_list() {
        let mut ll = create_ll();

        reorder_list(&mut ll);
        assert_eq!(1, 2);
    }
}
