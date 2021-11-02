#  You will be given a list of 32 bit unsigned integers. Flip all the bits (1->0) and (0->1) and return the result as an unsigned integer.
import unittest


def flipping_bits(n: int) -> int:
    flipped = list(str(bin(n)))[2:]
    if len(flipped) < 32:
        zeroes_to_fill = 32 - len(flipped)
        for i in range(0, zeroes_to_fill):
            flipped.insert(0, '0')

    for i in range(0, len(flipped)):
        if flipped[i] == '0':
            flipped[i] = '1'
        else:
            flipped[i] = '0'

    return int('0b{}'.format(''.join(flipped)), 2)


class TestFlippingBits(unittest.TestCase):
    def test_flipping_bits(self):
        self.assertEqual(flipping_bits(2), 4294967293)
