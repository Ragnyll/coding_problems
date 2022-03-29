# https://leetcode.com/problems/find-the-difference

from collections import Counter

class Solution:
    def findTheDifference(self, s: str, t: str) -> str:
        counter_s = Counter(s)
        counter_t = Counter(t)

        # there should only be one key based off the problem statement
        for k in (counter_t - counter_s).keys():
            return k
