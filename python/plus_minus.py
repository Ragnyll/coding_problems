#  Given an array of integers, calculate the ratios of its elements that are positive, negative, and zero. Print the decimal value of each fraction on a new line with places after the decimal.
import unittest


def plus_minus(arr):
    num_pos = len([i for i in arr if i > 0])
    num_neg = len([i for i in arr if i < 0])
    num_zero = len([i for i in arr if i == 0])

    pos_portion = num_pos / (num_neg + num_pos + num_zero)
    neg_portion = num_neg / (num_neg + num_pos + num_zero)
    zero_portion = num_zero / (num_neg + num_pos + num_zero)

    print(round(pos_portion, 6))
    print(round(neg_portion, 6))
    print(round(zero_portion, 6))


class TestPlusMinus(unittest.TestCase):
    def test_plus_minus(self):
        plus_minus([-4, 3, -9, 0, 4, 1])
