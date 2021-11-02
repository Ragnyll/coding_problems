# Another sorting method, the counting sort, does not require comparison. Instead, you create an integer array whose index range covers the entire range of values in your array to sort. Each time a value occurs in the original array, you increment the counter at that index. At the end, run through your counting array, printing the value of each non-zero valued index that number of times.
#  Given a list of integers, count and return the number of times each value appears as an array of integers.
import unittest
from collections import Counter


def counting_sort(arr: list[int]) -> list[int]:
    s = Counter(arr)
    return (s[i] for i in range(100))
