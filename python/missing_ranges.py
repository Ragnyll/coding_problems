from unittest import TestCase


def format_missing(lower: int, upper: int) -> str:
    if lower == upper:
        return '{}'.format(lower)
    return '{}->{}'.format(lower, upper)


def find_missing_ranges(nums: list[int], lower: int, upper: int) -> list[str]:
    if not nums:
        return [format_missing(lower, upper)]

    missing_ranges = []
    if lower < nums[0]:
        # find range from lower to first num
        missing_ranges.append(format_missing(lower, nums[0] - 1))

    for i in range(len(nums)):
        # calc range between number
        if i + 1 >= len(nums):
            break
        if nums[i + 1] - nums[i] > 1:
            missing_ranges.append(format_missing(nums[i] + 1, nums[i + 1] - 1))

    if not upper == nums[len(nums) - 1]:
        missing_ranges.append(format_missing(nums[len(nums) - 1] + 1, upper))

    return missing_ranges


class TestMissingRanges(TestCase):
    def test_left_range_missing(self):
        self.assertEqual(find_missing_ranges([2, 3, 4, 5], 0, 5), ['0->1'])

    def test_right_range_missing(self):
        self.assertEqual(find_missing_ranges(
            [0, 1, 2, 3, 4, 5], 0, 7), ['6->7'])

    def test_middle_range_missing(self):
        self.assertEqual(find_missing_ranges([1, 5], 1, 5), ['2->4'])

    def test_no_ranges_missing(self):
        self.assertEqual(find_missing_ranges([1, 2, 3, 4, 5], 1, 5), [])

    def test_leetcode_test(self):
        self.assertEqual(find_missing_ranges([0, 1, 3, 50, 75], 0, 99), [
                         '2', '4->49', '51->74', '76->99'])

    def test_empty_range(self):
        self.assertEqual(find_missing_ranges([], 1, 1), ['1'])
