# https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes
# NeetCode Explanation: https://youtu.be/TLzcum7vrTc

import collections


class Solution:
    def findSmallestSetOfVertices(self, n: int, edges: list[list[int]]) -> list[int]:
        incoming = collections.defaultdict(list)

        # reference: adjacency list
        for src, dst in edges:
            incoming[dst].append(src)

        res = []
        for i in range(n):
            if not incoming[i]:
                res.append(i)

        return res
