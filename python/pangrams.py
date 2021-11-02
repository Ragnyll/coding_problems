#  A pangram is a string that contains every letter of the alphabet. Given a sentence determine whether it is a pangram in the English alphabet. Ignore case. Return either pangram or not pangram as appropriate.
import unittest


def pangram(sentence: str) -> str:
    alphabet = set(['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
                   'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'])
    if alphabet.issubset(list(sentence.upper())):
        return 'pangram'
    return 'not pangram'


class TestPangram(unittest.TestCase):
    def test_is_pangram(self):
        self.assertEqual(pangram(
            'We promptly judged antique ivory buckles for the next prize'), 'pangram')

    def test_is_not_pangram(self):
        self.assertEqual(pangram(
            'We promptly judged antique ivory buckles for the prize'), 'not pangram')
