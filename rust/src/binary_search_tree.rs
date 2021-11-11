use std::fmt::Debug;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct BinarySearchTree<T>
where
    T: Ord + Debug,
{
    val: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Debug,
{
    /// Create an empty binary search tree
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            val: None,
            left: None,
            right: None,
        }
    }

    /// Returns whether or not the tree contains the specified val
    pub fn contains(&self, value: &T) -> bool {
        match &self.val {
            Some(key) => match key.cmp(value) {
                Ordering::Equal => true,
                Ordering::Greater => match &self.left {
                    Some(node) => node.contains(value),
                    None => false,
                },
                Ordering::Less => match &self.right {
                    Some(node) => node.contains(value),
                    None => false,
                },
            },
            None => false,
        }
    }

    /// Given a val insert it to the tree at the first valid location
    pub fn insert(&mut self, val: T) {
        if self.val.is_none() {
            self.val = Some(val);
        } else {
            match &self.val {
                None => (),
                Some(cur) => {
                    let target_node = if val < *cur {
                        &mut self.left
                    } else {
                        &mut self.right
                    };
                    match target_node {
                        Some(ref mut node) => {
                            node.insert(val);
                        }
                        None => {
                            let mut node = BinarySearchTree::new();
                            node.insert(val);
                            *target_node = Some(Box::new(node));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::BinarySearchTree;

    #[test]
    fn test_contains() {
        let mut bst = BinarySearchTree::<u8>::new();
        bst.insert(1);
        bst.insert(2);
        println!("{:?}", bst);
        assert_eq!(bst.contains(&1), true);
        assert_eq!(bst.contains(&3), false);
    }
}
