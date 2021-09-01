import sys
import unittest


def best_time_to_buy_stock(stock_prices: list[int]) -> int:
    max_profit = 0
    min_buy_price = sys.maxsize * 2 + 1
    for price in stock_prices:
        if price < min_buy_price:
            min_buy_price = price
        elif max_profit < price - min_buy_price:
            max_profit = price - min_buy_price

    return max_profit


class TestStocks(unittest.TestCase):
    def test_max_profit_0(self):
        self.assertEqual(best_time_to_buy_stock([]), 0)
        self.assertEqual(best_time_to_buy_stock([5, 4, 3, 2, 1]), 0)

    def test_max_profit_not_0(self):
        self.assertEqual(best_time_to_buy_stock([1, 2, 3, 4, 5]), 4)
        self.assertEqual(best_time_to_buy_stock([2, 1, 3, 4, 5]), 4)
