# https://leetcode.com/problems/check-if-string-is-decomposable-into-value-equal-substrings/submissions/

class Solution:
    def isDecomposable(self, s: str) -> bool:
        def is_3_equals(s: str):
            sub = s[0:3]
            return len(set(sub)) == 1

        def is_2_equals(sub: str):
            sub = s[0:2]
            return len(set(sub)) == 1

        if len(s) < 2:
            return False

        substring_2_used = False
        while s:
            if len(s) >= 3 and is_3_equals(s):
                # remove first 3 chars of string
                s = s[3:]


            elif is_2_equals(s):
                # make sure the substring 2 hasnt been used yet
                if substring_2_used:
                    return False

                # remove first 2 chars of string
                substring_2_used = True
                s = s[2:]
            else:
                # it doesnt work, thus it cant be decomposed
                return False
        return True and substring_2_used

