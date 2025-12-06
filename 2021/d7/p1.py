from math import ceil

data = []
with open('d7\data.txt', 'r') as f:
    data = [int(x) for x in f.read().split(',')]

data.sort()
median = data[len(data) // 2]

ans = sum(abs(x - median) for x in data)

print(ans)
