data = []
with open('d8\data.txt', 'r') as f:
    for l in f:
        for x in l.split(' | ')[1].split():
            data.append(x)

ans = sum(len(x) in [2, 3, 4, 7] for x in data)

print(data)
print(ans)
