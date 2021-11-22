// Given the root of a binary tree, invert the tree, and return its root.
use std::cell::RefCell;
use std::rc::Rc;
// fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // if head.is_none() {
        // return None;
    // }

    // let mut prev = None;
    // let mut current = head;
    // while let Some(mut tmp) = current.take() {
        // let next = tmp.next.take();
        // tmp.next = prev.take();
        // prev = Some(tmp);
        // current = next;
    // }

    // prev
// }


// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() { return None; }




    None
}
