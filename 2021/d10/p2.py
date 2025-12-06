data = []
with open('d10\data.txt', 'r') as f:
    data = [l.strip() for l in f.readlines()]

point_table = {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4
}

brackets = {
    '(': ')',
    '[': ']',
    '{': '}',
    '<': '>'
}

scores = []
for x in data:
    score = 0
    stack = []
    for c in x:
        if c in brackets:
            stack.append(c)
        elif c == brackets[stack[-1]]:
            stack.pop()
        else:
            score += point_table[c]
            break
    else:
        for s in reversed(stack):
            score = score * 5 + point_table[brackets[s]]
        scores.append(score)

scores.sort()
print(scores[len(scores) // 2])
