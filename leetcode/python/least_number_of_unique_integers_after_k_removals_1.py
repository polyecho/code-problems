# https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
# NeetCode Explanation: https://youtu.be/YUF3_eBdzsk


from collections import Counter
import heapq


class Solution:
    def findLeastNumOfUniqueInts(self, arr: list[int], k: int) -> int:
        freq_counter = Counter(arr)

        freq_heap = list(freq_counter.values())
        heapq.heapify(freq_heap)
        res = len(freq_heap)

        while k > 0 and freq_heap:
            freq = heapq.heappop(freq_heap)
            if k >= freq:
                k -= freq
                res -= 1

        return res
