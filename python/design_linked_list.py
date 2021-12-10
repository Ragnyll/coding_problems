import unittest

class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


class MyLinkedList:

    def __init__(self):
        self.size = 0
        self.head = None

    def get(self, index: int) -> int:
        if index > self.size or index < 0:
            return -1

        if not self.head:
            return -1

        curr = self.head
        for _ in range(index):
            curr = curr.next

        return curr.val

    def addAtHead(self, val: int) -> None:
        new_head = Node(val)
        new_head.next = self.head
        self.head = new_head
        self.size += 1

    def addAtTail(self, val: int) -> None:
        if not self.head:
            self.head = Node(val)
            self.size += 1
            return

        curr = self.head
        for _ in range(self.size - 1):
            curr = curr.next

        curr.next = Node(val)
        self.size += 1

    def addAtIndex(self, index: int, val: int) -> None:
        if not self.head:
            self.addAtHead(val)
            return

        curr = self.head
        for _ in range(index - 1):
            curr = curr.next

        # connect the new node to the curr.next
        new_node = Node(val)
        new_node.next = curr.next
        curr.next = new_node
        self.size += 1

    def deleteAtIndex(self, index: int) -> None:
        if index < 0 or index >= self.size:
            return

        if index == 0:
            self.head = self.head.next
            self.size -= 1
            return

        curr = self.head
        for _ in range(index - 1):
            curr = curr.next

        curr.next = curr.next.next
        self.size -= 1


class TestLinkedList(unittest.TestCase):
    def test_addAtTail(self):
        ll = MyLinkedList()
        ll.addAtTail(3)
        self.assertTrue(ll.size == 1)
        self.assertTrue(ll.get(0) == 3)
        ll.addAtTail(2)
        self.assertTrue(ll.size == 2)
        self.assertTrue(ll.get(1) == 2)

    def test_addAtIndex(self):
        ll = MyLinkedList()
        ll.addAtTail(1)
        ll.addAtTail(2)
        ll.addAtTail(3)
        ll.addAtIndex(1, 4)
        self.assertEqual(ll.get(1), 4)
        self.assertTrue(ll.size == 4)

    def test_deleteAtIndex(self):
        ll = MyLinkedList()
        ll.addAtTail(1)
        ll.addAtTail(2)
        ll.addAtTail(3)
        ll.deleteAtIndex(1)
        self.assertEqual(ll.get(0), 1)
        self.assertEqual(ll.get(1), 3)
