"""Given an array of integers, where all elements but one occur twice, find the unique element.
"""
import unittest

def lonely_integer(arr: list[int]) -> int:
    res = 0
    for e in arr:
        res ^= e

    return res


class TestLonelyInteger(unittest.TestCase):
    def test_lonely_integer(self):
        self.assertEqual(lonely_integer([1, 1, 2, 2, 3]), 3)

