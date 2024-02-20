# https://leetcode.com/problems/split-array-largest-sum/description/
# NeetCode Explanation: https://youtu.be/YUF3_eBdzsk


class Solution:
    def splitArray(self, nums: list[int], k: int) -> int:
        def can_split(largest: int):
            sub_array = 0
            current_sum = 0

            for n in nums:
                current_sum += n
                if current_sum > largest:
                    sub_array += 1
                    current_sum = n
            return sub_array + 1 <= k

        l, r = max(nums), sum(nums)
        res = r

        while l <= r:
            # mid = l + ((r - l) // 2)
            mid = (l + r) // 2

            if can_split(mid):
                res = mid
                r = mid - 1
            else:
                l = mid + 1

        return res
