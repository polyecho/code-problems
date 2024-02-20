# https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
# NeetCode Explanation: https://youtu.be/YUF3_eBdzsk


from collections import Counter


class Solution:
    def findLeastNumOfUniqueInts(self, arr: list[int], k: int) -> int:
        freq_counter = Counter(arr)
        res = len(freq_counter)

        # index = frequency in "arr", value = count
        freq_list = [0] * (len(arr) + 1)
        for _, freq in freq_counter.items():
            freq_list[freq] += 1

        # 0 is not going to be in "arr".
        for freq_counter in range(1, len(arr) + 1):
            removing_count = freq_list[freq_counter]

            if k >= freq_counter * removing_count:
                k -= freq_counter * removing_count
                res -= removing_count
            else:
                removing_count = k // freq_counter
                res -= removing_count
                break

        return res
