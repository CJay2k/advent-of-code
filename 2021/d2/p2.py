horizontal = 0
depth = 0
aim = 0

with open('d2\data.txt') as f:
    for line in f:
        command, x = line.split(' ')
        x = int(x)
        match command:
            case 'up':
                aim -= x
            case 'down':
                aim += x
            case 'forward':
                horizontal += x
                depth += aim * x

print(horizontal * depth)
