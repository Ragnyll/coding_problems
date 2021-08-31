import unittest


def fib_recursive(n: int):
    if n < 3:
        return 1
    else:
        return fib_recursive(n - 1) + fib_recursive(n - 2)


def fib_memoized(n: int, memo: dict) -> int:
    if n < 3:
        return 1
    else:
        if n in memo:
            return memo[n]
        else:
            memo[n] = fib_memoized(n - 1, memo) + fib_memoized(n - 2, memo)
            return memo[n]

    pass


class TestFib(unittest.TestCase):
    def test_fib_recursive_base_case(self):
        self.assertEqual(fib_recursive(1), 1)
        self.assertEqual(fib_recursive(2), 1)

    def test_fib_recursive_non_base(self):
        self.assertEqual(fib_recursive(5), 5)
        # Using a recursive approach this takes a long time
        # self.assertEqual(fib_recursive(50), 12586269025)

    def test_fib_memo_base(self):
        memo = {}
        self.assertEqual(fib_memoized(1, memo), 1)
        self.assertEqual(fib_memoized(2, memo), 1)

    def test_fib_memo_non_base(self):
        memo = {}
        self.assertEqual(fib_memoized(5, memo), 5)
        memo = {}
        self.assertEqual(fib_memoized(50, memo), 12586269025)
