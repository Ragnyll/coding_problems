# Permuting Two Arrays
#  https://www.hackerrank.com/challenges/one-month-preparation-kit-two-arrays/problem?h_l=interview&playlist_slugs%5B%5D=preparation-kits&playlist_slugs%5B%5D=one-month-preparation-kit&playlist_slugs%5B%5D=one-month-week-one
import unittest

def permuting_two_arrays(k: int, arr1: list[int], arr2: list[int]) -> str:
    arr1 = sorted(arr1)
    arr2 = sorted(arr2, reverse=True)
    for i, j in zip(arr1, arr2):
        if k > i + j:
            return 'NO'

    return 'YES'

class TestPermutingTwoArrays(unittest.TestCase):
    def test_permuation_yes(self):
        self.assertEqual(permuting_two_arrays(1, [0, 0, 0], [2, 2, 2]), 'YES')
        self.assertEqual(permuting_two_arrays(10, [2, 1, 3], [7, 8, 9]), 'YES')

    def test_permuation_no(self):
        self.assertEqual(permuting_two_arrays(12, [0, 0, 0], [2, 2, 2]), 'NO')
        self.assertEqual(permuting_two_arrays(12, [11, 0, 0], [2, 2, 2]), 'NO')
