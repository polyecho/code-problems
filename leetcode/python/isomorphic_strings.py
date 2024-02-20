# https://leetcode.com/problems/isomorphic-strings
# NeetCode Explanation: https://youtu.be/7yF-U1hLEqQ


class Solution:
    def isIsomorphic(self, s: str, t: str) -> bool:
        map_st, map_ts = {}, {}

        # for i in range(len(s)):
        #     c1, c2 = s[i], t[i]
        for c1, c2 in zip(s, t):
            if ((c1 in map_st and map_st[c1] != c2) or
                    (c2 in map_ts and map_ts[c2] != c1)):
                return False

            map_st[c1] = c2
            map_ts[c2] = c1

        print(map_st, map_ts)
        return True
