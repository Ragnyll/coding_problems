def heightChecker(heights: list[int]) -> int:
    expected = heights.copy()
    expected.sort()

    out_of_order = 0

    for a, b in zip(heights, expected):
        if a != b:
            out_of_order += 1

    return out_of_order
