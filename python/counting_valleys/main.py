#!/bin/python3

line = input()
in_valley = False
current_level = 0
num_valleys = 0

for char in line:
    if char == 'U':
        current_level = current_level + 1
        if in_valley is True and current_level == 0:
            num_valleys = num_valleys + 1
            in_valley = False
    elif char == 'D':
        current_level = current_level - 1
        if current_level >= 0:
            in_valley = False
        else:
            in_valley = True

print(num_valleys)
