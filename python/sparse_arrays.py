#  There is a collection of input strings and a collection of query strings. For each query string, determine how many times it occurs in the list of input strings. Return an array of the results.
# https://www.hackerrank.com/challenges/one-month-preparation-kit-sparse-arrays/problem?h_l=interview&playlist_slugs%5B%5D=preparation-kits&playlist_slugs%5B%5D=one-month-preparation-kit&playlist_slugs%5B%5D=one-month-week-one&h_r=next-challenge&h_v=zen

import unittest


def sparse_arrays(strings: list[str], queries: list[str]) -> list[int]:
    # convert strings to map of string by occurence
    string_occurences = {}
    for string in strings:
        if string not in string_occurences.keys():
            string_occurences[string] = 1
        else:
            string_occurences[string] += 1

    query_counts = []
    for query in queries:
        try:
            query_counts.append(string_occurences[query])
        except KeyError:
            query_counts.append(0)

    return query_counts


class TestSparseArrays(unittest.TestCase):
    def test_empty_strings_and_queries(self):
        self.assertEqual(sparse_arrays([], []), [])

    def test_empty_queries(self):
        self.assertEqual(sparse_arrays(['ba', 'ba', 'bc'], []), [])

    def test_empty_strings(self):
        self.assertEqual(sparse_arrays([], ['ba', 'ba', 'bc']), [0, 0, 0])

    def test_query_found_in_string(self):
        self.assertEqual(sparse_arrays(['ab', 'ab', 'abc'], [
                         'ab', 'abc', 'bc']), [2, 1, 0])

    def test_query_not_found_in_string(self):
        self.assertEqual(sparse_arrays(['ab', 'ab', 'abc'], ['xyz']), [0])
