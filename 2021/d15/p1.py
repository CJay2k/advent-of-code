data = []
with open('d15\data.txt', 'r') as f:
    data = [list(map(int, l.strip())) for l in f]


min_risk = sum(data[i][i] + data[i][i+1] for i in range(len(data) - 1))


def find_path(data, y=0, x=0, risk=0):
    global min_risk
    if y < 0 or y > len(data) - 1:
        return
    if x < 0 or x > len(data[0]) - 1:
        return
    if data[y][x] < 0:
        return
    if risk + len(data) - y + len(data[0]) - x > min_risk:
        return

    risk += data[y][x]
    if y == len(data) - 1 and x == len(data[0]) - 1:
        min_risk = min(min_risk, risk)
        print(min_risk)
        return

    data[y][x] = -data[y][x]
    for a, b in [[0, 1], [1, 0], [0, -1], [-1, 0]]:
        find_path(data, y+a, x+b, risk)

    data[y][x] = -data[y][x]


find_path(data)
print(min_risk - data[0][0])
