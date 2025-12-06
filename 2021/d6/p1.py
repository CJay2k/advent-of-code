data = []

with open('d6\data.txt', 'r') as f:
    data = [int(x) for x in f.read().split(',')]

for _ in range(20):
    for i in range(len(data)):
        if data[i] == 0:
            data[i] = 6
            data.append(8)
        else:
            data[i] -= 1
    print(data)
print(len(data))
