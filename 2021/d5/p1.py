from dataclasses import dataclass


@dataclass
class point:
    x: int
    y: int


data = []
with open('d5\data.txt', 'r') as f:
    for line in f:
        l, r = line.split(' -> ')
        start = point(int(l.split(',')[0]), int(l.split(',')[1]))
        end = point(int(r.split(',')[0]), int(r.split(',')[1]))

        if start.x == end.x or start.y == end.y:
            if start.x < end.x or start.y < end.y:
                data.append((start, end))
            else:
                data.append((end, start))


diagram = [[0 for _ in range(1000)] for _ in range(1000)]

for start, end in data:
    for y in range(start.y, end.y + 1):
        for x in range(start.x, end.x + 1):
            diagram[y][x] += 1

ans = 0
for y in diagram:
    for x in y:
        if x >= 2:
            ans += 1
print(ans)
