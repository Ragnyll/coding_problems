# https://leetcode.com/problems/path-crossing/

class Solution:
    def isPathCrossing(self, path: str) -> bool:
        previous_locations = set()
        current_coordinate = (0, 0)
        previous_locations.add(current_coordinate)


        for c in path:
            if c == 'N':
                current_coordinate = (current_coordinate[0], current_coordinate[1] + 1)
            elif c == 'E':
                current_coordinate = (current_coordinate[0] + 1, current_coordinate[1])
            elif c == 'S':
                current_coordinate = (current_coordinate[0], current_coordinate[1] - 1)
            elif c == 'W':
                current_coordinate = (current_coordinate[0] - 1, current_coordinate[1])

            if current_coordinate in previous_locations:
                return True

            previous_locations.add(current_coordinate)

        return False
