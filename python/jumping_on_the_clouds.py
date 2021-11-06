#  There is a new mobile game that starts with consecutively numbered clouds. Some of the clouds are thunderheads and others are cumulus. The player can jump on any cumulus cloud having a number that is equal to the number of the current cloud plus or

#  . The player must avoid the thunderheads. Determine the minimum number of jumps it will take to jump from the starting postion to the last cloud. It is always possible to win the game.

#  For each game, you will get an array of clouds numbered
#  if they are safe or if they must be avoided.
import unittest


SAFE_CLOUD = 0

def jumping_on_the_clouds(clouds: list[int]) -> int:
    """jumping_on_the_clouds: determines the minimum number it takes to jump from the starting position to the last cloud.

    :param clouds: an array representing if a cloud is jumpable (0) or impassable (1)
    :type clouds: list[int]
    :rtype: int
    """
    # when there are no clouds or 1 cloud it takes no jumps
    if len(clouds) <= 1:
        return 0

    current_cloud = 0
    num_jumps = 0

    while current_cloud != len(clouds) - 1:
        # always try two jumps first
        if clouds[current_cloud + 2] == SAFE_CLOUD:
            current_cloud += 2
        else:
            current_cloud += 1

        num_jumps += 1

    return num_jumps


class TestJumpingOnTheClouds(unittest.TestCase):
    def test_jump(self):
        self.assertEqual(jumping_on_the_clouds([0, 0, 0, 0, 1, 0]), 3)
        self.assertEqual(jumping_on_the_clouds([0, 0, 1, 0, 0, 1, 0]), 4)

