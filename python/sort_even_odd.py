# https://leetcode.com/problems/sort-even-and-odd-indices-independently/

class Solution:
    def sortEvenOdd(self, nums: list[int]) -> list[int]:

        odd_index_nums = [num[1] for num in enumerate(nums) if num[0] % 2 != 0]
        odd_index_nums.sort(reverse=True)
        even_index_nums = [num[1] for num in enumerate(nums) if num[0] % 2 == 0]
        even_index_nums.sort()

        result = []
        for i in range(len(nums)):
            # odd index
            if i % 2 != 0:
                result.append(odd_index_nums.pop(0))
            # even_index
            else:
                result.append(even_index_nums.pop(0))

        return result
