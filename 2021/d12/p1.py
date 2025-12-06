data = []
with open('d12\data.txt', 'r') as f:
    data = [[l.split('-')[0], l.strip().split('-')[1]] for l in f.readlines()]

caves = {}
for a, b in data:
    caves.setdefault(a, set())
    caves.setdefault(b, set())
    caves[a].add(b)
    caves[b].add(a)


def find_path(cave: str, count=0, path=[]):
    p = path.copy()

    if cave.islower() and cave in p:
        return count

    if cave == 'end':
        return count + 1

    p.append(cave)
    for c in caves.get(cave, []):
        count = find_path(c, count, path=p)

    return count


count = sum(find_path(x, path=['start']) for x in caves['start'])
print(count)
