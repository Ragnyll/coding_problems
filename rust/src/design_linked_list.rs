// Design your implementation of the linked list. You can choose to use a singly or doubly linked list.
// A node in a singly linked list should have two attributes: val and next. val is the value of the current node, and next is a pointer/reference to the next node.
// If you want to use the doubly linked list, you will need one more attribute prev to indicate the previous node in the linked list. Assume all nodes in the linked list are 0-indexed.
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

type RNode = Rc<RefCell<Node>>;
type OptionalNode = Option<RNode>;

#[derive(Debug)]
struct Node {
    val: i32,
    next: OptionalNode,
}

impl Node {
    #[allow(dead_code)]
    fn new(val: i32) -> RNode {
        Rc::new(RefCell::new(Node { val, next: None }))
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node:{}", self.val)
    }
}

#[derive(Debug)]
struct NodeIterator {
    curr: OptionalNode,
}

impl NodeIterator {
    fn new(start: OptionalNode) -> Self {
        NodeIterator { curr: start }
    }
}

impl Iterator for NodeIterator {
    type Item = RNode;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = &self.curr;
        let mut res = None;

        self.curr = match curr {
            Some(ref n) => {
                res = Some(Rc::clone(n));
                match &n.borrow().next {
                    Some(next_n) => Some(Rc::clone(next_n)),
                    _ => None,
                }
            }
            _ => None,
        };

        res
    }
}

#[derive(Debug)]

/// Singly linked list impl
struct MyLinkedList {
    size: u32,
    head: OptionalNode,
}

impl fmt::Display for MyLinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut nodes = self.get_node_iterator().peekable();
        write!(f, "MyLinkedList: Size = {} data = [", self.size);
        while let Some(node) = nodes.next() {
            if nodes.peek().is_some() {
                write!(f, "{}, ", node.borrow().val);
            } else {
                write!(f, "{}", node.borrow().val);
            }
        }
        write!(f, "]")
    }
}

impl MyLinkedList {
    /// Initializes a new linked list
    #[allow(dead_code)]
    fn new() -> Self {
        MyLinkedList {
            size: 0,
            head: None,
        }
    }

    fn get_node_iterator(&self) -> NodeIterator {
        match &self.head {
            Some(head) => NodeIterator::new(Some(Rc::clone(head))),
            _ => NodeIterator::new(None),
        }
    }

    /// Return the val of node at index
    #[allow(dead_code)]
    fn get(&self, index: i32) -> i32 {
        let mut node_iterator = self.get_node_iterator();
        for _ in 0..index {
            node_iterator.next();
        }
        node_iterator.next().unwrap().borrow().val
    }

    #[allow(dead_code)]
    fn add_at_head(&mut self, val: i32) {
        let new_head = Node::new(val);
        match self.head.take() {
            Some(old) => {
                new_head.borrow_mut().next = Some(Rc::clone(&old));
            }
            None => {}
        }

        self.head = Some(new_head);
        self.size += 1;
    }

    #[allow(dead_code)]
    fn add_at_tail(&mut self, val: i32) {
        if self.size == 0 {
            return self.add_at_head(val);
        }

        let mut node_iterator = self.get_node_iterator();
        for _ in 0..self.size - 1 {
            node_iterator.next();
        }
        node_iterator.next().unwrap().borrow_mut().next = Some(Node::new(val));
        self.size += 1;
    }
}

#[cfg(test)]
mod test {
    use super::MyLinkedList;

    #[test]
    fn test_add_at_head() {
        let mut ll = MyLinkedList::new();
        assert_eq!(ll.size, 0);
        ll.add_at_head(1);
        assert_eq!(ll.size, 1);
        ll.add_at_head(2);
        assert_eq!(ll.size, 2);
    }

    #[test]
    fn test_get() {
        let mut ll = MyLinkedList::new();
        ll.add_at_head(1);
        ll.add_at_head(2);
        assert_eq!(ll.get(1), 1);
        assert_eq!(ll.get(0), 2);
    }

    #[test]
    fn test_add_at_tail() {
        let mut ll = MyLinkedList::new();
        ll.add_at_tail(1);
        ll.add_at_tail(2);
        assert_eq!(ll.get(0), 1);
        assert_eq!(ll.get(1), 2);
    }
}
