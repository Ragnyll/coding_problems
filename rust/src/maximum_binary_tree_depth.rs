//Given the root of a binary tree, return its maximum depth.
//
//A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

use std::rc::Rc;
use std::cell::RefCell;

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
            right: None,
        }
    }
}

pub struct BinaryTree {
    pub root: Option<Rc<RefCell<TreeNode>>>,
}

impl BinaryTree {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> BinaryTree {
        BinaryTree { root: root }
    }
}

#[allow(dead_code)]
fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // build the tree

    // find len to longest leaf
    0
}

#[cfg(test)]
pub mod test {

    #[test]
    fn ex1() {
        // Input: root = [3,9,20,null,null,15,7]
        // Output: 3
    }

    #[test]
    fn ex2() {
        // Input: root = [1,null,2]
        // Output: 2
    }

    #[test]
    fn ex3() {
        // Input: root = []
        // Output: 0
    }

    #[test]
    fn ex4() {
        // Input: root = [0]
        // Output: 1
    }
}
