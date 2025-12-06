data = []
with open('d9\data.txt', 'r') as f:
    data = [[int(x) for x in l.strip()] for l in f.readlines()]


def calculate_area(y, x, area=0):
    if y < 0 or y > len(data) - 1:
        return area
    if x < 0 or x > len(data[0]) - 1:
        return area
    if data[y][x] == 9:
        return area

    data[y][x] = 9
    for a, b in [[0, 1], [1, 0], [0, -1], [-1, 0]]:
        area = calculate_area(y+a, x+b, area)

    return area + 1


areas = []
for i in range(len(data)):
    for j in range(len(data[0])):
        if ((i < 1 or data[i][j] < data[i-1][j])
            and (i + 1 == len(data) or data[i][j] < data[i+1][j])
            and (j < 1 or data[i][j] < data[i][j-1])
                and (j + 1 == len(data[0]) or data[i][j] < data[i][j+1])):

            areas.append(calculate_area(i, j))

areas.sort()

print(areas[-3] * areas[-2] * areas[-1])
