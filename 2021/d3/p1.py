freq = [0] * 12

with open('d3\data.txt', 'r') as f:
    for line in f:
        for i in range(12):
            if line[i] == '1':
                freq[i] += 1
            else:
                freq[i] -= 1

gamma = "".join('1' if x > 0 else '0' for x in freq)
epsilon = "".join('1' if x < 0 else '0' for x in freq)

print(int(gamma, 2) * int(epsilon, 2))
