from collections import Counter

template = ''
insertions = {}
with open('d14\data.txt', 'r') as f:
    template = f.readline().strip()
    f.readline()
    for l in f:
        key, value = l.strip().split(' -> ')
        insertions[key] = value

for _ in range(4):
    i = 0
    while i < len(template) - 1:
        s = insertions[template[i] + template[i + 1]]
        template = template[:i+1] + s + template[i+1:]
        i += 2
    print(Counter(template))
occurencies = Counter(template)
most_common = 0
least_common = float('inf')
for x in occurencies:
    most_common = max(most_common, occurencies[x])
    least_common = min(least_common, occurencies[x])

print(most_common - least_common)
