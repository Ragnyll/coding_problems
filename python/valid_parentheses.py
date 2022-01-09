def isValid(s: str) -> bool:
    grouping_symbols = {
        '}': '{',
        ')': '(',
        ']': '['
    }

    stack = []
    for c in s:
        if c not in grouping_symbols:
            stack.append(c)
        else:
            top = stack.pop() if stack else '@'
            if grouping_symbols[c] != top:
                return False

    return not stack
