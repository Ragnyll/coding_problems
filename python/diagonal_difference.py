#  Given a square matrix, calculate the absolute difference between the sums of its diagonals.
import unittest


def diagonal_difference(arr: list[list[int]]) -> int:
    if len(arr) == 0 or len(arr[0]) == 0:
        return 0

    l_diag = sum([arr[i][i] for i in range(len(arr))])
    r_diag = sum([arr[len(arr) - 1 - i][i] for i in range(len(arr))])

    return abs(l_diag - r_diag)


class TestDiagonalDifference(unittest.TestCase):
    def test_empty(self):
        self.assertEqual(0, diagonal_difference([[]]))
        self.assertEqual(0, diagonal_difference([]))

    def test_sample(self):
        self.assertEqual(15, diagonal_difference([[11, 12, 4], [4, 5, 6], [10, 8, -12]]))

