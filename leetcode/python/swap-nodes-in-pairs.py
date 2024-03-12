# https://leetcode.com/problems/swap-nodes-in-pairs
# NeetCode Explanation: https://youtu.be/o811TZLAWOo

from typing import Optional


class ListNode:
    # Definition for singly-linked list.
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(0, head)
        prev, curr = dummy, head

        while curr and curr.next:
            # save ptrs
            second = curr.next  # 1
            next_pair = curr.next.next  # 3

            # reverse the pair
            second.next = curr
            curr.next = next_pair
            prev.next = second

            # update pointers
            prev = curr
            curr = next_pair

        return dummy.next
