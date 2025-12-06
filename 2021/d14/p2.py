from collections import Counter

template = ''
insertions = {}
with open('d14\data.txt', 'r') as f:
    template = f.readline().strip()
    f.readline()
    for l in f:
        key, value = l.strip().split(' -> ')
        insertions[key] = value

occurencies = {}
letters_count = Counter(template)
for i in range(len(template) - 1):
    key = template[i] + template[i+1]
    if key in occurencies:
        occurencies[key] += 1
    else:
        occurencies[key] = 1

for _ in range(40):
    new_occurencies = {}
    for key, value in occurencies.items():
        char = insertions[key]
        new_key = key[0] + char
        if new_key in new_occurencies:
            new_occurencies[new_key] += value
        else:
            new_occurencies[new_key] = value

        new_key2 = char + key[1]
        if new_key2 in new_occurencies:
            new_occurencies[new_key2] += value
        else:
            new_occurencies[new_key2] = value

        if char in letters_count:
            letters_count[char] += value
        else:
            letters_count[char] = value
    occurencies = new_occurencies

most_common = 0
least_common = float('inf')
for x in letters_count:
    most_common = max(most_common, letters_count[x])
    least_common = min(least_common, letters_count[x])

print(letters_count)
print(most_common - least_common)
