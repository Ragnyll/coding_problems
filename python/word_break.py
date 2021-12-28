import unittest


def wordBreak(s: str, word_dict: list[str]) -> bool:
    word_dict_set = set(word_dict)
    dp = [False] * (len(s) + 1)
    dp[0] = True
    for right_ptr in range(1, len(s) + 1):
        for left_ptr in range(right_ptr):
            if dp[left_ptr] and s[left_ptr:right_ptr] in word_dict_set:
                dp[right_ptr] = True
                break

    return dp[-1]


class TestWordBreak(unittest.TestCase):
    def test_wordBreak(self):
        self.assertEqual(wordBreak('applepenapple', ['apple', 'pen']), True)
