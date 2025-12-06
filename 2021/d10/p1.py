data = []
with open('d10\data.txt', 'r') as f:
    data = [l.strip() for l in f.readlines()]

point_table = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137
}

brackets = {
    '(': ')',
    '[': ']',
    '{': '}',
    '<': '>'
}

score = 0
for x in data:
    stack = []
    for c in x:
        if c in brackets:
            stack.append(c)
        elif c == brackets[stack[-1]]:
            stack.pop()
        else:
            score += point_table[c]
            break

print(score)
