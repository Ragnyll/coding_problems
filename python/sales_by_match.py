#  There is a large pile of socks that must be paired by color. Given an array of integers representing the color of each sock, determine how many pairs of socks with matching colors there are.
from unittest import TestCase


def sales_by_match(n: int, arr: list[int]) -> int:
    sock_types = {}
    for sock in arr:
        if sock not in sock_types.keys():
            sock_types[sock] = 1
        else:
            sock_types[sock] += 1

    num_pairs = 0
    for sock_type in sock_types:
        num_pairs += sock_types[sock_type] // 2

    return num_pairs


class TestSalesByMatch(TestCase):
    def test_sales_by_match(self):
        self.assertEqual(sales_by_match(9, [10, 20, 20, 10, 10, 30, 50, 10, 20]), 3)
