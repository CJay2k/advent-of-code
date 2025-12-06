horizontal = 0
depth = 0
aim = 0

with open('d2\data.txt') as f:
    for line in f:
        command, x = line.split(' ')
        x = int(x)
        match command:
            case 'up':
                depth -= x
            case 'down':
                depth += x
            case 'forward':
                horizontal += x

print(horizontal * depth)
