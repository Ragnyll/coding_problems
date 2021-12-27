import unittest


def maxArea(bars: list[int]) -> int:
    def calculate_area(l, r, b): return min(b[l], b[r]) * (r - l)
    max_area = calculate_area(0, len(bars) - 1, bars)
    left = 0
    right = len(bars) - 1
    max_h = max(bars)

    while left != right:
        if max_h * (right - left) < max_area:
            break
        max_area = max(max_area, min(bars[right], bars[left]) * abs(right - left))
        if bars[left] < bars[right]:
            left += 1
        else:
            right -= 1

    return max_area



class TestMaxArea(unittest.TestCase):

    def test_maxArea(self):
        self.assertEqual(maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7]), 49)
