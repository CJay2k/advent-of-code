count = 0
curr = float('inf')
a = 0
b = 0
c = 0

with open('d1\data.txt', 'r') as f:
    a = int(f.readline())
    b = int(f.readline())
    c = int(f.readline())
    curr = a + b + c
    for line in f:
        a = b
        b = c
        c = int(line)
        if a + b + c > curr:
            count += 1
        curr = a + b + c

print(count)
