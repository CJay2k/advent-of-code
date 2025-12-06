count = 0
curr = float('inf')

with open('d1\data.txt', 'r') as f:
    for line in f:
        if int(line) > curr:
            count += 1
        curr = int(line)

print(count)
