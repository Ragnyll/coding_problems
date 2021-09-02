import unittest


def contains_duplicate(list_to_check: list[int]) -> bool:
    scanned_numbers = set()
    for num in list_to_check:
        if num in scanned_numbers:
            return True
        scanned_numbers.add(num)

    return False


class TestFib(unittest.TestCase):
    def empty_list(self):
        self.assertEqual(contains_duplicate([]), False)
        self.assertEqual(contains_duplicate([]), False)

    def no_duplicates(self):
        self.assertEqual(contains_duplicate([1, 2, 3, 4]), False)
        self.assertEqual(contains_duplicate([1, 2, 3, 4]), False)

    def one_duplicate(self):
        self.assertEqual(contains_duplicate([1, 2, 1]), True)
        self.assertEqual(contains_duplicate([1, 2, 1]), True)

    def multiple_duplicates(self):
        self.assertEqual(contains_duplicate([1, 2, 3, 1, 2, 3]), True)
        self.assertEqual(contains_duplicate([1, 2, 3, 1, 2, 3]), True)
