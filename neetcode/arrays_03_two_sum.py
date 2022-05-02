class Solution:
    def twoSum(self, nums, target: int):
        d = {}
        for i, n in enumerate(nums):
            if target - n in d:
                return [i, d[target - n]]
            else:
                d[n] = i
