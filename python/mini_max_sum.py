#  Given five positive integers, find the minimum and maximum values that can be calculated by summing exactly four of the five integers. Then print the respective minimum and maximum values as a single line of two space-separated long integers.
import unittest


def mini_max_sum(arr: list[int]) -> None:
    """given an arr of len 5 finds the min using 4 vals and the max using 4 values

    param arr: array of 5 values

    returns None: prints the the values
    """
    arr = sorted(arr)

    print('{} {}'.format(sum(arr[:-1]), sum(arr[1:])))


class TestMiniMaxSum(unittest.TestCase):
    def test_preordered(self):
        mini_max_sum([1, 2, 3, 4, 5])
