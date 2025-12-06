def split_list(l, i):
    ones = []
    zeros = []
    for num in l:
        if num[i] == '1':
            ones.append(num)
        else:
            zeros.append(num)
    return [ones, zeros] if len(ones) >= len(zeros) else [zeros, ones]


l1 = []
l2 = []
with open('d3\data.txt', 'r') as f:
    for line in f:
        if line[0] == '1':
            l1.append(line)
        else:
            l2.append(line)

if len(l1) < len(l2):
    l1, l2 = l2, l1

i = 1
while len(l1) != 1:
    l1 = split_list(l1, i)[0]
    i += 1

i = 1
while len(l2) != 1:
    l2 = split_list(l2, i)[1]
    i += 1

print(int(l1[0], 2) * int(l2[0], 2))
