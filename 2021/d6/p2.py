data = []
with open('d6\data.txt', 'r') as f:
    data = [int(x) for x in f.read().split(',')]

grouped_data = [0 for _ in range(9)]
for x in data:
    grouped_data[x] += 1

for _ in range(256):
    a = grouped_data.pop(0)
    grouped_data[6] += a
    grouped_data.append(a)


print(grouped_data)
print(sum(grouped_data))
