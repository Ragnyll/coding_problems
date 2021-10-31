# https://www.hackerrank.com/challenges/climbing-the-leaderboard/problem
#  An arcade game player wants to climb to the top of the leaderboard and track their ranking. The game uses Dense Ranking, so its leaderboard works like this:

#  The player with the highest score is ranked number

#  on the leaderboard.
#  Players who have equal scores receive the same ranking number, and the next player(s) receive the immediately following ranking number.
import unittest


def climbing_leaderboard(ranked, player):
    rank_after_each_game = []
    for score in player:
        ranked.append(score)
        ranked_set = set(ranked)
        ranked_set.add(score)
        ranked_list = sorted(ranked_set)
        rank_after_each_game.append(ranked_list[::-1].index(score) + 1)

    return rank_after_each_game


class TestClimbingLeaderboard(unittest.TestCase):
    def test_climbing_leaderboard_empty(self):
        self.assertEqual(climbing_leaderboard([], []), [])

    def test_climbing_leaderboard_no_ranked(self):
        self.assertEqual(climbing_leaderboard([], [1, 2, 3]), [1, 1, 1])

    def test_climbing_leaderboard_no_players(self):
        self.assertEqual(climbing_leaderboard([100, 200, 300], []), [])

    def test_climbing_leaderboard(self):
        self.assertEqual(climbing_leaderboard(
            [100, 100, 50, 40, 40, 20, 10], [5, 25, 50, 120]), [6, 4, 2, 1])
