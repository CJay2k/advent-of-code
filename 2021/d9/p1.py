data = []
with open('d9\data.txt', 'r') as f:
    data = [l.strip() for l in f.readlines()]

ans = 0
for i in range(len(data)):
    for j in range(len(data[0])):
        if ((i < 1 or data[i][j] < data[i-1][j])
            and (i + 1 == len(data) or data[i][j] < data[i+1][j])
            and (j < 1 or data[i][j] < data[i][j-1])
                and (j + 1 == len(data[0]) or data[i][j] < data[i][j+1])):
            ans += 1 + int(data[i][j])

print(ans)
