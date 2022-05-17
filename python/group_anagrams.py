# https://leetcode.com/problems/group-anagrams/submissions/

def count_chars_in_string(s: str) -> list[str]:
    """counts the characters in a string and returns it as a list the size of the charset + 1
    the charset size is assumed to be 26 and the 0 index is the null character
    """
    my_ord = lambda x: ord(x) - 96
    if not s:
        return '1' + ('0' * 26)

    char_counter = [0] * 27
    for c in s:
        char_counter[my_ord(c)] += 1

    return '|'.join([str(i) for i in char_counter])


class Solution:
    def groupAnagrams(self, strs: list[str]) -> list[list[str]]:
        counter_dict = {}

        for s in strs:
            cntr = count_chars_in_string(s)
            if cntr in counter_dict:
                counter_dict[cntr].append(s)
            else:
                counter_dict[cntr] = [s]

        return [counter_dict[l] for l in counter_dict]
