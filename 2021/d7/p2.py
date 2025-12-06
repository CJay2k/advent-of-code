from math import ceil

data = []
with open('d7\data.txt', 'r') as f:
    data = [int(x) for x in f.read().split(',')]

ans = float('+inf')

for i in range(max(data)):
    fuel = 0
    for x in data:
        steps = abs(x - i)
        fuel += steps * (steps + 1) / 2
    ans = min(ans, fuel)

print(int(ans))
