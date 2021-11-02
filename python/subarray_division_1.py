#  Two children, Lily and Ron, want to share a chocolate bar. Each of the squares has an integer on it.

#  Lily decides to share a contiguous segment of the bar selected such that:

#  The length of the segment matches Ron's birth month, and,
#  The sum of the integers on the squares is equal to his birth day.

#  Determine how many ways she can divide the chocolate.

def subarray_division(s: list[int], d: int, m: int) -> int:
    return sum([1 for i in range(len(s) - m + 1) if sum(s[i:i + m]) == d])
