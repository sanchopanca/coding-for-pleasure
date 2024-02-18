from typing import List
from math import prod


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        p = prod(nums)
        result = [0] * len(nums)
        for i, val in enumerate(nums):
            result[i] = p // val
        return result
