#  Given the root node of a binary tree, determine if it is a binary search tree.
import unittest

class Node:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def in_order(root: Node, ordered_nodes=[]) -> list:
    if not root:
        return ordered_nodes
    in_order(root.left, ordered_nodes)
    ordered_nodes.append(root.data)
    in_order(root.right, ordered_nodes)

    return ordered_nodes

def checkBST(root: Node):
    ordered_nodes = in_order(root)
    nodes_len = len(ordered_nodes)
    for i in range(nodes_len):
        if i + 1 == nodes_len:
            continue
        if not (ordered_nodes[i] < ordered_nodes[i+1]):
            return False

    return True


class TestCheckBST(unittest.TestCase):
    def test_in_order(self):
        node1 = Node(5)
        node2 = Node(3)
        node3 = Node(2)
        node4 = Node(12)

        node1.left = node2
        node2.left = node3
        node1.right = node4

        self.assertTrue(checkBST(node1))

