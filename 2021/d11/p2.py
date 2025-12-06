data = []
with open('d11\data.txt', 'r') as f:
    data = [[int(x) for x in list(l.strip())] for l in f.readlines()]


def flash(y, x):
    data[y][x] = -666
    for a, b in [[0, 1], [1, 0], [1, 1], [0, -1], [-1, 0], [-1, -1], [-1, 1], [1, -1]]:
        if 0 <= y + a < len(data) and 0 <= x + b < len(data[0]):
            data[y+a][x+b] += 1


i = 0
while not all(all(l == 0 for l in k) for k in data):
    for y in range(len(data)):
        for x in range(len(data[0])):
            data[y][x] += 1

    while any(any(l > 9 for l in k) for k in data):
        for y in range(len(data)):
            for x in range(len(data[0])):
                if data[y][x] > 9:
                    flash(y, x)

    for y in range(len(data)):
        for x in range(len(data[0])):
            data[y][x] = max(data[y][x], 0)

    i += 1

print(i)
